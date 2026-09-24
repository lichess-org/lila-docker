FROM node:24 AS build

WORKDIR /lila
COPY ./lila /lila

RUN corepack enable && pnpm install
RUN ./ui/build --no-install -p

RUN mkdir -p assets/public \
    && mv public/compiled public/css public/hashed public/npm assets/public/ \
    && cp -p LICENSE COPYING.md README.md assets/ \
    && git log -n 1 --pretty=oneline > assets/commit.txt

FROM nginx:alpine

WORKDIR /usr/share/nginx/html

COPY --from=build /lila/assets/ /usr/share/nginx/html
COPY --from=build /lila/public/ /usr/share/nginx/html/public
