FROM sbtscala/scala-sbt:eclipse-temurin-25.0.3_9_1.12.13_3.8.4

RUN apk add build-base

WORKDIR /mnt
