FROM rust:1.71

WORKDIR /lila-gif

ENTRYPOINT cargo run -- --bind 0.0.0.0:6175
