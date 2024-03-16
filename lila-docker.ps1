# Requires -Version 5.1

function run_setup {
    New-Item .env -ItemType File -Force
    docker compose run --rm -it lila_docker_rs bash -c "cargo run --manifest-path /mnt/Cargo.toml"
    Get-Content .env | ForEach-Object {
        $key, $value = $_ -split '='
        Write-Host $key $value
        Set-Variable -Name "Env:$key" -Value $value -Scope Global
    }

    $dirs = (Get-Variable Env:DIRS -Value) -split ','
    foreach ($dir in $dirs) {
        $dirName = [System.IO.Path]::GetFileName($dir)
        New-Item -Path "repos/$dirName" -ItemType Directory -Force
    }

    $repos = (Get-Variable Env:REPOS -Value) -split ','
    Write-Host "Cloning repos..."
    foreach ($repo in $repos) {
        if ($repo -match ".*/.*") {
            $org = $repo -replace '/.*', ''
        }
        else {
            $org = "lichess-org"
        }
        $repoName = [System.IO.Path]::GetFileName($repo)
        if (-not [string]::IsNullOrEmpty($repoName) -and -not (Test-Path -Path "repos/$repoName/.git")) {
            git clone --depth 1 --origin upstream "https://github.com/$org/$repo" "repos/$repoName"
        }
    }

    git -C repos/lila submodule update --init

    docker compose build
    docker compose --profile utils build

    docker compose up -d

    Write-Host "Compiling js/css..."
    docker compose run --rm ui bash -c "/lila/ui/build"

    if ($Env:SETUP_DB -eq "true") {
        Write-Host "Setting up database"
        setup_database
    }

    Write-Host "Setup complete"
}

function run_start {
    if ([string]::IsNullOrEmpty((docker compose ps -a --services | Out-String).Trim())) {
        run_setup
    }
    else {
        if (-not [string]::IsNullOrEmpty((docker compose ps -a --services --status=exited | Out-String).Trim())) {
            docker compose start
        }
        else {
            Write-Host "There are no stopped services to resume"
        }
    }
}

function run_stop {
    $Env:COMPOSE_PROFILES = all_profiles
    docker compose stop
}

function run_down {
    $Env:COMPOSE_PROFILES = all_profiles
    docker compose down -v
}

function all_profiles {
    $profiles = docker compose config --profiles | Out-String -Stream | ForEach-Object { $_.Trim() }
    return ($profiles -join ',')
}

function build_all_profiles {
    $Env:COMPOSE_PROFILES = all_profiles
    docker compose pull
    docker compose build
}

function setup_database {
    do {
        Write-Host "Waiting for mongodb to be ready..."
        Start-Sleep -Seconds 1
    }
    while (-not (docker compose exec mongodb mongo --eval "db.adminCommand('ping')" > $null 2>&1))

    Write-Host "Adding test data..."

    docker compose run --rm mongodb bash -c "mongo --host mongodb lichess /lila/bin/mongodb/indexes.js"

    docker compose run --rm python bash -c "python /lila-db-seed/spamdb/spamdb.py --uri=mongodb://mongodb/lichess --password=$Env:PASSWORD --su-password=$Env:SU_PASSWORD --es --es-host=elasticsearch:9200"

    docker compose run --rm mongodb bash -c "mongo --quiet --host mongodb lichess /scripts/mongodb/users.js"
}

function run_formatter {
    docker compose run --rm ui bash -c "cd /lila && pnpm install && pnpm run format && (test -f /chessground/package.json && cd /chessground && pnpm install && pnpm run format) || echo 'Skipping chessground' && (test -f /pgn-viewer/package.json && cd /pgn-viewer && pnpm install && pnpm run format) || echo 'Skipping pgn-viewer'"

    docker compose run --rm --entrypoint "sbt scalafmtAll" lila
}

function show_help {
    Write-Host "Usage: $PSCommandPath [start|stop|restart|down|build|format]"
}

switch ($args[0]) {
    "--help" { show_help; break }
    "-h" { show_help; break }
    "start" { run_start; break }
    "stop" { run_stop; break }
    "restart" { run_stop; run_start; break }
    "down" { run_down; break }
    "build" { build_all_profiles; break }
    "format" { run_formatter; break }
    default { show_help; exit 1 }
}
