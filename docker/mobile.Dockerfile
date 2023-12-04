FROM ghcr.io/cirruslabs/flutter:3.16.0

RUN apt-get update
RUN apt install --yes \
    clang \
    cmake \
    libgtk-3-dev \
    ninja-build \
    pkg-config

RUN dart --disable-analytics
RUN flutter precache
RUN sdkmanager \
  "build-tools;30.0.3" \
  "emulator" \
  "ndk;23.1.7779620" \
  "platforms;android-29" \
  "platforms;android-30" \
  "platforms;android-31"
RUN flutter doctor -v

# Pre-install mobile app + Flutter dependencies
RUN git clone --depth 1 https://github.com/lichess-org/mobile.git /opt/mobile && \
    cd /opt/mobile && \
    flutter pub get && \
    dart run build_runner build

WORKDIR /app
