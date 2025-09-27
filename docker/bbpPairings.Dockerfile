FROM sbtscala/scala-sbt:eclipse-temurin-alpine-25_36_1.11.6_3.7.3

RUN apk add build-base

WORKDIR /mnt
