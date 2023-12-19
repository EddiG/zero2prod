use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::{
    routes::{health_check, subscribe},
    state::AppState,
};

pub async fn run(listener: TcpListener, db_pool: PgPool) {
    let state = AppState { db_pool };
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    DefaultMakeSpan::new()
                        .include_headers(false)
                        .level(Level::INFO),
                )
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        );

    axum::serve(listener, app)
        .await
        .expect("Failed to serve the app");
}
