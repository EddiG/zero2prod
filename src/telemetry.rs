use tracing_subscriber::{filter::LevelFilter, EnvFilter};

pub fn setup_tracing() {
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
}
