.PHONY: help build build-frontend build-server test clean docker-build docker-up docker-down serve-frontend run-server

.DEFAULT_GOAL := help

# Help target
help: ## Show this help message
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

# Build everything
build: ## Build the project
	cargo build

# Build in release mode
build-release: ## Build in release mode
	cargo build --release

# Build frontend only
build-frontend: ## Build frontend only
	cargo build -p gertjanassies_dev-yew

# Build server only
build-server: ## Build server only
	cargo build -p page-stats-server

# Build frontend with Trunk
build-frontend-web: ## Build frontend with Trunk (release)
	cd frontend && trunk build --release

# Test everything
test: ## Run all tests
	cargo test

# Clean build artifacts
clean: ## Clean build artifacts
	cargo clean
	rm -rf dist

# Serve frontend for development
serve-frontend: ## Start frontend development server
	cd frontend && trunk serve --port 8080

# Run server
run-server: ## Run the page stats server
	cargo run -p page-stats-server

# Docker commands
docker-build: ## Build Docker image
	docker build -f deploy/Dockerfile -t gertjanassies-combined .

# Check workspace
check: ## Check workspace for errors
	cargo check --workspace
