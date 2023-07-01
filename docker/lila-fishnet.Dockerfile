FROM sbtscala/scala-sbt:eclipse-temurin-focal-17.0.5_8_1.9.1_3.3.0

WORKDIR /lila-fishnet

ENTRYPOINT sbt run -Dhttp.port=9665
