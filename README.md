# lila-docker

Lichess local development using Docker Compose.

The only requirements for running on your local machine are `git` and Docker Desktop. All the other dependencies (Scala, MongoDB, Node.js, etc) are installed and run in Docker containers.

## Instructions

1. Install [Docker Desktop](https://www.docker.com/products/docker-desktop/) and have it running

1. Clone this repo:

    ```bash
    git clone https://github.com/fitztrev/lila-docker
    ```

1. Setup the Lichess repos:

    ```bash
    cd lila-docker
    ./setup.sh
    ```

1. Start the services:

    ```bash
    ## start the basic services (lila, lila-ws, mongodb, redis)
    docker compose up
    ```

    Or to include optional services, depending on what you're working on, apply the appropriate profile(s):

    ```bash
    ## include stockfish services (for playing and analyzing)
    COMPOSE_PROFILES=stockfish docker compose up

    ## include external engine service
    COMPOSE_PROFILES=external-engine docker compose up

    ## include ALL optional services
    COMPOSE_PROFILES=stockfish,external-engine,search,images docker compose up
    ```

    Might take 5-10 minutes. Some services will start before others and you may see errors in the logs until everything comes online.

    Lila will be the last service to complete, at which point you can visit http://localhost:8080/ to see the site.

1. (Optional, but recommended) Seed your database with test data (users, games, etc):

    In a separate terminal:

    ```bash
    ./init-db.sh
    ```

## URLs

| Service               | URL                                                      | Profile |
| --------------------- | -------------------------------------------------------- | ------- |
| Main lila instance    | http://localhost:8080/                                   | \*      |
| Chessground demo      | http://localhost:8080/chessground/demo.html              | \*      |
| API docs              | http://localhost:8089/                                   | \*      |
| PGN Viewer            | http://localhost:8090/                                   | \*      |
| Mongodb manager       | http://localhost:8081/                                   | \*      |
| Email inbox           | http://localhost:8025/                                   | \*      |
| lila-gif              | http://localhost:6175/image.gif?fen=4k3/6KP/8/8/8/8/7p/8 | images  |
| Picfit                | http://localhost:3001/healthcheck                        | images  |
| Elasticsearch manager | http://localhost:5601/                                   | search  |

## Usage

### Scala development:

To restart lila (after making changes to any Scala code):

```bash
docker compose restart lila
```

### UI (JS/CSS) development:

To watch for Typescript/SCSS changes and automatically recompile:

```bash
docker run --rm -v $(pwd)/repos/lila:/mnt node:latest \
    bash -c "npm install -g pnpm && /mnt/ui/build -w"
```

#### Chessground:

```bash
# watch for changes
docker run --rm -v $(pwd)/repos/chessground:/mnt node:latest \
    bash -c "npm install -g pnpm && cd /mnt && pnpm install && pnpm run compile -- --watch"
```

### Code formatting:

```bash
# pnpm run lint
docker run --rm -v $(pwd)/repos/lila:/mnt node:latest \
    bash -c "npm install -g pnpm && cd /mnt && pnpm install && pnpm run lint"

# sbt scalafmtAll
docker run --rm -v $(pwd)/repos/lila:/mnt \
    sbtscala/scala-sbt:eclipse-temurin-focal-17.0.5_8_1.9.1_3.3.0 \
    bash -c "cd /mnt && sbt scalafmtAll"
```

### Berserk (Python library):

To install the development version of [Berserk](https://github.com/lichess-org/berserk) and run a sample script against your local development site:

```bash
docker run --rm -v $(pwd)/repos/berserk:/berserk -v $(pwd)/scripts:/scripts python:latest \
    bash -c "cd /berserk && pip install -e . && python /scripts/berserk-example.py"
```

### Scala Metals (IDE helper):

1. In VS Code, install the [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)
3. Cmd+Shift+P > "Dev Containers: Reopen in Container"
4. A new VS Code window will open, attached to the container instead of your host machine
5. File > Open Folder > "/workspaces/lila-docker/repos/lila" (or whichever Scala project you want to work on)
6. Install + Enable the Scala Metals extension (Cmd+Shift+X > "Scala (Metals)")
7. Cmd+Shift+P > "Metals: Import build"

Once the build has been imported, you should have code completion, go to definition, etc when you open a Scala file.

### Scalachess:

```bash
## compile
docker run --rm -v $(pwd)/repos/scalachess:/mnt \
    sbtscala/scala-sbt:eclipse-temurin-focal-17.0.5_8_1.9.1_3.3.0 \
    bash -c "cd /mnt && sbt compile"

## test
docker run --rm -v $(pwd)/repos/scalachess:/mnt \
    sbtscala/scala-sbt:eclipse-temurin-focal-17.0.5_8_1.9.1_3.3.0 \
    bash -c "cd /mnt && sbt test"

## package
docker run --rm -v $(pwd)/repos/scalachess:/mnt \
    sbtscala/scala-sbt:eclipse-temurin-focal-17.0.5_8_1.9.1_3.3.0 \
    bash -c "cd /mnt && sbt package"
```

### bbpPairings:

```bash
docker build -f docker/bbpPairings.Dockerfile . -t bbppairings
docker run --rm -v $(pwd)/repos:/mnt bbppairings bash -c "\
    git clone https://github.com/cyanfish/bbpPairings \
    && cd bbpPairings \
    && make \
    && chmod +x bbpPairings.exe \
    && cp bbpPairings.exe /mnt"

## verify
./repos/bbpPairings.exe
```
