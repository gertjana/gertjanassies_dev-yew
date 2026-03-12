.PHONY: help build build-frontend build-server test clean docker-build docker-up docker-down serve-frontend serve-dist run-server meta-pages

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

# Build frontend with Trunk and generate social meta pages
build-frontend-web: ## Build frontend with Trunk (release) + generate meta pages
	cd frontend && trunk build --release
	cargo run -p meta-gen -- --content-dir content --dist-dir dist

# Test everything
test: ## Run all tests
	cargo test

# Clean build artifacts
clean: ## Clean build artifacts
	cargo clean
	rm -rf dist

# Serve frontend for development (meta pages not generated; use build-frontend-web for a full build)
serve-frontend: ## Start frontend development server (live reload, no meta pages)
	cd frontend && trunk serve --port 8080

# Run server
run-server: ## Run the page stats server
	cargo run -p page-stats-server

# Docker commands
docker-build: ## Build Docker image
	docker build -f deploy/Dockerfile -t gertjanassies-combined .

# Serve the dist/ folder as a static site — use after build-frontend-web
# Replicates Nginx try_files: serves real files first, falls back to index.html.
# Real browsers get the JS redirect; curl sees the raw meta tags.
# Requires Python 3 (standard library only).
serve-dist: ## Serve dist/ on port 8080, real files first then SPA fallback
	python3 tools/serve_dist.py 8080 dist
# Check workspace
check: ## Check workspace for errors
	cargo check --workspace

# Generate static meta pages for social sharing (run after build-frontend-web)
meta-pages: ## Generate dist/post/*/index.html with OG/Twitter meta tags
	cargo run -p meta-gen -- --content-dir content --dist-dir dist
