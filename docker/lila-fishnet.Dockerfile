FROM sbtscala/scala-sbt:eclipse-temurin-focal-17.0.8.1_1_1.9.7_3.3.1

WORKDIR /lila-fishnet

ENTRYPOINT sbt stage && \
    ./target/universal/stage/bin/lila-fishnet \
        -Dhttp.port=9665 \
        -Dpidfile.path=/dev/null \
        -Dredis.uri="redis://redis"
