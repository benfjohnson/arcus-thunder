use warp::Filter;
use warp::http::StatusCode;
use super::engine;
use tokio::sync::Mutex;
use std::convert::Infallible;
use std::sync::Arc;

pub type AsyncGame = Arc<Mutex<engine::Game>>;

async fn wrap_unlocked_game(g: AsyncGame) -> Result<impl warp::Reply, Infallible> {
    let game = g.lock().await;
    let game = game.clone();
    Ok(warp::reply::json(&game))
}

async fn wrap_updated_game(g: AsyncGame) -> Result<impl warp::Reply, Infallible> {
    let mut game = g.lock().await;

    game.player_move(engine::Player::Black, engine::MoveDirection::Down);

    Ok(StatusCode::OK)
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
        .and(with_game(g))
        .and_then(wrap_updated_game)
}

#[tokio::main]
pub async fn serve() {
    let game = Arc::new(Mutex::new(engine::Game::new()));

    warp::serve(get_game(game.clone()).or(update_game(game.clone())).with(warp::cors().allow_origin("http://localhost:8000").allow_methods(vec!["GET", "PUT"])))
        .run(([127, 0, 0, 1], 3000))
        .await;
}
