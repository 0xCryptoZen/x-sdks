#!/bin/bash
# Sync version across TypeScript and Rust SDKs

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

info() { echo -e "${GREEN}[INFO]${NC} $1"; }
error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Read version from VERSION file
if [ ! -f "VERSION" ]; then
    error "VERSION file not found"
    exit 1
fi

VERSION=$(cat VERSION | tr -d '[:space:]')
info "Syncing version: $VERSION"

# Update TypeScript SDK
info "Updating TypeScript SDK (package.json)..."
cd typescript-sdk
npm version $VERSION --no-git-tag-version --allow-same-version
cd ..

# Update Rust SDK
info "Updating Rust SDK (Cargo.toml)..."
cd rust-sdk
sed -i.bak "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml
rm -f Cargo.toml.bak
cargo check --quiet 2>/dev/null || true
cd ..

info "Version synced to $VERSION across all SDKs"
