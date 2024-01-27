FROM sbtscala/scala-sbt:eclipse-temurin-jammy-21.0.2_13_1.9.8_3.3.1

WORKDIR /lila

ENTRYPOINT ./lila run
