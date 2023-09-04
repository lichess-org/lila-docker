FROM rust:1.72

WORKDIR /lila-gif

ENTRYPOINT cargo run -- --bind 0.0.0.0:6175
