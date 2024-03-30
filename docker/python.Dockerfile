FROM eclipse-temurin:22_36-jdk-alpine

COPY --from=python:3.12.2-alpine3.19 / /

RUN pip install --upgrade pip
RUN pip install berserk pymongo requests termcolor

WORKDIR /lila-db-seed
