FROM sbtscala/scala-sbt:eclipse-temurin-jammy-21.0.2_13_1.9.9_3.4.0

ENV CONFIG_FORCE_kamon_influxdb_authentication_token="secret"
ENV CONFIG_FORCE_kamon_influxdb_hostname="influxdb"
ENV REDIS_HOST="redis"

ENV APP_BACKUP_FILE=/backup.json
RUN touch "$APP_BACKUP_FILE"

WORKDIR /lila-fishnet

ENTRYPOINT sbt app/run
