#!/bin/bash
set -e

echo "Cloning repos..."

repos=(lila lila-ws lila-db-seed lila-engine lila-fishnet lila-gif lila-search lifat scalachess api pgn-viewer chessground berserk)

for repo in "${repos[@]}"; do
    [ ! -d repos/$repo ] && git clone https://github.com/lichess-org/$repo.git repos/$repo
done

cd repos/lila
git submodule update --init
cd ../..

COMPOSE_PROFILES=$(docker compose config --profiles | xargs | sed -e 's/ /,/g') docker compose build

echo "Compiling lila js/css..."
docker compose run --rm ui bash -c "/lila/ui/build"

echo "Compiling chessground..."
docker compose run --rm ui bash -c "cd /chessground && pnpm install && pnpm run compile"

echo "Done!"
