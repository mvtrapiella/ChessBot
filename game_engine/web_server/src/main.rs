use tokio::net::TcpListener;

use axum::{
    routing::{get, post},
    serve, Router,
};

use crate::handlers::{create_game, get_game, make_move};
use crate::state::AppState;

mod dto;
mod error;
mod handlers;
mod state;

#[tokio::main]
async fn main() {
    // Create the shared application state
    let state = AppState::new();

    // Define the routes, then attach the state last
    let app = Router::new()
        .route("/games", post(create_game))
        .route("/games/{id}", get(get_game))
        .route("/games/{id}/move", post(make_move))
        .with_state(state);

    // Define the port
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    // Start the server
    println!("Server running at  http://127.0.0.1:3000");
    serve(listener, app).await.unwrap();
}

