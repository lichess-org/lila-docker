FROM rust:1.76.0-slim-bookworm

RUN apt-get update && \
    apt-get install --yes build-essential libssl-dev pkg-config && \
    apt-get clean

RUN echo "-----BEGIN EC PRIVATE KEY-----"  > /private.pem && \
    echo "MHcCAQEEIApExkq9UzdUxUoenwYOuhfnPfn9+7EoWqLNeeOdFIYNoAoGCCqGSM49" >> /private.pem && \
    echo "AwEHoUQDQgAE/5ZxKABB7RU2P4glIQOSi9MeTSnryxptBZPNHPJi4yChb15z5fcF" >> /private.pem && \
    echo "XyjWiBq+UsQ5urXortD2NVfbudzhC1xFeQ==" >> /private.pem && \
    echo "-----END EC PRIVATE KEY-----" >> /private.pem

ENV PUSH_LOG=trace

WORKDIR /lila-push

ENTRYPOINT cargo run -- --bind 0.0.0.0:9054 --vapid /private.pem --subject mailto:contact@localhost
