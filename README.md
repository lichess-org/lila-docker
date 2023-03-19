# Lichess Local Development

1. Install [Docker Desktop](https://www.docker.com/products/docker-desktop/) and have it running
2. Open 2 terminals

## Terminal 1:

```
git clone https://github.com/fitztrev/lichess-docker-compose.git
cd lichess-docker-compose
./setup.sh
docker-compose up
```

## Terminal 2:

Seed your database with test data:

```
docker compose run --rm -v $(pwd)/lila:/lila mongodb bash -c "mongo --host host.docker.internal lichess /lila/bin/mongodb/indexes.js"

docker run --rm -v $(pwd)/lila-db-seed:/lila-db-seed python:3.9-slim bash -c "pip install pymongo && python /lila-db-seed/spamdb/spamdb.py --uri=mongodb://host.docker.internal/lichess"

docker compose run --rm -v $(pwd)/scripts:/scripts mongodb bash -c "mongosh --host host.docker.internal lichess --file docker/scripts/mongodb/users.js"
```

## Development Site

Visit: http://localhost:9663/

Can login with `lichess`/`password` or any of the other logins displayed in the 2nd terminal.

## Usage

To restart lila (after making changes to any Scala code):

```
docker compose restart lila
```
