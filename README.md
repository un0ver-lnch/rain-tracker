# Rain Tracker

Monorepo containing a Rust backend and a React Native frontend built with Expo.

## Backend

The backend is a Rust service built with [axum](https://github.com/tokio-rs/axum) and the [Tokio](https://tokio.rs/) async runtime. It exposes a basic API server.

```bash
cd backend
cargo run
```

The server listens on `localhost:3000` and currently provides:

- `GET /` – simple health check
- `GET /records` – returns an empty list of rain records

## Frontend

The frontend uses Expo and React Native.

```bash
cd frontend
npm install
npm start
```
