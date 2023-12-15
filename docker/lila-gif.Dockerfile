FROM rust:1.74.1-slim-bookworm

WORKDIR /lila-gif

ENTRYPOINT cargo run -- --bind 0.0.0.0:6175
