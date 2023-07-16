FROM node:latest

WORKDIR /pgn-viewer

ENTRYPOINT npm install && npm run dev && npm run demo
