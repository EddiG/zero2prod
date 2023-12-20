use sqlx::PgPool;
use zero2prod::configuration::get_configutation;
use zero2prod::startup::run;
use zero2prod::telemetry::setup_tracing;

#[tokio::main]
async fn main() {
    // Configure the tracing output
    setup_tracing();

    // Get the configuration from the "configuration.yaml" file
    let configuration = get_configutation().expect("Failed to read configuration");

    // Create the database connection pool
    let connection_pool = PgPool::connect_lazy(&configuration.database.connection_string())
        .expect("Failed to connect to Postgres");

    // Start listening at the configured port
    let address = configuration.application.address();
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .unwrap_or_else(|_| panic!("Failed to start listener at {}", &address));

    // Run the webservice
    run(listener, connection_pool).await;
}
