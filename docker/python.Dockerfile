FROM eclipse-temurin:21.0.1_12-jdk-alpine

COPY --from=python:3.12.1-alpine3.19 / /

RUN pip install --upgrade pip
RUN pip install berserk pymongo requests termcolor

WORKDIR /lila-db-seed
