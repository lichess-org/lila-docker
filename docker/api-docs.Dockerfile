FROM node:20.8.0-bookworm

WORKDIR /api/doc

ENTRYPOINT npm install && npm run serve -- --host=0.0.0.0
