FROM ghcr.io/cirruslabs/flutter:3.22.0

RUN apt-get update && \
    apt-get install --yes clang cmake libgtk-3-dev ninja-build pkg-config && \
    apt-get clean

RUN dart --disable-analytics
RUN flutter precache
RUN sdkmanager \
  "build-tools;34.0.0" \
  "emulator" \
  "ndk;26.2.11394342"
RUN flutter doctor -v

WORKDIR /app
