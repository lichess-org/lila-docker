FROM node:20.11.1-bookworm

RUN git config --global --add safe.directory /chessground
RUN git config --global --add safe.directory /lila
RUN git config --global --add safe.directory /pgn-viewer

RUN npm install -g pnpm

RUN pnpm config set store-dir /.pnpm-store
