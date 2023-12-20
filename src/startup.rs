use axum::{
    body::Body,
    http::{Request, Response},
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use std::time::Duration;
use tokio::net::TcpListener;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::Span;

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
                .make_span_with(|_request: &Request<Body>| {
                    tracing::info_span!("request", request_id = %uuid::Uuid::new_v4())
                })
                .on_request(|request: &Request<Body>, _span: &Span| {
                    tracing::info!("{} {}", request.method(), request.uri().path())
                })
                .on_response(
                    |response: &Response<Body>, latency: Duration, _span: &Span| {
                        tracing::info!("Response generated latency={}ms status={}", latency.as_millis(), response.status())
                    },
                )
                .on_failure(
                    |error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        tracing::error!("Response failed status={}", error)
                    },
                ),
        );

    axum::serve(listener, app)
        .await
        .expect("Failed to serve the app");
}
