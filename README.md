# URL Shortener

A production-ready URL shortening service built with Rust, Rocket, and PostgreSQL.

## Features

✅ **URL Validation** - Validates URLs before creating short links
✅ **Collision Detection** - Handles ID collisions with configurable retry limit
✅ **Deduplication** - Optionally reuses existing short URLs for the same long URL
✅ **Click Tracking** - Tracks the number of clicks for each short URL
✅ **TTL Support** - Set expiration dates for short URLs
✅ **Configurable** - All settings configurable via environment variables
✅ **Structured Logging** - Using `tracing` for comprehensive logging
✅ **Structured Errors** - Proper error handling with meaningful responses
✅ **CORS Support** - Configurable CORS settings
✅ **Database Migrations** - Managed with Diesel

## Prerequisites

- Rust 1.70+
- Docker & Docker Compose (for database)
- Diesel CLI: `cargo install diesel_cli --no-default-features --features postgres`

## Quick Start

```bash
# Option 1: Using Makefile (recommended)
make setup  # Install dependencies and setup database
make dev    # Run development server
make test   # Run tests

# Option 2: Manual setup
docker-compose up -d postgres
sleep 5
diesel migration run
cargo run
```

## Setup

### 1. Database Setup

```bash
# Start PostgreSQL with Docker Compose
docker-compose up -d postgres

# Wait for database to be ready
sleep 5

# Run migrations
diesel migration run
```

### 2. Configuration

Copy `.env.example` to `.env` and configure:

```bash
cp .env.example .env
```

Available configuration options:

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | `8080` | Server port |
| `SHORT_ID_LENGTH` | `7` | Length of generated short IDs |
| `MAX_COLLISION_ATTEMPTS` | `10` | Max retries for ID collision |
| `ENABLE_DEDUPLICATION` | `true` | Reuse existing short URLs for same long URL |
| `DEFAULT_TTL_DAYS` | None | Default TTL for URLs (in days) |
| `CORS_ALLOWED_ORIGINS` | `*` | Comma-separated list of allowed origins |
| `RUST_LOG` | `info` | Logging level (trace, debug, info, warn, error) |

Database configuration is in `Rocket.toml`:

```toml
[default.databases.postgres]
url = "postgres://user:password@localhost/urlshortener"
```

## Running

```bash
# Start database
docker-compose up -d postgres

# Development
cargo run

# Production (optimized build)
cargo build --release
./target/release/server

# Stop database
docker-compose down
```

## API Endpoints

### Create Short URL

```http
POST /create_short_url
Content-Type: application/json

{
  "url": "https://example.com/very/long/url",
  "ttl_days": 30  // optional
}
```

Response:
```json
{
  "short_url": "abc1234",
  "expires_at": "2025-01-27T10:30:00"  // optional
}
```

### Access Short URL

```http
GET /{short_url}
```

Redirects to the original URL (HTTP 303) or returns error if not found/expired.

## Error Responses

All errors return JSON with structure:

```json
{
  "error": "Error message",
  "status": 400
}
```

HTTP Status Codes:
- `400` - Invalid URL format
- `404` - Short URL not found
- `410` - URL has expired
- `500` - Internal server error (database, collision limit exceeded)

## Testing

Tests use a separate PostgreSQL database running in Docker.

```bash
# Using Makefile (recommended)
make test

# Using script
./scripts/test.sh

# Or manually:
# 1. Start test database
docker-compose up -d postgres_test

# 2. Run migrations on test database
DATABASE_URL="postgres://postgres:postgres@localhost:5433/urlshortener_test" diesel migration run

# 3. Run tests
cargo test

# Run with logs
RUST_LOG=debug cargo test -- --nocapture
```

**Database Configuration:**
- Development database: `localhost:5432/urlshortener`
- Test database: `localhost:5433/urlshortener_test` (in-memory, tmpfs)

The test database uses tmpfs for faster performance and automatic cleanup.

## Available Commands

Using the Makefile for common tasks:

```bash
make help          # Show all available commands
make setup         # Initial setup (install deps, setup db)
make dev           # Run development server
make build         # Build release binary
make test          # Run tests
make test-clean    # Clean test database and run tests
make migrate       # Run database migrations
make migrate-test  # Run migrations on test database
make docker-up     # Start all databases
make docker-down   # Stop all databases
make docker-logs   # Show database logs
make clean         # Clean build artifacts
make fmt           # Format code
make clippy        # Run clippy linter
make check         # Run all checks (format, lint, test)
```

## Monitoring & Metrics

The service logs structured events using `tracing`:

- URL creation events with full URL
- Collision warnings
- Click events with redirect information
- Database errors
- Expired URL access attempts

Example log output:
```
INFO Creating short URL for: https://example.com
INFO Successfully created short URL: abc1234 -> https://example.com
INFO Redirecting abc1234 -> https://example.com (clicks: 1)
WARN Collision detected for short URL: abc1234 (attempt 1)
```

## Architecture

```
src/
├── bin/
│   └── server.rs          # Server entry point with logging setup
├── routes/
│   ├── mod.rs            # CORS and database connection
│   └── shortener.rs      # URL shortening logic
├── models/
│   └── mod.rs            # Database models and request/response types
├── repositories/
│   └── mod.rs            # Database operations
├── id_provider.rs        # ID generation (NanoID)
├── errors.rs             # Error types and handling
├── config.rs             # Configuration management
├── schema.rs             # Database schema (Diesel)
└── lib.rs               # Library exports

migrations/               # Database migrations
tests/                   # Integration tests
```

## Production Deployment

### Environment Variables

Set these in production:
```bash
export PORT=8080
export SHORT_ID_LENGTH=8  # Longer IDs for large scale
export CORS_ALLOWED_ORIGINS=https://yourdomain.com
export ENABLE_DEDUPLICATION=true
export DEFAULT_TTL_DAYS=365
export RUST_LOG=info
```

### Database

Configure connection pooling in `Rocket.toml`:
```toml
[release.databases.postgres]
url = "postgres://user:password@localhost/urlshortener"
max_connections = 20
```

### Scaling Considerations

- **ID Length**: Increase `SHORT_ID_LENGTH` for higher capacity (7 chars ≈ 3.5 trillion combinations)
- **Database Indexes**: Already optimized with indexes on `short` and `expires_at`
- **Caching**: Consider adding Redis for frequently accessed URLs
- **Cleanup**: Add a cron job to delete expired URLs

## License

MIT
