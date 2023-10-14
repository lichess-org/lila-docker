FROM sbtscala/scala-sbt:eclipse-temurin-jammy-21_35_1.9.6_2.13.12

WORKDIR /lila-search

ENTRYPOINT sbt stage && ./target/universal/stage/bin/lila-search
