use warp::Filter;
use warp::http::StatusCode;
use warp::ws::{Message, WebSocket};
use super::engine::{Player, MoveDirection, Game};
use tokio::sync::{Mutex, mpsc};
use std::convert::Infallible;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use futures::{FutureExt, StreamExt};

pub type AsyncGame = Arc<Mutex<Game>>;

#[derive(Deserialize, Serialize)]
struct PutGameRequest {
    player: Player,
    direction: MoveDirection,
}

async fn wrap_unlocked_game(g: AsyncGame) -> Result<impl warp::Reply, Infallible> {
    let game = g.lock().await;
    let game = game.clone();
    Ok(warp::reply::json(&game))
}

async fn wrap_updated_game(req_body: PutGameRequest, g: AsyncGame) -> Result<impl warp::Reply, Infallible> {
    let mut game = g.lock().await;

    game.player_move(req_body.player, req_body.direction);

    Ok(StatusCode::OK)
}

async fn player_move(msg: Message, g: &mut Game, tx: &mpsc::UnboundedSender<Result<Message, warp::Error>>) {
    let validMove = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };

    if let Err(_disconnected) = tx.send(Ok(Message::text("hey"))) {
        // The tx is disconnected, our `player_disconnected` code should
        // be happening in another task, nothing more to do here
    }

    g.player_move(Player::Black, MoveDirection::Down);
}

async fn player_connected(ws: WebSocket, g: AsyncGame) {
    println!("Hit the web server at least!");
    let (player_ws_tx, mut player_ws_rx) = ws.split();

    let mut game = g.lock().await;

    // Use an unbounded channel to handle buffering and flushing of
    // messages to the websocket...
    // TODO: Figure out what this means...
    let (tx, rx) = mpsc::unbounded_channel();
    tokio::task::spawn(rx.forward(player_ws_tx).map(|result| {
        if let Err(e) = result {
            eprintln!("websocket send error: {}", e);
        }
    }));

    while let Some(result) = player_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error: {}", e);
                break;
            }
        };

        player_move(msg, &mut game, &tx).await;
    }
}


fn with_game(g: AsyncGame) -> impl Filter<Extract = (AsyncGame,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || g.clone())
}

fn get_game(g: AsyncGame) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("game")
        .and(warp::get())
        .and(with_game(g))
        .and_then(wrap_unlocked_game)
}

fn update_game(g: AsyncGame) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("game")
        .and(warp::put())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(with_game(g))
        .and_then(wrap_updated_game)
}

fn game_server(g: AsyncGame) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("connect")
        .and(warp::ws())
        .and(with_game(g))
        .map(| ws: warp::ws::Ws, g| {
            ws.on_upgrade(move |socket| player_connected(socket, g))
        })
}

#[tokio::main]
pub async fn serve() {
    let game = Arc::new(Mutex::new(Game::new()));

    warp::serve(game_server(game.clone()).or(get_game(game.clone())).or(update_game(game.clone())).with(warp::cors().allow_origin("http://localhost:8080").allow_header("Content-Type").allow_methods(vec!["GET", "PUT", "OPTIONS"])))
        .run(([127, 0, 0, 1], 3000))
        .await;
}
