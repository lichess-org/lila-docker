FROM sbtscala/scala-sbt:eclipse-temurin-focal-17.0.5_8_1.8.2_3.2.2

WORKDIR /lila-fishnet

ENTRYPOINT sbt run -Dhttp.port=9665
