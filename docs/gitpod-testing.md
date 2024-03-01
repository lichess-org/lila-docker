## Testing the auto PR checkout locally

```bash
cd command

## test with no PR specified
GITPOD_WORKSPACE_URL=https://example.com \
GITPOD_WORKSPACE_CONTEXT={} \
cargo run -- setup

## test with PR specified
GITPOD_WORKSPACE_URL=https://example.com \
GITPOD_WORKSPACE_CONTEXT='{"envvars":[{"name":"LILA_PR","value":"14738"}]}' \
cargo run -- setup
```

Verify with the `repos` folder in `./command/` (not the other main `repos` folder at the root level)
