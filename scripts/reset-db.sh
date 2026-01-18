#!/bin/bash -e

DOCKER_SECRET_PASSWORD_FILE="/run/secrets/lila_user_password"

if [ -f "$DOCKER_SECRET_PASSWORD_FILE" ]; then
    USER_PASSWORD=$(cat $DOCKER_SECRET_PASSWORD_FILE)
    echo "Using database password from Docker secret."
elif [ -n "$LILA_USER_PASSWORD" ]; then
    USER_PASSWORD="$LILA_USER_PASSWORD"
    echo "Using database password from LILA_USER_PASSWORD environment variable."
else
    USER_PASSWORD="password"
    ADDL_PARAMS="--tokens"
    echo "Using default database password."
fi

/lila-db-seed/spamdb/spamdb.py \
    --drop-db \
    --password="$USER_PASSWORD" \
    --su-password="$USER_PASSWORD" \
    --streamers \
    --coaches \
    $ADDL_PARAMS

mongosh --quiet lichess /lila/bin/mongodb/indexes.js
