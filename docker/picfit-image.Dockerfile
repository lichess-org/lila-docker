# Build stage
FROM golang:1.23.5-alpine3.21 AS build

RUN apk --no-cache add make

COPY /picfit /picfit
WORKDIR /picfit

RUN make -C /picfit build

# Runtime stage
FROM alpine:3.21

COPY --from=build /picfit/bin/picfit /picfit

ENTRYPOINT ["/picfit"]