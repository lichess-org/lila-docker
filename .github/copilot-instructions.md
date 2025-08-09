# Lichess Development Environment (lila-docker)

Always reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.

## Working Effectively

### Bootstrap and Setup
- **Minimum requirements**: 12GB RAM (Docker allocates 50% by default), Docker Desktop installed
- **Initial setup**: `./lila-docker start` -- NEVER CANCEL: Takes 15-45 minutes depending on network. Set timeout to 60+ minutes.
  - Runs interactive setup with options for services, database seeding, passwords
  - Choose "Advanced" for development, "Quick" for basic testing only
  - Default passwords are "password" for both admin and regular users
  - Database seeding is recommended and takes ~7 minutes
- **Alternative setup commands**:
  - `./lila-docker build` -- Pre-builds all images. NEVER CANCEL: Set timeout to 45+ minutes.
  - `COMPOSE_PROFILES=base docker compose up -d` -- Starts core services without full setup

### Network Issues and Workarounds
- **SSL/Certificate issues**: Common in CI environments. Use `NODE_TLS_REJECT_UNAUTHORIZED=0` environment variable:
  ```bash
  export NODE_TLS_REJECT_UNAUTHORIZED=0
  docker compose build --build-arg NODE_TLS_REJECT_UNAUTHORIZED=0
  ```
- **Maven/SBT download failures**: May occur in restricted networks. Services will retry automatically.
- **UI build failures**: If pnpm/corepack fails due to network, the main Scala application can still function

### Core Services Management
- **Start services**: `./lila-docker start` or `COMPOSE_PROFILES=base docker compose up -d`
- **Stop services**: `./lila-docker stop`
- **Restart services**: `./lila-docker restart` 
- **Remove containers and data**: `./lila-docker down`
- **View logs**: `./lila-docker logs` or `docker compose logs -f [service]`
- **Check status**: `./lila-docker status` or `docker compose ps`

### Development Workflows

#### Scala Development
- **Restart lila after code changes**: `./lila-docker lila restart` -- Takes 2-5 minutes for restart
- **Clean build**: `./lila-docker lila clean` -- Use if compilation errors occur
- **Lila compilation**: NEVER CANCEL: Initial compilation takes 15-45 minutes. Monitor with `docker compose logs -f lila`
- **Expected compilation pattern**: Compiles modules sequentially (core, coreI18n, streamer, study, etc.)

#### UI Development (TypeScript/SCSS)
- **Compile UI**: `./lila-docker ui` -- Takes 5-15 minutes
- **Watch mode**: `./lila-docker ui --watch` -- Automatically recompiles on changes
- **UI build troubleshooting**: If network issues occur, try with SSL workaround above

#### Code Formatting
- **Format all code**: `./lila-docker format` -- Runs scalafmt, pnpm format. NEVER CANCEL: Takes 5-10 minutes.
- **Always run before commits** to match Lichess code style

#### Database Operations
- **Reset database**: `./lila-docker db` -- Drops and re-seeds with test data. Takes ~7 minutes.
- **Database seeding includes**: Test users, games, tournaments, puzzles, forums, teams, messages
- **Database is accessible**: MongoDB on localhost:27017, use mongo-express service for web interface

### Adding Optional Services
- **Add services**: `./lila-docker add-services` -- Interactive selection of additional features
- **Available profiles**: api-docs, chessground, email, external-engine, gifs, mongo-express, monitoring, pgn-viewer, push, search, stockfish-analysis, stockfish-play, thumbnails, utils
- **After adding services**: Restart with new profile: `COMPOSE_PROFILES=base,new-profile docker compose up -d`

### Repository Management  
- **Check git status**: `./pull-all` -- Shows status of all cloned repositories
- **Update repositories**: `./pull-all --pull` -- Pulls latest changes from all repos
- **Repository locations**: All source code in `./repos/` directory
  - `./repos/lila/` - Main Lichess application (Scala/Play Framework)
  - `./repos/lila-ws/` - WebSocket server (Scala)
  - `./repos/chessground/` - Chess board UI (TypeScript)
  - `./repos/pgn-viewer/` - PGN viewer component (TypeScript)
  - And others...

## Validation and Testing

### Application Access
- **Main application**: http://localhost:8080/ -- WAIT: May take 45+ minutes after initial start for lila compilation to complete
- **Check if ready**: `curl -I http://localhost:8080/` -- Returns HTTP headers when ready
- **Login credentials**: Use seeded test users with password "password"

### Service Endpoints (when optional services enabled)
- **Database admin**: http://localhost:8081/ (mongo-express profile)
- **Email inbox**: http://localhost:8025/ (email profile)  
- **API docs**: http://localhost:8089/ (api-docs profile)
- **Chessground demo**: http://localhost:8090/demo.html (chessground profile)
- **PGN Viewer demo**: http://localhost:8091/ (pgn-viewer profile)
- **Monitoring**: http://localhost:9090/ (monitoring profile)

### Manual Validation Steps
1. **Verify containers are running**: `docker compose ps` -- All services should show "Up" status
2. **Check lila compilation**: `docker compose logs lila | tail` -- Look for "done compiling" messages
3. **Test database connectivity**: `docker compose exec mongodb mongosh --eval "db.runCommand({ping: 1})"`
4. **Verify database seeding**: `docker compose exec mongodb mongosh lichess --eval "db.user4.countDocuments()"`
5. **Test HTTP response**: `curl -s -o /dev/null -w "%{http_code}" http://localhost:8080/` -- Should return 200 when ready

### Development Testing Scenarios
After making changes, always test these workflows:
1. **Login flow**: Visit http://localhost:8080/, click "Sign in", use test user credentials
2. **Play a game**: Create or join a game, make moves, verify real-time updates
3. **Browse site features**: Check puzzles, tournaments, studies, forums work correctly

## Timing Expectations and Timeouts

### Critical: Always Use Proper Timeouts
- **Initial setup**: 45-60 minutes (NEVER CANCEL)
- **Docker builds**: 45+ minutes (NEVER CANCEL)  
- **Lila compilation**: 15-45 minutes (NEVER CANCEL)
- **Database seeding**: 7-10 minutes (NEVER CANCEL)
- **UI compilation**: 5-15 minutes (NEVER CANCEL)
- **Code formatting**: 5-10 minutes (NEVER CANCEL)
- **Service restarts**: 2-5 minutes
- **Database operations**: 1-2 minutes

### Build Time Measurements (Actual)
- Rust command tool compilation: ~52 seconds
- Container startup: ~11 seconds  
- Database seeding: ~7 minutes
- Database indexing: ~16 seconds
- Repository cloning: ~3-5 minutes

## Common Issues and Solutions

### Build Failures
- **SSL certificate errors**: Use `NODE_TLS_REJECT_UNAUTHORIZED=0` environment variable
- **Network timeouts**: Retry commands, check internet connectivity
- **Out of memory**: Increase Docker memory allocation to 8GB+ 
- **Port conflicts**: Stop other services using ports 8080, 27017, 6379

### Runtime Issues  
- **Nginx restarting**: Wait for lila compilation to complete
- **502 Bad Gateway**: Lila service is still starting up, wait longer
- **Database connection errors**: Ensure MongoDB container is healthy: `docker compose ps`
- **Missing UI assets**: Run `./lila-docker ui` to compile frontend

### Development Issues
- **Scala compilation errors**: Run `./lila-docker lila clean` then restart
- **Hot reload not working**: Use `./lila-docker ui --watch` for frontend, restart lila for backend
- **Database state issues**: Reset with `./lila-docker db`

## Environment and Configuration

### Generated Files
- `.env` - Docker user ID and group ID configuration
- `settings.env` - Setup configuration (profiles, passwords, options)
- `settings.toml` - Configuration in TOML format
- `.pnpm-store/` - pnpm package cache directory

### Important Commands Reference
```bash
# Essential commands that work in all environments
./lila-docker start                    # Interactive setup
./lila-docker build                    # Pre-build all images  
./lila-docker status                   # Check git repos and containers
docker compose ps                      # Container status
docker compose logs -f lila            # Monitor lila compilation
COMPOSE_PROFILES=base docker compose up -d  # Start core services

# Development commands
./lila-docker lila restart             # After Scala changes
./lila-docker ui --watch               # Frontend development  
./lila-docker format                   # Code formatting
./lila-docker db                       # Reset database

# Troubleshooting commands  
docker compose logs [service]          # View service logs
docker compose exec mongodb mongosh    # Database access
curl -I http://localhost:8080/         # Test HTTP connectivity
```

### File Locations
- **Main lila code**: `./repos/lila/`
- **Configuration**: `./conf/` directory
- **Scripts**: `./scripts/` directory  
- **Docker files**: `./docker/` directory
- **Documentation**: `./docs/` directory

## Key Points for Agents
- **NEVER cancel long-running builds or compilation** - Lila compilation can take 45+ minutes
- **Always wait for lila compilation to complete** before testing the application
- **Use proper timeouts** - Set 60+ minutes for builds, 30+ minutes for compilation
- **Environment variables matter** - Use SSL workarounds in CI environments
- **Database must be seeded** - Application won't work properly without test data
- **Monitor logs actively** - Use `docker compose logs -f` to track progress
- **Network issues are common** - Have fallback approaches and workarounds ready