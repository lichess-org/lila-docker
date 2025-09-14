# Build stage
FROM golang:1.25.1-alpine3.22 AS build

RUN apk --no-cache add g++ gcc make \
    && wget https://github.com/thoas/picfit/archive/refs/tags/0.15.1.zip \
    && unzip 0.15.1.zip \
    && mv picfit-0.15.1 /picfit

WORKDIR /picfit

RUN make build

# Runtime stage
FROM alpine:3.22.1

COPY --from=build /picfit/bin/picfit /picfit

COPY assets/coach.png /uploads/coach.png
COPY assets/streamer.png /uploads/streamer.png

ENTRYPOINT ["/picfit", "-c", "/mnt/config.json"]
