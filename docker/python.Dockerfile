FROM eclipse-temurin:21.0.1_12-jdk-alpine

COPY --from=python:3.12.0-alpine3.18 / /

RUN pip install pymongo

WORKDIR /lila-db-seed
