FROM rust:latest

RUN cargo install cargo-watch && \
    cargo install sqlx-cli --no-default-features --features native-tls,postgres

WORKDIR /app

ENV PATH="${PATH}:/usr/local/cargo/bin"

EXPOSE 8000