#!/bin/bash

echo "Cloning repos..."

repos=(lila lila-ws lila-db-seed lila-engine lila-fishnet lila-gif lila-search)

for repo in "${repos[@]}"; do
    [ ! -d $repo ] && git clone https://github.com/lichess-org/$repo.git
done

cd lila
git submodule update --init
cd ..

echo "Compiling js/css..."
docker run --rm -v $(pwd):/mnt node:latest bash -c "npm install -g pnpm && /mnt/lila/ui/build"

docker-compose build

echo "Done!"
