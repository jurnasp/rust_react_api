FROM rust:latest

RUN cargo install cargo-watch

WORKDIR /app

ENV ROCKET_ADDRESS=0.0.0.0

ENV PATH="${PATH}:/usr/local/cargo/bin"

EXPOSE 8000