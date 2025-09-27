#!/bin/bash

# Build script for deploying the blog with Docker

set -e

NAME=$(grep '^name = ' Cargo.toml | head -n1 | cut -d'"' -f2)
VERSION=$(grep '^version = ' Cargo.toml | head -n1 | cut -d'"' -f2)
GIT_HASH=$(git rev-parse --short HEAD)

TAG="${VERSION}-${GIT_HASH}"

echo "Building Yew application..."
trunk build --release

echo "Building Docker image with tag: ${NAME}:${TAG}"
cd deploy
docker build -t ${NAME}:${TAG} -t ${NAME}:latest -f Dockerfile ..

echo "Build complete!"
echo "Created Docker image: ${NAME}:${TAG}"
echo "Also tagged as: ${NAME}:latest"
echo ""
echo "You can now run the container with:"
echo "  docker run -p 80:80 ${NAME}:${TAG}"
echo "  docker run -p 80:80 ${NAME}:latest"
