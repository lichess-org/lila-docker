# Testing the CI image build locally

## Build

```bash
docker build -t lila-ci -f docker/ci.Dockerfile .

## or

docker buildx build -t lila-ci -f docker/ci.Dockerfile --platform linux/amd64,linux/arm64 .
```

Inspect the multi-arch image:

```bash
docker buildx imagetools inspect lila-ci
```

## Run

```bash
docker run -it --rm -p 9663:9663 -p 9664:9664 --name lichess lila-ci
```

## Test

Visit: <http://localhost:9663/>

```bash
curl localhost:9663/api/user/lichess
```

## Get a shell in the container for debugging:

```bash
docker exec -it lichess bash
```

# Run ghcr-hosted image

```bash
docker pull ghcr.io/lichess-org/lila-docker:lila-ws
docker run -it --rm -p 9663:9663 -p 9664:9664 ghcr.io/lichess-org/lila-docker:lila-ws
```
