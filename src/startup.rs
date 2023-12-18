use axum::{
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;

use crate::routes::{health_check, subscribe};

pub async fn run(listener: TcpListener) {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe));

    axum::serve(listener, app)
        .await
        .expect("Failed to serve the app");
}
