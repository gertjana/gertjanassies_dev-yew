#!/usr/bin/env bash

# Local Development Script
# This script runs the development server locally for testing and development purposes.
# It sets up the necessary environment and starts the server.
set -e

echo "Starting backend server..."
nohup cargo run --bin page-stats-server &

echo "Starting local development server.."
cd frontend
trunk serve
