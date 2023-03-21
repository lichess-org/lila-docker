FROM sbtscala/scala-sbt:eclipse-temurin-17.0.5_8_1.8.2_2.13.10

WORKDIR /lila-search

ENTRYPOINT sbt clean && sbt stage && target/universal/stage/bin/lila-search
