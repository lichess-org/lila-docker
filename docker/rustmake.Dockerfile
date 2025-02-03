FROM rust:1.84.0-slim-bookworm

RUN apt update \
    && apt install -y make \
    && apt clean