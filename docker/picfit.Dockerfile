# Build stage
FROM golang:1.22.1-alpine3.19 AS build

RUN apk --no-cache add git make

WORKDIR /opt

RUN git clone --depth 1 https://github.com/thoas/picfit.git
RUN make -C /opt/picfit build

# Runtime stage
FROM alpine:3.19

COPY --from=build /opt/picfit/bin/picfit /picfit
COPY assets/coach.png /uploads/coach.png
COPY assets/streamer.png /uploads/streamer.png

ENTRYPOINT ["/picfit", "-c", "/mnt/config.json"]