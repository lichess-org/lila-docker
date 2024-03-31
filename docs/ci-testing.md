## Testing the CI image build locally

```bash
docker build -t lila-ci -f docker/ci.Dockerfile .
docker run -it --rm -p 9663:9663 -p 9664:9664 --name lichess lila-ci
```

Visit: <http://localhost:9663/>

```bash
curl localhost:9663/api/user/lichess
```

### Get a shell in the container for debugging:

```bash
docker exec -it lichess bash
```
