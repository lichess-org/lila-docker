FROM node:20.11.1-alpine3.19

RUN npm install -g pnpm

WORKDIR /chessground

ENTRYPOINT pnpm install && pnpm run compile && pnpx http-server -p 8080
