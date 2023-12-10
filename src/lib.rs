use axum::{routing::get, Router};
use tokio::net::TcpListener;

pub async fn run(listener: TcpListener) {
    let app = Router::new().route("/health_check", get(|| async {}));

    axum::serve(listener, app)
        .await
        .expect("Failed to serve the app");
}
