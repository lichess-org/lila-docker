FROM sbtscala/scala-sbt:eclipse-temurin-jammy-21.0.2_13_1.9.9_3.3.1

RUN git clone --depth 1 https://github.com/lichess-org/lila.git /lila

COPY conf/ci.conf /lila/conf/application.conf

WORKDIR /lila
RUN ./lila stage

ENTRYPOINT sleep 10 && JAVA_OPTS="-Xms4g -Xmx4g" ./target/universal/stage/bin/lila -Dconfig.file="/lila/conf/application.conf" -Dlogger.file="/lila/conf/logger.dev.xml"
