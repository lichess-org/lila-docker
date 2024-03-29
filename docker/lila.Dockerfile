FROM sbtscala/scala-sbt:eclipse-temurin-alpine-21.0.2_13_1.9.9_3.4.0

WORKDIR /lila

ENTRYPOINT ./lila run
