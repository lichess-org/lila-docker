#!/bin/bash

echo "Cloning repos..."

repos=(lila lila-ws lila-db-seed lila-engine lila-fishnet)

for repo in "${repos[@]}"; do
    [ ! -d $repo ] && git clone https://github.com/lichess-org/$repo.git
done

echo "Compiling js/css..."
docker run --rm -v $(pwd):/mnt node:latest bash -c "git config --global --add safe.directory /mnt/lila && npm install -g pnpm && /mnt/lila/ui/build -p"

docker-compose build

echo "Done!"
