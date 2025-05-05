FROM sbtscala/scala-sbt:eclipse-temurin-alpine-21.0.7_6_1.10.11_3.6.4

RUN apk add build-base

WORKDIR /mnt
