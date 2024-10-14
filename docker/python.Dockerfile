FROM eclipse-temurin:22.0.2_9-jdk-alpine

COPY --from=python:3.12.6-alpine3.20 / /

RUN pip install --upgrade pip \
    && pip install \
        berserk \
        pymongo \
        requests \
        termcolor

WORKDIR /lila-db-seed
