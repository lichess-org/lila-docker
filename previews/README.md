This deploys a `previews` stack from the compose file [here](https://github.com/lichess-org/lila-docker/blob/main/stacks/previews/compose.yml).

## Initial Setup

```bash
DEPOT_TOKEN=<your API token from https://depot.dev/settings>
secretspec set DEPOT_TOKEN --profile previews --provider keyring $DEPOT_TOKEN

PORTAINER_API_KEY="your portainer API key"
secretspec set PORTAINER_API_KEY --profile previews --provider keyring $PORTAINER_API_KEY
```

## Quick Usage

```bash
preview up <link to PR>

preview down pr-{PR-number}
```

## Advanced Usage 

If you want to do steps individually or override defaults:

```bash
devenv shell

cd previews
```

### Build + Deploy

```sh
bun run build.ts --pr https://github.com/lichess-org/lila/pull/21394 --deploy

# or a branch

bun run build.ts --branch master --deploy
```

### or just build/deploy

```sh
bun run build.ts --pr https://github.com/lichess-org/lila/pull/21394
bun run build.ts --branch master

bun run deploy.ts --tag pr-21394 --dry-run
bun run deploy.ts --tag pr-21394
```

### Deploy with custom images (skip the build)

Point the stack at already-built images (e.g. GHCR's `latest`) instead of
depot-built ones, and override the subdomain / seed password:

```sh
bun run deploy.ts --tag preview \
  --subdomain test \
  --site-name "lila preview" \
  --branch master \
  --server-image ghcr.io/lichess-org/lila-server:latest \
  --assets-image ghcr.io/lichess-org/lila-assets:latest \
  --user-seed-password password
```

### Destroy

```sh
bun run deploy.ts --tag pr-21394 --remove
```
