#!/bin/bash

echo "Choose a password for admin users:"
read -s su_password

echo "Choose a password for regular users:"
read -s password

echo "Adding test data..."

docker compose run --rm -v $(pwd)/lila:/lila mongodb bash -c \
    "mongo --host mongodb lichess /lila/bin/mongodb/indexes.js"

docker compose run --rm python bash -c \
    "pip install pymongo && python /lila-db-seed/spamdb/spamdb.py --uri=mongodb://mongodb/lichess --password=$password --su-password=$su_password --es --es-host=elasticsearch:9200"

docker compose run --rm -v $(pwd)/scripts:/scripts mongodb bash -c \
    "mongosh --host mongodb lichess --file /scripts/mongodb/users.js"
