FROM rust:1.70.0 AS builder
WORKDIR /app/url_shortener
COPY ./src /app/url_shortener/src
COPY ./migration /app/url_shortener/migration
COPY ./entity /app/url_shortener/entity
COPY ./Cargo.lock /app/url_shortener
COPY ./Cargo.toml /app/url_shortener
RUN cargo build --release

FROM debian:bullseye-slim AS app
COPY --from=builder /app/url_shortener/target/release/url_shortener /usr/local/bin/url_shortener
EXPOSE 8080
CMD ["url_shortener"]