FROM sbtscala/scala-sbt:eclipse-temurin-jammy-21.0.2_13_1.9.9_3.4.0

WORKDIR /lila

ENTRYPOINT ./lila run
