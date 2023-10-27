FROM node:20.9.0-alpine3.18

RUN npm install -g pnpm

WORKDIR /pgn-viewer

ENTRYPOINT pnpm install \
    && pnpm run demo
