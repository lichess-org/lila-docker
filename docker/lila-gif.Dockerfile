FROM rust:1.73.0-slim-bookworm

WORKDIR /lila-gif

ENTRYPOINT cargo run -- --bind 0.0.0.0:6175
