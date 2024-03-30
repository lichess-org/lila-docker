##################################################################################
FROM mongo:7-jammy as dbbuilder

RUN apt update && apt install -y git python3-pip curl
RUN pip3 install pymongo requests
RUN curl -L https://github.com/adoptium/temurin21-binaries/releases/download/jdk-21.0.2%2B13/OpenJDK21U-jdk_x64_linux_hotspot_21.0.2_13.tar.gz | tar xzf - && mv jdk-21* /jdk-21
ENV PATH=/jdk-21/bin:$PATH

RUN git clone --depth 1 https://github.com/lichess-org/lila-db-seed /lila-db-seed
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
FROM sbtscala/scala-sbt:eclipse-temurin-jammy-21.0.2_13_1.9.9_3.4.0 as lilabuilder

RUN apt update && apt install -y git

RUN mkdir /lila
WORKDIR /lila
RUN git clone --depth 1 https://github.com/lichess-org/lila.git .
COPY conf/ci.conf ./conf/application.conf
RUN ./lila stage

##################################################################################
FROM mongo:7-jammy

RUN apt update && apt install -y curl redis python3-pip && apt clean
RUN pip3 install berserk pytest

COPY --from=dbbuilder /seeded /seeded
COPY --from=dbbuilder /jdk-21 /jdk-21
COPY --from=lilabuilder /lila/target /lila/target
COPY --from=lilabuilder /lila/public /lila/public
COPY --from=lilabuilder /lila/conf   /lila/conf

ENV JAVA_HOME=/jdk-21
ENV PATH=/jdk-21/bin:$PATH
ENV LANG=C.utf8

LABEL version="0.0.1"

WORKDIR /lila
CMD mongod --fork --logpath /var/log/mongodb/mongod.log --dbpath /seeded \
    && redis-server --daemonize yes \
    && JAVA_OPTS="-Xms4g -Xmx4g" ./target/universal/stage/bin/lila -Dconfig.file="/lila/conf/application.conf" -Dlogger.file="/lila/conf/logger.dev.xml"
