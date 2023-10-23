FROM sbtscala/scala-sbt:eclipse-temurin-jammy-21_35_1.9.7_2.13.12

WORKDIR /lila-search

ENTRYPOINT sbt stage && ./target/universal/stage/bin/lila-search
