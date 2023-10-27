FROM golang:1.21.3-alpine3.18

RUN apk add git make

WORKDIR /opt

RUN git clone --depth 1 https://github.com/thoas/picfit.git
RUN make -C /opt/picfit build

RUN echo '{"port": 3001}' > /opt/config.json

ENTRYPOINT /opt/picfit/bin/picfit -c /opt/config.json
