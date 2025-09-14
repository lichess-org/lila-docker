FROM sbtscala/scala-sbt:eclipse-temurin-alpine-24.0.1_9_1.11.6_3.7.3

RUN apk add build-base

WORKDIR /mnt
