FROM rust:1.76.0-slim-bookworm

WORKDIR /lila-gif

ENTRYPOINT cargo run -- --bind 0.0.0.0:6175
