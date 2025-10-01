#!/bin/bash
# GitHub Actions Validation Script
# This script simulates the key steps that GitHub Actions will perform

set -e

echo "🔍 Validating GitHub Actions workflow compatibility..."

# Check workspace structure
echo "📁 Workspace structure:"
ls -la

echo "📦 Workspace members:"
cargo metadata --format-version 1 | jq -r '.workspace_members[]'

# Check Cargo operations
echo "🔍 Checking workspace..."
cargo check --workspace

echo "🧪 Testing workspace..."
cargo test --workspace

# Check version extraction
echo "🏷️ Version extraction test:"
VERSION=$(cargo metadata --format-version 1 | jq -r '.packages[] | select(.name == "gertjanassies_dev-yew") | .version')
echo "Extracted version: $VERSION"

# Check frontend build
echo "🌐 Testing frontend build..."
cd frontend
trunk build --release
cd ..

echo "✅ Frontend build output:"
ls -la dist/

# Test Docker build
echo "🐳 Testing Docker build..."
docker build -f deploy/Dockerfile -t test-build .

echo "✅ All GitHub Actions validation checks passed!"
