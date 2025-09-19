# Testing the mono image build locally

## Build

```bash
docker build -t lila-mono -f docker/mono.Dockerfile .

## or

docker buildx build -t lila-mono -f docker/mono.Dockerfile --platform linux/amd64,linux/arm64 .
```

Inspect the multi-arch image:

```bash
docker buildx imagetools inspect lila-mono
```

## Run

```bash
docker run --rm -p 8080:8080 --name lichess lila-mono
docker run -it --rm -p 8080:8080 --name lichess -e LILA_DOMAIN=custom:8080 -e LILA_URL=http://custom:8080 lila-mono
```

## Test

Visit: <http://localhost:8080/>

```bash
curl localhost:8080/api/user/lichess
```

## Get a shell in the container for debugging:

```bash
docker exec -it lichess bash
```

# GHCR

## Manually push to ghcr.io

```bash
docker tag lila-mono ghcr.io/lichess-org/lila-docker:testing
docker push ghcr.io/lichess-org/lila-docker:testing
```

## Run ghcr-hosted image

```bash
docker run --rm -p 8080:8080 --pull always ghcr.io/lichess-org/lila-docker:main
```

Visit: http://localhost:8080
