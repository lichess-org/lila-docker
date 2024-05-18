##################################################################################
FROM node:lts-bookworm AS node

COPY repos/lila /lila
COPY conf/ci.conf /lila/conf/application.conf
ENV COREPACK_ENABLE_DOWNLOAD_PROMPT=0
RUN corepack enable
RUN /lila/ui/build --clean-build --debug

##################################################################################
FROM mongo:7-jammy AS dbbuilder

RUN apt update && apt install -y python3-pip curl && apt clean
RUN pip3 install pymongo requests

ENV JAVA_HOME=/opt/java/openjdk
COPY --from=eclipse-temurin:21-jdk $JAVA_HOME $JAVA_HOME
ENV PATH="${JAVA_HOME}/bin:${PATH}"

COPY repos/lila-db-seed /lila-db-seed
WORKDIR /lila-db-seed

RUN mkdir /seeded
RUN mongod --fork --logpath /var/log/mongodb/mongod.log --dbpath /seeded \
 && ./spamdb/spamdb.py \
        --drop-db \
        --password=password \
        --su-password=password \
        --streamers \
        --coaches \
        --tokens

##################################################################################
FROM sbtscala/scala-sbt:eclipse-temurin-alpine-21.0.2_13_1.10.0_3.4.2 AS lilawsbuilder

COPY repos/lila-ws /lila-ws
WORKDIR /lila-ws
RUN sbt stage

##################################################################################
FROM sbtscala/scala-sbt:eclipse-temurin-alpine-21.0.2_13_1.10.0_3.4.2 AS lilabuilder

COPY --from=node /lila /lila
WORKDIR /lila
RUN ./lila stage

##################################################################################
FROM mongo:7-jammy

RUN apt update && apt install -y curl redis python3-pip && apt clean
RUN pip3 install berserk pytest

COPY --from=dbbuilder /seeded /seeded
COPY --from=lilawsbuilder /lila-ws/target /lila-ws/target
COPY --from=lilabuilder /lila/target /lila/target
COPY --from=lilabuilder /lila/public /lila/public
COPY --from=lilabuilder /lila/conf   /lila/conf
COPY --from=node /lila/public /lila/target/universal/stage/public
COPY --from=thegeeklab/wait-for /usr/local/bin/wait-for /usr/local/bin/wait-for

ENV JAVA_HOME=/opt/java/openjdk
COPY --from=eclipse-temurin:21-jdk $JAVA_HOME $JAVA_HOME
ENV PATH="${JAVA_HOME}/bin:${PATH}"
ENV LANG=C.utf8

WORKDIR /lila
CMD mongod --fork --logpath /var/log/mongodb/mongod.log --dbpath /seeded \
    && redis-server --daemonize yes \
    && wait-for localhost:27017 --timeout=15 \
    && wait-for localhost:6379 --timeout=15 \
    && /lila-ws/target/universal/stage/bin/lila-ws \
    & wait-for localhost:9664 --timeout=15 \
    && JAVA_OPTS="-Xms4g -Xmx4g" ./target/universal/stage/bin/lila -Dconfig.file="/lila/conf/application.conf" -Dlogger.file="/lila/conf/logger.dev.xml"
