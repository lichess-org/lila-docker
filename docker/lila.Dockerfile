FROM sbt-alpine

WORKDIR /lila

ENTRYPOINT ./lila run
