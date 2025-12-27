# Testing Guide

## Quick Start

```bash
# Easiest way - using Makefile
make test
```

## Manual Testing

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

Example GitHub Actions workflow:

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest

    services:
      postgres:
        image: postgres:15-alpine
        env:
          POSTGRES_USER: postgres
          POSTGRES_PASSWORD: postgres
          POSTGRES_DB: urlshortener_test
        ports:
          - 5433:5432
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5

    steps:
    - uses: actions/checkout@v3

    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable

    - name: Install Diesel CLI
      run: cargo install diesel_cli --no-default-features --features postgres

    - name: Run migrations
      run: diesel migration run
      env:
        DATABASE_URL: postgres://postgres:postgres@localhost:5433/urlshortener_test

    - name: Run tests
      run: cargo test
```
