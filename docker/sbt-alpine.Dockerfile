# Use a multi-stage build
FROM eclipse-temurin:21.0.2_13-jdk-alpine as builder

ARG SCALA_VERSION=3.4.0
ARG SBT_VERSION=1.9.9
ENV SCALA_HOME=/usr/share/scala

# Combine RUN commands and remove unnecessary files
RUN apk add --no-cache --virtual=.build-dependencies wget ca-certificates bash curl bc && \
    cd "/tmp" && \
    wget "https://github.com/lampepfl/dotty/releases/download/${SCALA_VERSION}/scala3-${SCALA_VERSION}.tar.gz" && \
    tar xzf "scala3-${SCALA_VERSION}.tar.gz" && \
    mkdir "${SCALA_HOME}" && \
    rm "/tmp/scala3-${SCALA_VERSION}/bin/"*.bat && \
    mv "/tmp/scala3-${SCALA_VERSION}/bin" "/tmp/scala3-${SCALA_VERSION}/lib" "${SCALA_HOME}" && \
    ln -s "${SCALA_HOME}/bin/"* "/usr/bin/" && \
    update-ca-certificates && \
    scala -version && \
    scalac -version && \
    curl -fsL https://github.com/sbt/sbt/releases/download/v$SBT_VERSION/sbt-$SBT_VERSION.tgz | tar xfz - -C /usr/local && \
    $(mv /usr/local/sbt-launcher-packaging-$SBT_VERSION /usr/local/sbt || true) && \
    ln -s /usr/local/sbt/bin/* /usr/local/bin/ && \
    sbt -Dsbt.rootdir=true -batch sbtVersion && \
    apk del .build-dependencies && \
    rm -rf "/tmp/"* && \
    rm -rf /var/cache/apk/*

# Start a new stage for the final image
FROM eclipse-temurin:21.0.2_13-jdk-alpine

COPY --from=builder /usr/share/scala /usr/share/scala
COPY --from=builder /usr/local/sbt /usr/local/sbt
COPY --from=builder /usr/local/bin/sbt /usr/local/bin/sbt

WORKDIR /root