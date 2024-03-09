FROM golang:1.22.1-alpine3.19

RUN apk --no-cache add git make

COPY assets/coach.png /uploads/coach.png
COPY assets/streamer.png /uploads/streamer.png

WORKDIR /opt

RUN git clone --depth 1 https://github.com/thoas/picfit.git
RUN make -C /opt/picfit build

ENTRYPOINT /opt/picfit/bin/picfit -c /mnt/config.json
