This deploys a complete `lila-preview` stack from the compose file
[here](https://github.com/lichess-org/lila-docker/blob/main/stacks/lila-preview/compose.yml),
meaning `lila`, its dependencies, and some helpful tools, to a URL like
https://preview.pr-21426.lichess.app.

## Dependency

[devenv](https://devenv.sh) is required. It provides the `preview` command and
the tools it uses (`bun`, `secretspec`, `depot`). Install it, then run
`devenv shell` from the repo root.

## Initial Setup

```bash
DEPOT_TOKEN=<your API token from https://depot.dev/settings>
secretspec set DEPOT_TOKEN --profile previews --provider keyring $DEPOT_TOKEN
```

## Usage

```bash
preview up <PR number or link>

preview down <PR number or link>
```

## How it works

`preview` ([preview.ts](preview.ts)) dispatches the
[Depot CI workflow](../.depot/workflows/preview.yml), waits for it, and prints
the site URL and test user passwords.

The workflow builds the lila server and assets images from the PR
([docker/](docker/)), then [deploy.ts](deploy.ts) creates or updates the
Portainer stack pointing at them. `down` just removes the stack.

The Portainer API key is a Depot CI secret:

```bash
depot ci secrets add PORTAINER_API_KEY --repo lichess-org/lila-docker
```
