# Build stage
FROM golang:1.23.1-alpine3.20 AS build

RUN apk --no-cache add make

COPY /picfit /picfit
WORKDIR /picfit

RUN make -C /picfit build

# Runtime stage
FROM alpine:3.19

COPY --from=build /picfit/bin/picfit /picfit

ENTRYPOINT ["/picfit"]