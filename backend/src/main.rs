mod models;

use axum::{extract::State, routing::get, Json, Router};
use models::{AppState, RainRecord, WearEntry};
use std::net::SocketAddr;
use tokio::net::TcpListener;

async fn root() -> &'static str {
    "Rain Tracker backend running"
}

async fn list_records(State(state): State<AppState>) -> Json<Vec<RainRecord>> {
    Json(state.records.clone())
}

async fn list_wear_entries(State(state): State<AppState>) -> Json<Vec<WearEntry>> {
    Json(state.wear_entries.clone())
}

#[tokio::main]
async fn main() {
    let app_state = AppState::default();

    let app = Router::new()
        .route("/", get(root))
        .route("/records", get(list_records))
        .route("/wear-entries", get(list_wear_entries))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}

