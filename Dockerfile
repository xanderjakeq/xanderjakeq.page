FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR /app 
RUN apt update && apt install lld clang -y
RUN cargo install sccache --locked

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release --bin wips_server

FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get install libc6 \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/wips_server wips_server
COPY configuration configuration

ENV APP_ENVIRONMENT=production
ENTRYPOINT ["./wips_server"]
