FROM ubuntu:focal

RUN apt-get update && \
    apt-get install --yes git make g++ && \
    apt-get clean

WORKDIR /mnt
