# lila-docker

[![Publish Image](https://github.com/lichess-org/lila-docker/actions/workflows/publish-image.yml/badge.svg)](https://github.com/lichess-org/lila-docker/actions/workflows/publish-image.yml)

Lichess development environment using Docker Compose, for developing on Mac, Linux, or Windows (via WSL).

The only requirement for running on your local machine is Docker Desktop, and optionally git. All the other dependencies (Scala, MongoDB, Node.js, etc) are installed and run in Docker containers.

![image](https://github.com/user-attachments/assets/0192574a-4cb7-42da-a19e-e75af24b0565)

## Instructions

1. Install [Docker Desktop](https://www.docker.com/products/docker-desktop/) and have it running

1. Clone this repo:

    ```bash
    git clone https://github.com/lichess-org/lila-docker
    ```

1. Start the services

    ```bash
    cd lila-docker
    ./lila-docker start
    ```

    Then complete the dialog to set up lila-docker.

    Starting new services may take 5-10 minutes. Some services will start before others and you may see errors in the logs until everything comes online.

    Lila requires about 12GB of RAM to build. Make sure there is enough RAM available, especially when using Docker Desktop, which allocates 50% of the available RAM by default.

    Lila will be the last service to complete, at which point you can visit http://localhost:8080/ to see the site.

### Stopping

To stop the containers, for later resuming via `./lila-docker start`:

```bash
./lila-docker stop
```

To remove the containers:

```bash
./lila-docker down
```

### Adding a new service

To add a new optional service after the initial setup has already been done:

```bash
./lila-docker add-services
```

Select the service you want to add from the list of options.

NOTE: This will not affect the existing services, only the new ones among the selected services will be added.

## URLs

Always available:

| Service            | URL                    |
| ------------------ | ---------------------- |
| Main lila instance | http://localhost:8080/ |

Depending on which optional services you start:

| Service               | URL                                                      |
| --------------------- | -------------------------------------------------------- |
| Mongodb manager       | http://localhost:8081/                                   |
| Email inbox           | http://localhost:8025/                                   |
| lila-gif              | http://localhost:6175/image.gif?fen=4k3/6KP/8/8/8/8/7p/8 |
| Picfit                | http://localhost:3001/healthcheck                        |
| Elasticsearch manager | http://localhost:8092/                                   |
| lila-search docs      | http://localhost:9673/docs/                              |
| API docs              | http://localhost:8089/                                   |
| Chessground           | http://localhost:8090/demo.html                          |
| PGN Viewer            | http://localhost:8091/                                   |
| Prometheus            | http://localhost:9090/                                   |
| InfluxDB              | http://localhost:8086/ (admin/password)                  |

## Usage

### Scala development:

To restart lila (after making changes to any Scala code):

```bash
./lila-docker lila restart
```

### UI (JS/CSS) development:

To watch for Typescript/SCSS changes and automatically recompile:

```bash
./lila-docker ui --watch
```

### Updating Routes

If you edit the `conf/routes` file, you'll need to update the route cache.

```bash
docker compose exec lila ./lila.sh playRoutes
```

### To update translation keys:

After modifying a `translation/source/*.xml` file, run:

```bash
docker compose run --rm -w /lila ui pnpm run i18n-file-gen
```

### Code formatting:

```bash
./lila-docker format
```

### Optional: Make the database persistent

```bash
docker compose cp mongodb:/data/db ./database
```

Then, in `compose.yml`,  under `services.mongodb.volumes`: 

Add `- ./database:/data/db`

### Berserk (Python library):

To install the development version of [Berserk](https://github.com/lichess-org/berserk) and run a sample script against your local development site:

```bash
docker compose run --rm -w /berserk python sh -c "pip install -e . && python /scripts/berserk-example.py"
docker compose run --rm -w /berserk python sh -c "pip install -e . && python /scripts/berserk-connect-bots.py"
```

### Scala Metals (IDE helper):

1. In VS Code, open this `lila-docker` project and install the [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)
2. Cmd+Shift+P > "Dev Containers: Rebuild and Reopen in Container"
3. A new VS Code window will open, attached to the container instead of your host machine
4. File > Open Folder > "/workspaces/lila-docker/repos/lila" (or whichever Scala project you want to work on)
5. Install + Enable the Scala Metals extension (Cmd+Shift+X > "Scala (Metals)")
6. Cmd+Shift+P > "Metals: Import build"

Once the build has been imported, you should have code completion, go to definition, etc when you open a Scala file.

### Scalachess:

If you're making changes to the Scalachess library, you can have lila use it instead of the published Maven version:

1. Update the `build.sbt` file in the scalachess repo:

    ```diff
    -  ThisBuild / version           := "15.6.7"
    +  ThisBuild / version           := "my-test-1"  # give it a custom version
    ```

2. Update the `Dependencies.scala` file in the lila repo:

    ```diff
    -  val chess = "org.lichess" %% "scalachess" % "15.6.7"
    +  val chess = "org.lichess" %% "scalachess" % "my-test-1"
    ```

3. Publish the local scalachess changes and restart lila:

    ```bash
    docker compose exec -w /scalachess lila sbt publishLocal
    docker compose restart lila
    ```

Other Scalachess commands:

```bash
## formatting
docker compose run --rm -w /scalachess --entrypoint="sbt check" lila
docker compose run --rm -w /scalachess --entrypoint="sbt prepare" lila

## compile
docker compose run --rm -w /scalachess --entrypoint="sbt compile" lila

## test
docker compose run --rm -w /scalachess --entrypoint="sbt testKit/test" lila

## package
docker compose run --rm -w /scalachess --entrypoint="sbt package" lila
```

### Developing Chessground or PGN-Viewer locally

By default, your local lila instance will use the version of chessground + pgn-viewer that are published to npm. If you want to make changes to either library and see them reflected in your local lila instance, you can do the following:

1. Start lila-docker with the optional chessground and/or pgn-viewer services

1. Have lila use the local copy:

    ```bash
    docker compose run --rm -w /lila ui bash -c "pnpm link /chessground"
    ```

1. Start the compilers in watch mode:

    ```bash
    docker compose run --rm -w /chessground ui bash -c "pnpm install && pnpm run bundle && pnpm run compile --watch"

    docker compose run --rm -w /pgn-viewer ui bash -c "pnpm install && pnpm run dist"
    ```

    See the updated chessground demo: http://localhost:8090/demo.html
    See the updated pgn-viewer demo: http://localhost:8091/

1. Start the lila ui build in watch mode:

    ```bash
    ./lila-docker ui
    ```

    and when you refresh lila, it will use the local copy of chessground and/or pgn-viewer.

### Monitoring

To view the monitoring dashboards, start your environment with the `Monitoring` optional services enabled. You can view the metrics at:

| Service      | URL                                        |
| ------------ | ------------------------------------------ |
| lila metrics | http://localhost:8080/prometheus-metrics/x |
| Prometheus   | http://localhost:9090/                     |
| InfluxDB     | http://localhost:8086/ (admin/password)    |

You can run queries against the InfluxDB database using curl:

```bash
curl --get http://localhost:8086/query \
    --header "Authorization: Token secret" \
    --data-urlencode 'db=kamon'  \
    --data-urlencode 'q=show measurements;'

curl --get http://localhost:8086/query \
    --header "Authorization: Token secret" \
    --data-urlencode 'db=kamon'  \
    --data-urlencode 'q=show field keys'

curl --get http://localhost:8086/query \
    --header "Authorization: Token secret" \
    --data-urlencode 'db=kamon' \
    --data-urlencode 'q=show tag keys'
```
