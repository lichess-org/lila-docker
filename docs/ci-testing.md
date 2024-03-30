## Testing the CI image build locally

```bash
docker build -t lila-ci -f docker/ci.Dockerfile .

docker run -it --rm -p 9663:9663 lila-ci

curl localhost:9663/api/user/lichess
```
