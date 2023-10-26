FROM node:20.8.1-bookworm-slim

RUN npm install -g pnpm

WORKDIR /chessground

ENTRYPOINT pnpm install && pnpm run compile && pnpx http-server -p 8080
