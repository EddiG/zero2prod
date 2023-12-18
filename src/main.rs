use zero2prod::configuration::get_configutation;
use zero2prod::startup::run;

#[tokio::main]
async fn main() {
    let configuration = get_configutation().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .unwrap_or_else(|_| panic!("Failed to start listener at {}", &address));
    run(listener).await;
}
