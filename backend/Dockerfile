FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/backend /usr/local/cargo/bin/backend
ENV PATH="${PATH}:/usr/local/cargo/bin"
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD ["backend"]
