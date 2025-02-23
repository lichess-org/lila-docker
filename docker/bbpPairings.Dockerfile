FROM sbtscala/scala-sbt:eclipse-temurin-alpine-21.0.6_7_1.10.7_3.6.3

RUN apk add build-base

WORKDIR /mnt
