use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tokio::net::TcpListener;

use crate::{
    routes::{health_check, subscribe},
    state::AppState,
};

pub async fn run(listener: TcpListener, db_pool: PgPool) {
    let state = AppState { db_pool };
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(state);

    axum::serve(listener, app)
        .await
        .expect("Failed to serve the app");
}
