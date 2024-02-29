FROM sbtscala/scala-sbt:eclipse-temurin-jammy-21.0.2_13_1.9.9_2.13.13

WORKDIR /lila-search

ENTRYPOINT sbt stage && ./target/universal/stage/bin/lila-search
