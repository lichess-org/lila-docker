FROM sbtscala/scala-sbt:eclipse-temurin-focal-17.0.5_8_1.8.2_3.2.2

WORKDIR /lila

ENTRYPOINT ./lila run
