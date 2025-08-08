mod models;

use axum::{
    extract::{Path, State},
    routing::{get, put},
    Json, Router,
};
use models::{AppState, RainRecord, RainRecordInput, WearEntry, WearEntryInput};
use sqlx::Row;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;

async fn root() -> &'static str {
    "Rain Tracker backend running"
}

// ---------------- Rain record handlers ----------------

async fn list_records(State(state): State<AppState>) -> Json<Vec<RainRecord>> {
    let recs = sqlx::query_as::<_, RainRecord>("SELECT id, amount_mm FROM rain_records")
        .fetch_all(&state.pool)
        .await
        .unwrap();
    Json(recs)
}

async fn create_record(
    State(state): State<AppState>,
    Json(input): Json<RainRecordInput>,
) -> Json<RainRecord> {
    let row = sqlx::query("INSERT INTO rain_records (amount_mm) VALUES (?) RETURNING id")
        .bind(input.amount_mm)
        .fetch_one(&state.pool)
        .await
        .unwrap();
    let id: i64 = row.get("id");
    Json(input.with_id(id))
}

async fn update_record(
    Path(id): Path<i64>,
    State(state): State<AppState>,
    Json(input): Json<RainRecordInput>,
) -> Json<RainRecord> {
    sqlx::query("UPDATE rain_records SET amount_mm = ? WHERE id = ?")
        .bind(input.amount_mm)
        .bind(id)
        .execute(&state.pool)
        .await
        .unwrap();
    Json(input.with_id(id))
}

// ---------------- Wear entry handlers ----------------

async fn list_wear_entries(State(state): State<AppState>) -> Json<Vec<WearEntry>> {
    let rows = sqlx::query("SELECT id, data FROM wear_entries")
        .fetch_all(&state.pool)
        .await
        .unwrap();

    let mut entries = Vec::new();
    for row in rows {
        let id: i64 = row.get("id");
        let data: String = row.get("data");
        let entry_input: WearEntryInput = serde_json::from_str(&data).unwrap();
        entries.push(entry_input.with_id(id));
    }

    Json(entries)
}

async fn create_wear_entry(
    State(state): State<AppState>,
    Json(input): Json<WearEntryInput>,
) -> Json<WearEntry> {
    let data = serde_json::to_string(&input).unwrap();
    let row = sqlx::query("INSERT INTO wear_entries (data) VALUES (?) RETURNING id")
        .bind(data)
        .fetch_one(&state.pool)
        .await
        .unwrap();
    let id: i64 = row.get("id");
    Json(input.with_id(id))
}

async fn update_wear_entry(
    Path(id): Path<i64>,
    State(state): State<AppState>,
    Json(input): Json<WearEntryInput>,
) -> Json<WearEntry> {
    let data = serde_json::to_string(&input).unwrap();
    sqlx::query("UPDATE wear_entries SET data = ? WHERE id = ?")
        .bind(data)
        .bind(id)
        .execute(&state.pool)
        .await
        .unwrap();
    Json(input.with_id(id))
}

#[tokio::main]
async fn main() {
    // Determine database URL. Default to in-memory SQLite for local usage.
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());

    // Create the connection pool and initialise schema.
    let pool = sqlx::any::AnyPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    // Create tables if they do not exist.
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS rain_records (
            id INTEGER PRIMARY KEY,
            amount_mm REAL NOT NULL
        )",
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS wear_entries (
            id INTEGER PRIMARY KEY,
            data TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await
    .unwrap();

    let app_state = AppState { pool };

    let app = Router::new()
        .route("/", get(root))
        .route("/records", get(list_records).post(create_record))
        .route("/records/:id", put(update_record))
        .route(
            "/wear-entries",
            get(list_wear_entries).post(create_wear_entry),
        )
        .route("/wear-entries/:id", put(update_wear_entry))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}

