FROM eclipse-temurin:24.0.2_12-jre-alpine-3.22

COPY --from=python:3.13.7-alpine3.22 / /

RUN pip install --upgrade pip \
    && pip install \
        berserk \
        pymongo \
        requests \
        termcolor

WORKDIR /lila-db-seed
