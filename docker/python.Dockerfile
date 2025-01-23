FROM eclipse-temurin:21.0.5_11-jdk-alpine-3.21

COPY --from=python:3.13.1-alpine3.21 / /

RUN pip install --upgrade pip \
    && pip install \
        berserk \
        pymongo \
        requests \
        termcolor

WORKDIR /lila-db-seed
