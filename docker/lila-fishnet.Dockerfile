FROM sbtscala/scala-sbt:eclipse-temurin-jammy-21.0.2_13_1.9.8_3.3.1

ENV CONFIG_FORCE_kamon_influxdb_authentication_token="secret"
ENV CONFIG_FORCE_kamon_influxdb_hostname="influxdb"
ENV REDIS_HOST="redis"

WORKDIR /lila-fishnet

ENTRYPOINT sbt app/run
