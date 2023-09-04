FROM golang:1.21-bookworm

WORKDIR /opt

RUN echo '{"port": 3001}' > /opt/config.json

RUN git clone --depth 1 https://github.com/thoas/picfit.git \
    && cd picfit \
    && make build

EXPOSE 3001

ENTRYPOINT /opt/picfit/bin/picfit -c /opt/config.json
