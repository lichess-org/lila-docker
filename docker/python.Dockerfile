FROM openjdk:slim

COPY --from=python:3.11.0 / /

WORKDIR /lila-db-seed
