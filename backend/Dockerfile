FROM rust:latest

RUN cargo install cargo-watch

WORKDIR /app

ENV PATH="${PATH}:/usr/local/cargo/bin"

EXPOSE 8000