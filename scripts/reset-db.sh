#!/bin/bash -e

DOCKER_SECRET_PASSWORD_FILE="/run/secrets/lila_db_password"

if [ -f "$DOCKER_SECRET_PASSWORD_FILE" ]; then
    DB_PASSWORD=$(cat $DOCKER_SECRET_PASSWORD_FILE)
    echo "Using database password from Docker secret."
elif [ -n "$LILA_DB_PASSWORD" ]; then
    DB_PASSWORD="$LILA_DB_PASSWORD"
    echo "Using database password from LILA_DB_PASSWORD environment variable."
else
    DB_PASSWORD="password"
    ADDL_PARAMS="--tokens"
    echo "Using default database password."
fi

/lila-db-seed/spamdb/spamdb.py \
    --drop-db \
    --password="$DB_PASSWORD" \
    --su-password="$DB_PASSWORD" \
    --streamers \
    --coaches \
    $ADDL_PARAMS
