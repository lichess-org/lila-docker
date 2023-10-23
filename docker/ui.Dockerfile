FROM node:20.8.1-bookworm

RUN git config --global --add safe.directory /lila
RUN git config --global --add safe.directory /chessground
RUN git config --global --add safe.directory /pgn-viewer
RUN npm install -g pnpm
