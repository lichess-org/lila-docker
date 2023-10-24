FROM node:20.8.1-bookworm-slim

RUN apt-get update
RUN apt-get install -y git

RUN npm install -g pnpm
