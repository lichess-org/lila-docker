#!/bin/bash -e

/lila-db-seed/spamdb/spamdb.py \
    --drop-db \
    --password=$1 \
    --su-password=$1 \
    --streamers \
    --coaches \
    --tokens
