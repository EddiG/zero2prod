use sqlx::PgPool;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::EnvFilter;
use zero2prod::configuration::get_configutation;
use zero2prod::startup::run;

#[tokio::main]
async fn main() {
    // Configure the logs output (stdout)
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .compact()
        .init();

    // Get the configuration from the "configuration.yaml" file
    let configuration = get_configutation().expect("Failed to read configuration");

    // Create the database connection pool
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    // Start listening at the configured port
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .unwrap_or_else(|_| panic!("Failed to start listener at {}", &address));

    // Run the webservice
    run(listener, connection_pool).await;
}
