#!/bin/bash

echo "Cloning repos..."

repos=(lila lila-ws lila-db-seed lila-engine lila-fishnet lila-gif lila-search lifat scalachess api pgn-viewer)

for repo in "${repos[@]}"; do
    [ ! -d repos/$repo ] && git clone https://github.com/lichess-org/$repo.git repos/$repo
done

cd repos/lila
git submodule update --init
cd ../..

echo "Compiling js/css..."
docker run --rm -v $(pwd)/repos/lila:/mnt node:latest bash -c "npm install -g pnpm && /mnt/ui/build"

COMPOSE_PROFILES=$(docker-compose config --profiles | xargs | sed -e 's/ /,/g') docker-compose build

echo "Done!"
