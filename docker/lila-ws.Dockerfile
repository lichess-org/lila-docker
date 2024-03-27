FROM sbt-alpine

WORKDIR /lila-ws

ENTRYPOINT sbt run -Dconfig.file=/lila-ws.conf
