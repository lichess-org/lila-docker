FROM sbt-alpine

RUN apk add --no-cache bash

WORKDIR /lila

ENTRYPOINT ./lila run
