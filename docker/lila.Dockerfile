FROM sbtscala/scala-sbt:eclipse-temurin-focal-17.0.8.1_1_1.9.6_3.3.1

WORKDIR /lila

ENTRYPOINT ./lila run
