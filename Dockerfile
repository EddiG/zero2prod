FROM rust:1.74.1 AS chef
RUN cargo install cargo-chef

FROM chef AS planner
WORKDIR app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR app
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

# We do not need the Rust toolchain to run the binary!
FROM debian:12-slim AS runtime
WORKDIR app
RUN addgroup --system app && adduser --system --ingroup app app
USER app
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]
