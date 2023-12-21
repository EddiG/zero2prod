FROM rust:1.74.1 AS builder
RUN cargo new --quiet --vcs none --bin app
WORKDIR app

COPY .cargo .cargo
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN cargo build --release

COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

FROM debian:12-slim AS runtime
RUN addgroup --system app && adduser --system --ingroup app app
USER app
WORKDIR app
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]
