use tokio::net::TcpListener;

use axum::{serve, Router};

use crate::state::AppState;

mod error;
mod dto;   
mod state;

#[tokio::main]
async fn main() {
    // Create and define the routes
    let state = AppState::new();
    let app = Router::new().with_state(state);

    // Define the port
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    // Start the server
    println!("Server running at  http://127.0.0.1:3000");
    serve(listener, app).await.unwrap();
}
