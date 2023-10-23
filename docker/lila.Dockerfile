FROM sbtscala/scala-sbt:eclipse-temurin-jammy-21_35_1.9.6_3.3.1

WORKDIR /lila

ENTRYPOINT ./lila run
