# match node version from https://github.com/lichess-org/lila/blob/master/.node-version
FROM node:24.11.1-trixie-slim

USER root
ENV COREPACK_ENABLE_DOWNLOAD_PROMPT=0
ENV COREPACK_INTEGRITY_KEYS=0

RUN apt update \
    && apt install -y git \
    && apt clean \
    && git config --global --add safe.directory /chessground \
    && git config --global --add safe.directory /lila \
    && git config --global --add safe.directory /pgn-viewer \
    && corepack enable \
    && pnpm config set store-dir /.pnpm-store \
    # needed for ui, chessground images
    && mkdir -p /.cache && chmod -R 777 /.cache \
    # needed for api_docs
    && mkdir -p /.npm && chmod -R 777 /.npm

ARG USER_ID
ARG GROUP_ID

RUN if [ "$USER_ID" != "1000" ] && [ "$USER_ID" != "0" ]; then \
        adduser -D -u $USER_ID -G $(id -gn node) -h $(eval echo ~node) newuser; \
        chown -R $USER_ID:$(id -gn node) $(eval echo ~node); \
    fi

WORKDIR /app
