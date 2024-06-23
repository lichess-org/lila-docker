FROM node:22.3.0-bookworm-slim

USER root

RUN apt update && apt install -y git && apt clean

RUN git config --global --add safe.directory /chessground
RUN git config --global --add safe.directory /lila
RUN git config --global --add safe.directory /pgn-viewer

ENV COREPACK_ENABLE_DOWNLOAD_PROMPT=0
RUN corepack enable

RUN pnpm config set store-dir /.pnpm-store

# needed for ui, chessground images
RUN mkdir -p /.cache && chmod -R 777 /.cache
# needed for api_docs
RUN mkdir -p /.npm && chmod -R 777 /.npm

ARG USER_ID
ARG GROUP_ID

RUN if [ "$USER_ID" != "1000" ] && [ "$USER_ID" != "0" ]; then \
        adduser -D -u $USER_ID -G $(id -gn node) -h $(eval echo ~node) newuser; \
        chown -R $USER_ID:$(id -gn node) $(eval echo ~node); \
    fi

WORKDIR /app
