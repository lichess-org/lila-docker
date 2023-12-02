FROM sbtscala/scala-sbt:eclipse-temurin-jammy-21.0.1_12_1.9.7_3.3.1

WORKDIR /lila-fishnet

ENTRYPOINT sbt app/run
