FROM sbtscala/scala-sbt:eclipse-temurin-jammy-21.0.2_13_1.9.9_3.4.0

# Install mongodb
# https://www.mongodb.com/docs/manual/tutorial/install-mongodb-on-ubuntu/
RUN apt-get update \
    && apt-get install gnupg curl -y \
    && curl -fsSL https://www.mongodb.org/static/pgp/server-7.0.asc | \
        gpg -o /usr/share/keyrings/mongodb-server-7.0.gpg \
        --dearmor \
    && echo "deb [ arch=amd64,arm64 signed-by=/usr/share/keyrings/mongodb-server-7.0.gpg ] https://repo.mongodb.org/apt/ubuntu jammy/mongodb-org/7.0 multiverse" | tee /etc/apt/sources.list.d/mongodb-org-7.0.list \
    && apt-get update \
    && apt-get install -y mongodb-org \
    && apt-get clean \
    && mkdir -p /data/db

# Install redis
# https://redis.io/docs/install/install-redis/install-redis-on-linux/
RUN apt install lsb-release curl gpg -y \
    && curl -fsSL https://packages.redis.io/gpg | gpg --dearmor -o /usr/share/keyrings/redis-archive-keyring.gpg \
    && echo "deb [signed-by=/usr/share/keyrings/redis-archive-keyring.gpg] https://packages.redis.io/deb $(lsb_release -cs) main" | tee /etc/apt/sources.list.d/redis.list \
    && apt-get update \
    && apt-get install redis -y \
    && apt-get clean

RUN git clone --depth 1 https://github.com/lichess-org/lila-db-seed /lila-db-seed
WORKDIR /lila-db-seed
RUN apt install python3-pip -y \
    && pip3 install pymongo requests
RUN mongod --fork --logpath /var/log/mongod.log \
    && ./spamdb/spamdb.py \
        --drop-db \
        --password=password \
        --su-password=password \
        --streamers \
        --coaches \
        --tokens

## Pre-install dependencies for Berserk client
RUN pip3 install berserk pytest

RUN git clone --depth 1 https://github.com/lichess-org/lila.git /lila
WORKDIR /lila

COPY conf/ci.conf /lila/conf/application.conf
RUN ./lila stage

CMD mongod --fork --logpath /var/log/mongod.log \
    && redis-server --daemonize yes \
    && JAVA_OPTS="-Xms4g -Xmx4g" ./target/universal/stage/bin/lila -Dconfig.file="/lila/conf/application.conf" -Dlogger.file="/lila/conf/logger.dev.xml"
