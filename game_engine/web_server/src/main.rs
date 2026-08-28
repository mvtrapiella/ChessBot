use tokio::net::TcpListener;

use axum::{
    http::{header::CONTENT_TYPE, HeaderValue, Method},
    routing::{get, post},
    serve, Router,
};
use tower_http::cors::CorsLayer;

use crate::handlers::{create_game, get_game, legal_moves, make_move, undo_move};
use crate::state::AppState;

mod dto;
mod error;
mod handlers;
mod notation;
mod state;

#[tokio::main]
async fn main() {
    // Create the shared application state
    let state = AppState::new();

    // Only needed for local development (plain `npm run dev`/`cargo run` on separate
    // ports). In production, nginx reverse-proxies /api/* to this service, so browser
    // requests are same-origin and never hit this CORS layer at all.
    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:5173".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:5173".parse::<HeaderValue>().unwrap(),
            "http://localhost:5175".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:5175".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE]);

    // Define the routes, then attach the state and middleware last
    let app = Router::new()
        .route("/games", post(create_game))
        .route("/games/{id}", get(get_game))
        .route("/games/{id}/legal-moves/{square}", get(legal_moves))
        .route("/games/{id}/move", post(make_move))
        .route("/games/{id}/undo", post(undo_move))
        .with_state(state)
        .layer(cors);

    // 0.0.0.0 so the server is reachable from outside its container, not just localhost
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // Start the server
    println!("Server running at  http://0.0.0.0:3000");
    serve(listener, app).await.unwrap();
}

