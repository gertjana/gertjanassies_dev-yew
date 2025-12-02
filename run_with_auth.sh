#!/bin/bash
# Script to build and run the application with custom authentication credentials
# Usage: ./run_with_auth.sh [username:password]

# Default credentials if none provided
DEFAULT_CREDS="demo:demo123"
CREDS=${1:-$DEFAULT_CREDS}

echo "Building with authentication credentials: $CREDS"
echo "Encoded header will be: Basic $(echo -n "$CREDS" | base64)"

# Build with custom credentials
AUTH_CREDENTIALS="$CREDS" cargo build

# If you want to also run in development mode:
# AUTH_CREDENTIALS="$CREDS" trunk serve --port 8080
