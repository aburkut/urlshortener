.PHONY: help setup dev test clean migrate docker-up docker-down

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

setup: ## Initial setup (install deps, setup db)
	@echo "Installing Diesel CLI..."
	cargo install diesel_cli --no-default-features --features postgres
	@echo "Starting database..."
	docker-compose up -d postgres
	@echo "Waiting for database..."
	@sleep 5
	@echo "Running migrations..."
	diesel migration run
	@echo "Setup complete!"

dev: ## Run development server
	docker-compose up -d postgres
	cargo run

build: ## Build release binary
	cargo build --release

test: ## Run tests
	./scripts/test.sh

test-clean: ## Clean test database and run tests
	docker-compose down postgres_test
	docker-compose up -d postgres_test
	@sleep 5
	DATABASE_URL="postgres://postgres:postgres@localhost:5433/urlshortener_test" diesel migration run
	cargo test

migrate: ## Run database migrations
	diesel migration run

migrate-test: ## Run migrations on test database
	DATABASE_URL="postgres://postgres:postgres@localhost:5433/urlshortener_test" diesel migration run

docker-up: ## Start all databases
	docker-compose up -d

docker-down: ## Stop all databases
	docker-compose down

docker-logs: ## Show database logs
	docker-compose logs -f

clean: ## Clean build artifacts
	cargo clean
	docker-compose down -v

fmt: ## Format code
	cargo fmt

clippy: ## Run clippy linter
	cargo clippy -- -D warnings

check: fmt clippy test ## Run all checks (format, lint, test)

test-docker: ## Run tests in Docker (fully isolated)
	@echo "Building and running tests in Docker..."
	@docker-compose --profile test up --build --abort-on-container-exit test
	@docker-compose --profile test down

test-docker-clean: ## Clean and run tests in Docker
	@echo "Cleaning Docker test environment..."
	@docker-compose --profile test down -v
	@docker rmi -f urlshortener-test 2>/dev/null || true
	@echo "Building and running tests in Docker..."
	@docker-compose --profile test up --build --abort-on-container-exit test
	@docker-compose --profile test down
