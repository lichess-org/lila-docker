FROM node:20.8.1-bookworm-slim

RUN apt-get update
RUN apt-get install -y git

RUN git config --global --add safe.directory /chessground
RUN git config --global --add safe.directory /lila
RUN git config --global --add safe.directory /pgn-viewer

RUN npm install -g pnpm
