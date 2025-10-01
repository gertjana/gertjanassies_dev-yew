.PHONY: build build-frontend build-server test clean docker-build docker-up docker-down serve-frontend run-server

# Build everything
build:
	cargo build

# Build in release mode
build-release:
	cargo build --release

# Build frontend only
build-frontend:
	cargo build -p gertjanassies_dev-yew

# Build server only
build-server:
	cargo build -p page-stats-server

# Build frontend with Trunk
build-frontend-web:
	cd frontend && trunk build --release

# Test everything
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean
	rm -rf dist

# Serve frontend for development
serve-frontend:
	cd frontend && trunk serve --port 8080

# Run server
run-server:
	cargo run -p page-stats-server

# Docker commands
docker-build:
	docker build -f deploy/Dockerfile -t gertjanassies-combined .

docker-up:
	docker-compose up --build -d

docker-down:
	docker-compose down

docker-logs:
	docker-compose logs --tail=50

# Full development setup
dev-setup: build-frontend-web docker-up

# Check workspace
check:
	cargo check --workspace
