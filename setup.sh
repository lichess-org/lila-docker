#!/bin/bash

echo "Cloning repos..."

repos=(lila lila-ws lila-db-seed lila-engine)

for repo in "${repos[@]}"; do
    [ ! -d $repo ] && git clone --depth 1 https://github.com/lichess-org/$repo.git
done

echo "Compiling js/css..."
docker run --rm -v $(pwd):/mnt node:latest bash -c "npm install -g pnpm && /mnt/lila/ui/build"

echo "Done!"
