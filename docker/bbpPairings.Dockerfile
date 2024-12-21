FROM sbtscala/scala-sbt:eclipse-temurin-alpine-21.0.2_13_1.10.4_3.5.2

RUN apk add build-base

WORKDIR /mnt
