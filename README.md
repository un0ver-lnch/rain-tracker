# Rain Tracker

Monorepo containing a Rust backend and a React Native frontend built with Expo.

## Backend

The backend is a Rust service built with [axum](https://github.com/tokio-rs/axum) and the [Tokio](https://tokio.rs/) async runtime. Data is stored using [`sqlx`](https://github.com/launchbadge/sqlx). By default the service runs with an in-memory SQLite database which requires no configuration. For deployments we recommend pointing `DATABASE_URL` at a persistent PostgreSQL instance.

```bash
cd backend
cargo run
```

The server listens on `localhost:3000` and provides endpoints for listing, creating and updating data:

- `GET /` – simple health check
- `GET /records` – list rain records
- `POST /records` – create a record (`{ "amount_mm": 2.5 }`)
- `PUT /records/:id` – update a record
- `GET /wear-entries` – list clothing entries
- `POST /wear-entries` – create a clothing entry
- `PUT /wear-entries/:id` – update a clothing entry

## Frontend

The frontend uses Expo and React Native.

```bash
cd frontend
npm install
npm start
```
