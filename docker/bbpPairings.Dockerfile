FROM sbtscala/scala-sbt:eclipse-temurin-alpine-21.0.5_11_1.10.7_3.6.2

RUN apk add build-base

WORKDIR /mnt
