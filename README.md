# Lichess Local Development

## Instructions

1. Install [Docker Desktop](https://www.docker.com/products/docker-desktop/) and have it running

2. Run these commands in your terminal:

    ```
    git clone https://github.com/fitztrev/lichess-docker-compose.git
    cd lichess-docker-compose
    ./setup.sh
    docker-compose up
    ```

    Might take 5-10 minutes. Some services will start before others and you may see errors in the logs until everything comes online.

    Lila will be the last service to complete, at which point you can visit http://localhost:8080/ to see the site.

1. (Optional) Seed your database with test data:

    ```
    ./init-db.sh
    ```

## Usage

To restart lila (after making changes to any Scala code):

```
docker compose restart lila
```

Elasticsearch indexes can be seen managed: http://localhost:5601/

