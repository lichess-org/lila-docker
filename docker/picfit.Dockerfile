FROM golang:1.21.4-alpine3.18

RUN apk add git make

WORKDIR /opt

RUN git clone --depth 1 https://github.com/thoas/picfit.git
RUN make -C /opt/picfit build

ENTRYPOINT /opt/picfit/bin/picfit -c /mnt/config.json
