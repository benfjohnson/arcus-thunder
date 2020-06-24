use warp::Filter;
use warp::ws::{Message, WebSocket};
use super::engine::{Player, MoveDirection, Game};
use tokio::sync::{RwLock, mpsc};
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use futures::{FutureExt, StreamExt};
use std::collections::HashMap;
use uuid::Uuid;

type AsyncGame = Arc<RwLock<Game>>;
type Players = Arc<RwLock<HashMap<Uuid, mpsc::UnboundedSender<Result<Message, warp::Error>>>>>;

#[derive(Deserialize, Serialize)]
struct PutGameRequest {
    player: Player,
    direction: MoveDirection,
}

async fn stream_game(id: &Uuid, g: &Game, ps: &Players) {
    let stringified_game_state = serde_json::to_string(g).unwrap_or("[]".to_string());

    let players = ps.read().await;
    let tx = if let Some(tx) = players.get(id) {
        tx
    } else {
        return;
    };

    if let Err(_disconnected) = tx.send(Ok(Message::text(stringified_game_state))) {
        // The tx is disconnected, our `player_disconnected` code should
        // be happening in another task, nothing more to do here
    }
}

async fn player_move(msg: Message, g: &mut Game, players: &Players) {
    let move_request: PutGameRequest = if let Ok(s) = msg.to_str() {
        serde_json::from_str(s).unwrap()
    } else {
        return;
    };

    g.player_move(move_request.player, move_request.direction);

    // For each active player, send a transmission with latest game state
    for (id, _) in players.read().await.iter() {
        stream_game(&id, g, players).await;
    }
}

// TODO: Drop players from players map on disconnect!
async fn player_connected(ws: WebSocket, g: AsyncGame, ps: Players) {
    let (player_ws_tx, mut player_ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of
    // messages to the websocket...
    // TODO: Figure out what this means...
    let (tx, rx) = mpsc::unbounded_channel();
    tokio::task::spawn(rx.forward(player_ws_tx).map(|result| {
        if let Err(e) = result {
            eprintln!("websocket send error: {}", e);
        }
    }));

    let id = Uuid::new_v4();
    ps.write().await.insert(id, tx);

    // Don't want this read to block the below listening, so make sure it's immediately dropped
    {
        let game = g.read().await;
        stream_game(&id, &game, &ps).await;
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


fn with_game(g: AsyncGame) -> impl Filter<Extract = (AsyncGame,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || g.clone())
}

fn with_players(ps: Players) -> impl Filter<Extract = (Players,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || ps.clone())
}

fn game_server(g: AsyncGame, ps: Players) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("connect")
        .and(warp::ws())
        .and(with_game(g))
        .and(with_players(ps))
        .map(| ws: warp::ws::Ws, g, ps| {
            ws.on_upgrade(move |socket| player_connected(socket, g, ps))
        })
}

#[tokio::main]
pub async fn serve() {
    let game = Arc::new(RwLock::new(Game::new()));
    let players = Players::default();

    warp::serve(game_server(game.clone(), players.clone()).with(warp::cors().allow_origin("http://localhost:8080").allow_header("Content-Type").allow_methods(vec!["GET", "PUT", "OPTIONS"])))
        .run(([127, 0, 0, 1], 3000))
        .await;
}
