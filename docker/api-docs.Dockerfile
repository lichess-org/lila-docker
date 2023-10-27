FROM node:20.9.0-alpine3.18

WORKDIR /api/doc

ENTRYPOINT npm install && npm run serve -- --host=0.0.0.0
