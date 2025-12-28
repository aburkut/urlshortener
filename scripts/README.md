# Scripts

This directory contains utility scripts for the project.

## Available Scripts

### `test.sh`
Run tests locally with local Rust installation and Docker database.

```bash
./scripts/test.sh
```

**Requirements:**
- Rust and Cargo installed locally
- Diesel CLI installed locally
- Docker and Docker Compose

### `test-docker.sh` (Recommended)
Run tests in a fully isolated Docker environment.

```bash
./scripts/test-docker.sh
```

**Requirements:**
- Docker and Docker Compose only

**Benefits:**
- No local Rust or Diesel CLI required
- Consistent test environment
- Perfect for CI/CD
- Isolated from local development

## Quick Comparison

| Feature | test.sh | test-docker.sh |
|---------|---------|----------------|
| Local Rust required | ✅ Yes | ❌ No |
| Diesel CLI required | ✅ Yes | ❌ No |
| Docker required | ✅ Yes | ✅ Yes |
| Speed | Fast | Slower (initial build) |
| Isolation | Partial | Complete |
| CI/CD friendly | Good | Excellent |

## Usage Examples

```bash
# Quick local test
make test

# Full isolated test
make test-docker

# Clean and test in Docker
make test-docker-clean
```
