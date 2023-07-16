FROM rust:1.71

ENV LILA_ENGINE_LOG=lila_engine=debug,tower_http=debug

WORKDIR /lila-engine

ENTRYPOINT cargo run -- --bind 0.0.0.0:9666 --mongodb mongodb://mongodb
