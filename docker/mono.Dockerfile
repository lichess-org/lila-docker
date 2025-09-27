##################################################################################
FROM node:24-trixie AS node

COPY repos/lila /lila
COPY conf/mono.conf /lila/conf/mono.conf
ENV COREPACK_ENABLE_DOWNLOAD_PROMPT=0
ENV COREPACK_INTEGRITY_KEYS=0
RUN corepack enable \
    && /lila/ui/build --clean --debug

##################################################################################
FROM mongo:7-jammy AS dbbuilder

RUN apt update \
    && apt install -y \
        curl \
        python3-pip \
        python3-venv \
    && apt clean

ENV JAVA_HOME=/opt/java/openjdk
COPY --from=eclipse-temurin:25-jdk $JAVA_HOME $JAVA_HOME
ENV PATH="${JAVA_HOME}/bin:${PATH}"

COPY repos/lila /lila
COPY repos/lila-db-seed /lila-db-seed
WORKDIR /lila-db-seed

RUN mkdir /seeded \
    && mongod --fork --logpath /var/log/mongodb/mongod.log --dbpath /seeded \
    && ./spamdb/spamdb.py \
        --drop-db \
        --password=password \
        --su-password=password \
        --streamers \
        --coaches \
        --tokens \
    && mongosh --quiet lichess /lila/bin/mongodb/indexes.js

##################################################################################
FROM sbtscala/scala-sbt:eclipse-temurin-alpine-25_36_1.11.6_3.7.3 AS lilawsbuilder

COPY repos/lila-ws /lila-ws
WORKDIR /lila-ws
RUN sbt stage

##################################################################################
FROM sbtscala/scala-sbt:eclipse-temurin-alpine-25_36_1.11.6_3.7.3 AS lilabuilder

COPY --from=node /lila /lila
WORKDIR /lila
RUN ./lila.sh stage

##################################################################################
FROM mongo:7-jammy

RUN apt update \
    && apt install -y debian-keyring debian-archive-keyring apt-transport-https curl \
    && curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/gpg.key' | gpg --dearmor -o /usr/share/keyrings/caddy-stable-archive-keyring.gpg \
    && curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/debian.deb.txt' | tee /etc/apt/sources.list.d/caddy-stable.list \
    && apt update \
    && apt install -y \
        caddy \
        curl \
        python3-pip \
        redis \
        supervisor \
    && apt clean \
    && pip3 install berserk pytest \
    && mkdir -p /var/log/supervisor

COPY --from=dbbuilder /seeded /seeded
COPY --from=lilawsbuilder /lila-ws/target /lila-ws/target
COPY --from=lilabuilder /lila/target /lila/target
COPY --from=lilabuilder /lila/public /lila/public
COPY --from=lilabuilder /lila/conf   /lila/conf
COPY --from=node /lila/public /lila/target/universal/stage/public

COPY conf/supervisord.conf /etc/supervisor/conf.d/supervisord.conf
COPY conf/mono.Caddyfile /mono.Caddyfile
COPY static /static

ENV JAVA_HOME=/opt/java/openjdk
ENV JAVA_OPTS="-Xms4g -Xmx4g"
ENV PATH="${JAVA_HOME}/bin:${PATH}"
ENV LANG=C.utf8
COPY --from=eclipse-temurin:25-jdk $JAVA_HOME $JAVA_HOME

ENV LILA_DOMAIN=localhost:8080
ENV LILA_URL=http://localhost:8080

CMD ["supervisord", "-c", "/etc/supervisor/supervisord.conf"]
