FROM node:latest

WORKDIR /pgn-viewer

ENTRYPOINT npm install \
    && npm run dev \
    && npx sass --no-source-map --update --style=expanded scss:demo \
    && npm run demo
