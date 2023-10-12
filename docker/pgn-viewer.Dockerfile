FROM node:20.8.0-bookworm

RUN npm install -g pnpm

WORKDIR /pgn-viewer

ENTRYPOINT pnpm install \
    && pnpm run demo
