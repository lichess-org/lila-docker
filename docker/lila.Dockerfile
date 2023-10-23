FROM sbtscala/scala-sbt:eclipse-temurin-jammy-21_35_1.9.7_3.3.1

WORKDIR /lila

ENTRYPOINT ./lila run
