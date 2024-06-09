FROM node:22.2.0-bookworm-slim

RUN apt update && apt install -y git && apt clean

RUN git config --global --add safe.directory /chessground
RUN git config --global --add safe.directory /lila
RUN git config --global --add safe.directory /pgn-viewer

ENV COREPACK_ENABLE_DOWNLOAD_PROMPT=0
RUN corepack enable

RUN pnpm config set store-dir /.pnpm-store

WORKDIR /app
