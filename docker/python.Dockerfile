FROM eclipse-temurin:25_36-jre-alpine-3.22

COPY --from=python:3.13.7-alpine3.22 / /

RUN pip install --upgrade pip \
    && pip install \
        berserk \
        pymongo \
        requests \
        termcolor \
        faker

WORKDIR /lila-db-seed
