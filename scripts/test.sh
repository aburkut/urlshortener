#!/bin/bash
# Run tests with Docker PostgreSQL

set -e

echo "Starting test databases..."
docker-compose up -d postgres_test

echo "Waiting for test database to be ready..."
timeout 30 bash -c 'until docker-compose exec -T postgres_test pg_isready -U postgres > /dev/null 2>&1; do sleep 1; done'

echo "Running migrations on test database..."
DATABASE_URL="postgres://postgres:postgres@localhost:5433/urlshortener_test" diesel migration run

echo "Running tests..."
cargo test "$@"

echo "Tests complete!"
