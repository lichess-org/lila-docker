FROM node:20.9.0-alpine3.18

RUN npm install -g pnpm

WORKDIR /chessground

ENTRYPOINT pnpm install && pnpm run compile && pnpx http-server -p 8080
