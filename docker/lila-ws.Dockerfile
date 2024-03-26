FROM sbt-alpine

RUN apk add --no-cache bash

WORKDIR /lila-ws

ENTRYPOINT sbt run -Dconfig.file=/lila-ws.conf
