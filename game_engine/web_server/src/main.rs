use tokio::net::TcpListener;

use axum::{
    http::{header::CONTENT_TYPE, HeaderValue, Method},
    routing::{get, post},
    serve, Router,
};
use tower_http::cors::CorsLayer;

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

    // Allow the Vite dev server to call this API from the browser
    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:5175".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:5175".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE]);

    // Define the routes, then attach the state and middleware last
    let app = Router::new()
        .route("/games", post(create_game))
        .route("/games/{id}", get(get_game))
        .route("/games/{id}/move", post(make_move))
        .with_state(state)
        .layer(cors);

    // Define the port
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    // Start the server
    println!("Server running at  http://127.0.0.1:3000");
    serve(listener, app).await.unwrap();
}

