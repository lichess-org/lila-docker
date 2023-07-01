# Lichess Local Development

## Instructions

1. Install [Docker Desktop](https://www.docker.com/products/docker-desktop/) and have it running

1. Set up the repos:

    ```
    git clone https://github.com/fitztrev/lichess-docker-compose.git
    cd lichess-docker-compose
    ./setup.sh
    ```

1. Start the services:

    ```
    ## start the basic services (lila, lila-ws, mongodb, redis)
    docker compose up

    ## include stockfish services (for playing and analyzing)
    COMPOSE_PROFILES=stockfish docker compose up

    ## include external engine service
    COMPOSE_PROFILES=external-engine docker compose up

    ## include all optional services
    COMPOSE_PROFILES=stockfish,external-engine,search,images docker compose up
    ```

    Might take 5-10 minutes. Some services will start before others and you may see errors in the logs until everything comes online.

    Lila will be the last service to complete, at which point you can visit http://localhost:8080/ to see the site.

1. (Optional) Seed your database with test data:

    ```
    ./init-db.sh
    ```

## Usage

To watch for Typescript/SCSS changes and automatically recompile:

```
docker run --rm -v $(pwd):/mnt node:latest bash -c "npm install -g pnpm && /mnt/lila/ui/build -w"
```

To restart lila (after making changes to any Scala code):

```
docker compose restart lila
```

Elasticsearch indexes can be managed at http://localhost:5601/

Emails can be viewed at http://localhost:8025/
