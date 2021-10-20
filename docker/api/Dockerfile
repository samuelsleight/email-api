FROM lukemathwalker/cargo-chef:0.1.31-rust-1.55 AS chef
WORKDIR api-server
RUN RUSTUP_DIST_SERVER=https://dev-static.rust-lang.org rustup default 1.56

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /api-server/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM debian:buster-slim AS runtime
WORKDIR api-server
RUN apt-get update
RUN apt-get install -y openssl ca-certificates
COPY --from=builder /api-server/target/release/email-api-server /usr/local/bin
COPY .env Rocket.toml ./
ENTRYPOINT ["/usr/local/bin/email-api-server"]