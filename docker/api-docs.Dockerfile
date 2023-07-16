FROM node:latest

WORKDIR /api/doc

ENTRYPOINT npm run serve -- --host=0.0.0.0 --watch
