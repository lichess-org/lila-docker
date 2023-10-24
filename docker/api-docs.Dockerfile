FROM node:20.8.1-bookworm-slim

WORKDIR /api/doc

ENTRYPOINT npm install && npm run serve -- --host=0.0.0.0
