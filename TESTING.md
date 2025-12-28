# Testing Guide

## Quick Start

```bash
# Run tests locally (requires local Rust and database)
make test

# Run tests in Docker (fully isolated, no local dependencies)
make test-docker
```

## Docker-based Testing (Recommended for CI/CD)

Run tests in a completely isolated Docker environment:

```bash
# Using Makefile
make test-docker

# Or using script
./scripts/test-docker.sh

# Or using docker-compose directly
docker-compose --profile test up --build --abort-on-container-exit test
docker-compose --profile test down
```

**Benefits:**
- ✅ No local Rust installation required
- ✅ No local Diesel CLI required
- ✅ Consistent environment across all machines
- ✅ Perfect for CI/CD pipelines
- ✅ Isolated from local development environment

## Manual Testing (Local)

### 1. Start Test Database

```bash
docker-compose up -d postgres_test
```

This starts a PostgreSQL container on port 5433 with:
- Database name: `urlshortener_test`
- User: `postgres`
- Password: `postgres`
- Storage: tmpfs (in-memory for fast tests)

### 2. Run Migrations

```bash
DATABASE_URL="postgres://postgres:postgres@localhost:5433/urlshortener_test" diesel migration run
```

### 3. Run Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_create_short_url

# Run with output
cargo test -- --nocapture

# Run with logs
RUST_LOG=debug cargo test -- --nocapture
```

## Test Database Architecture

The project uses two PostgreSQL instances:

| Database | Port | Purpose | Storage |
|----------|------|---------|---------|
| `urlshortener` | 5432 | Development | Persistent volume |
| `urlshortener_test` | 5433 | Testing | tmpfs (in-memory) |

**Why tmpfs for tests?**
- Faster test execution
- Automatic cleanup on container stop
- No disk I/O overhead
- Isolated from development data

## Test Structure

```
tests/
└── integration_test.rs
    ├── test_create_short_url()
    ├── test_create_short_url_with_invalid_url()
    ├── test_get_full_url()
    ├── test_get_full_url_not_found()
    └── test_create_short_url_multiple_times()
```

## Cleaning Up

```bash
# Stop test database
docker-compose down postgres_test

# Clean and restart
make test-clean

# Remove all containers and volumes
docker-compose down -v
```

## Troubleshooting

### Database connection failed

```bash
# Check if container is running
docker-compose ps

# Check logs
docker-compose logs postgres_test

# Restart container
docker-compose restart postgres_test
```

### Migrations not applied

```bash
# Check migration status
DATABASE_URL="postgres://postgres:postgres@localhost:5433/urlshortener_test" diesel migration list

# Revert and reapply
DATABASE_URL="postgres://postgres:postgres@localhost:5433/urlshortener_test" diesel migration revert
DATABASE_URL="postgres://postgres:postgres@localhost:5433/urlshortener_test" diesel migration run
```

### Port already in use

If port 5433 is already used:

1. Edit `docker-compose.yml` and change postgres_test port
2. Update `Rocket.toml` `[debug.databases.postgres]` URL with new port
3. Restart containers

## CI/CD Integration

Our GitHub Actions workflow uses Docker Compose for testing, making it simple and consistent:

```yaml
test:
  name: Test (Docker)
  runs-on: ubuntu-latest

  steps:
    - uses: actions/checkout@v4

    - name: Run tests in Docker
      run: docker compose --profile test up --build --abort-on-container-exit test

    - name: Cleanup
      if: always()
      run: docker compose --profile test down -v
```

**Benefits:**
- ✅ No manual Rust/Diesel CLI installation needed
- ✅ Same environment as local Docker tests
- ✅ Faster setup (no dependency installation)
- ✅ Easier to maintain
