# lila-docker

Lichess local development using Docker Compose, for developing on Mac or Linux.

The only requirements for running on your local machine are `git` and Docker Desktop. All the other dependencies (Scala, MongoDB, Node.js, etc) are installed and run in Docker containers.

## Instructions

1. Install [Docker Desktop](https://www.docker.com/products/docker-desktop/) and have it running

1. Clone this repo:

    ```bash
    git clone https://github.com/fitztrev/lila-docker
    ```

1. Start the services

    ```bash
    cd lila-docker
    ./lila-docker start
    ```

    Might take 5-10 minutes. Some services will start before others and you may see errors in the logs until everything comes online.

    Lila will be the last service to complete, at which point you can visit http://localhost:8080/ to see the site.

### Shutting down / Resetting

When you're done working, you can shut down the services with:

```bash
./lila-docker stop
```

## URLs

| Service               | URL                                                      | Availability |
| --------------------- | -------------------------------------------------------- | ------------ |
| Main lila instance    | http://localhost:8080/                                   | \*           |
| Chessground demo      | http://localhost:8080/chessground/demo.html              | \*           |
| API docs              | http://localhost:8089/                                   | \*           |
| PGN Viewer            | http://localhost:8090/                                   | \*           |
| Mongodb manager       | http://localhost:8081/                                   | \*           |
| Email inbox           | http://localhost:8025/                                   | \*           |
| lila-gif              | http://localhost:6175/image.gif?fen=4k3/6KP/8/8/8/8/7p/8 | optional     |
| Picfit                | http://localhost:3001/healthcheck                        | optional     |
| Elasticsearch manager | http://localhost:5601/                                   | optional     |

## Usage

### Scala development:

To restart lila (after making changes to any Scala code):

```bash
docker compose restart lila
```

### UI (JS/CSS) development:

To watch for Typescript/SCSS changes and automatically recompile:

```bash
docker compose run --rm ui bash -c "/lila/ui/build -w"
```

### To add translation keys:

After modifying a `translation/source/*.xml` file, run:

```bash
docker compose run --rm ui bash -c "/lila/bin/trans-dump"
```

### Code formatting:

```bash
docker compose run --rm ui bash -c "cd /lila && pnpm install && pnpm run format"
docker compose run --rm ui bash -c "cd /chessground && pnpm install && pnpm run format"
docker compose run --rm ui bash -c "cd /pgn-viewer && pnpm install && pnpm run format"

# sbt scalafmtAll
docker run --rm -v $(pwd)/repos/lila:/lila \
    sbtscala/scala-sbt:eclipse-temurin-focal-17.0.5_8_1.9.1_3.3.0 \
    bash -c "cd /lila && sbt scalafmtAll"
```

### Berserk (Python library):

To install the development version of [Berserk](https://github.com/lichess-org/berserk) and run a sample script against your local development site:

```bash
docker run --rm -v $(pwd)/repos/berserk:/berserk -v $(pwd)/scripts:/scripts python:latest \
    bash -c "cd /berserk && pip install -e . && python /scripts/berserk-example.py"
```

### Scala Metals (IDE helper):

1. In VS Code, open this `lila-docker` project and install the [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)
2. Cmd+Shift+P > "Dev Containers: Reopen in Container"
3. A new VS Code window will open, attached to the container instead of your host machine
4. File > Open Folder > "/workspaces/lila-docker/repos/lila" (or whichever Scala project you want to work on)
5. Install + Enable the Scala Metals extension (Cmd+Shift+X > "Scala (Metals)")
6. Cmd+Shift+P > "Metals: Import build"

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

### Developing Chessground locally

By default, your local lila instance will use the version of chessground that is published to npm. If you want to make changes to that library and see them reflected in your local lila instance, you can do the following:

1. Update the `package.json` in the `lila` repo:

    ```diff
    "dependencies": {
    -  "chessground": "^8.3.11",
    +  "chessground": "link:/chessground",
    }
    ```

2. Start the chessground compiler in watch mode:

    ```bash
    docker compose run --rm ui bash -c "cd /chessground && pnpm install && pnpm run compile --watch"
    ```

3. Start the lila ui build in watch mode:

    ```bash
    docker compose run --rm ui bash -c "/lila/ui/build -w"
    ```

Then you can see the updated chessground demo at http://localhost:8080/chessground/demo.html and when you refresh lila, it will use the local copy of chessground.

### Developing PGN Viewer locally

To re-compile the PGN Viewer after making changes:

```bash
docker compose run --rm ui bash -c "cd /pgn-viewer && pnpm run sass-dev && pnpm run bundle-dev"
```

See the changes on the PGN Viewer demo page: http://localhost:8090/
