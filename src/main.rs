use zero2prod::startup::run;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .expect("Failed to bind 8000 port");
    run(listener).await;
}
