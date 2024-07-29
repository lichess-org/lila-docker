# Build stage
FROM golang:1.22.1-alpine3.19 AS build

RUN apk --no-cache add make

COPY /picfit /picfit
WORKDIR /picfit

RUN make -C /picfit build

# Runtime stage
FROM alpine:3.19

COPY --from=build /picfit/bin/picfit /picfit

ENTRYPOINT ["/picfit"]