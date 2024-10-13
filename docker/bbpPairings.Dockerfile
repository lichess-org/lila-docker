FROM ubuntu:focal

RUN apt-get update && \
    apt-get install --yes \
        g++ \
        git \
        make \
    && apt-get clean

WORKDIR /mnt
