use super::engine::{Game, MoveDirection};
use futures::{FutureExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;
use warp::http::{header, Response, StatusCode};
use warp::ws::{Message, WebSocket};
use warp::Filter;

type AsyncGame = Arc<RwLock<Game>>;
type PlayerConnections =
    Arc<RwLock<HashMap<Uuid, mpsc::UnboundedSender<Result<Message, warp::Error>>>>>;

#[derive(Deserialize, Serialize)]
struct PutGameRequest {
    player_id: Uuid,
    direction: MoveDirection,
}

// transmit lastest game state to all players
async fn stream_game(g: &Game, ps: &PlayerConnections) {
    let stringified_game_state = serde_json::to_string(g).unwrap_or("[]".to_string());

    let players = ps.read().await;

    for (_, tx) in players.iter() {
        if let Err(_disconnected) = tx.send(Ok(Message::text(stringified_game_state.clone()))) {
            // the tx is disconnected, our `player_disconnected` code should
            // be happening in another task, nothing more to do here
        }
    }
}

async fn player_move(msg: Message, g: &mut Game, players: &PlayerConnections) {
    let move_request: PutGameRequest = if let Ok(s) = msg.to_str() {
        serde_json::from_str(s).unwrap()
    } else {
        return;
    };

    g.player_move(move_request.player_id, move_request.direction);

    // for each active player, send a transmission with latest game state
    stream_game(g, players).await;
}

// TODO: Drop players from players map on disconnect!
async fn player_connected(ws: WebSocket, g: AsyncGame, at_id: String, ps: PlayerConnections) {
    let (player_ws_tx, mut player_ws_rx) = ws.split();

    // use an unbounded channel to handle buffering and flushing of
    // messages to the websocket...
    // TODO: Figure out what this means...
    let (tx, rx) = mpsc::unbounded_channel();
    tokio::task::spawn(rx.forward(player_ws_tx).map(|result| {
        if let Err(e) = result {
            eprintln!("websocket send error: {}", e);
        }
    }));

    // don't want this write to block the below listening, so make sure it's immediately dropped
    {
        let new_id = Uuid::parse_str(&at_id);

        if let Err(_) = new_id {
            return;
        }

        let mut game = g.write().await;
        let new_player_id = game.add_player(new_id.unwrap());
        ps.write().await.insert(new_player_id, tx);

        stream_game(&game, &ps).await;
    }

    while let Some(result) = player_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error: {}", e);
                break;
            }
        };

        let mut game = g.write().await;
        player_move(msg, &mut game, &ps).await;
    }
}

fn with_game(
    g: AsyncGame,
) -> impl Filter<Extract = (AsyncGame,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || g.clone())
}

fn with_players(
    ps: PlayerConnections,
) -> impl Filter<Extract = (PlayerConnections,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || ps.clone())
}

fn game_server(
    g: AsyncGame,
    ps: PlayerConnections,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("connect")
        .and(warp::ws())
        .and(with_game(g))
        .and(warp::cookie("atid"))
        .and(with_players(ps))
        .map(|ws: warp::ws::Ws, g, at_id, ps| {
            ws.on_upgrade(move |socket| player_connected(socket, g, at_id, ps))
        })
}

fn authenticate() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("auth").and(warp::cookie::optional("atid")).map(
        |cookie: Option<String>| match cookie {
            Some(_) => Response::builder().status(StatusCode::OK).body("{}"),
            None => {
                let new_id = Uuid::new_v4();
                let new_cookie = format!("atid={}; Path=/;", new_id);

                Response::builder()
                    .status(StatusCode::CREATED)
                    .header(header::SET_COOKIE, new_cookie)
                    .body("{}")
            }
        },
    )
}

#[tokio::main]
pub async fn serve() {
    let game = Arc::new(RwLock::new(Game::new()));
    let players = PlayerConnections::default();

    warp::serve(
        game_server(game.clone(), players.clone())
            .or(authenticate())
            .with(
                warp::cors()
                    .allow_credentials(true)
                    .allow_origin("http://localhost:8080")
                    .allow_header("Content-Type")
                    .allow_methods(vec!["GET", "PUT", "OPTIONS"]),
            ),
    )
    .run(([127, 0, 0, 1], 3000))
    .await;
}
