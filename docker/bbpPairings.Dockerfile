FROM ubuntu:focal

RUN apt-get update
RUN apt-get install --yes git make g++
