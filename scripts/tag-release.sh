#!/bin/bash
# Simplified release script - just provide the version tag
# Usage: ./scripts/tag-release.sh v2.0.3

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

info() { echo -e "${GREEN}[INFO]${NC} $1"; }
error() { echo -e "${RED}[ERROR]${NC} $1"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }

# Check arguments
if [ $# -eq 0 ]; then
    error "Usage: $0 <version-tag>"
    echo "Example: $0 v2.0.3"
    exit 1
fi

TAG=$1

# Validate tag format
if [[ ! $TAG =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    error "Invalid tag format. Must be vX.Y.Z (e.g., v2.0.3)"
    exit 1
fi

# Extract version (remove 'v' prefix)
VERSION=${TAG#v}
info "Version: $VERSION"

# Check for uncommitted changes
if ! git diff-index --quiet HEAD --; then
    error "You have uncommitted changes. Please commit or stash them first."
    git status --short
    exit 1
fi

# Check if tag already exists
if git rev-parse "$TAG" >/dev/null 2>&1; then
    error "Tag $TAG already exists"
    exit 1
fi

# Update VERSION file
info "Updating VERSION file to $VERSION..."
echo "$VERSION" > VERSION

# Sync version to both SDKs
info "Syncing version to TypeScript and Rust SDKs..."
./scripts/sync-version.sh

# Run tests (optional, comment out if you want faster releases)
info "Running TypeScript tests..."
cd typescript-sdk
npm test || { error "TypeScript tests failed"; exit 1; }
cd ..

info "Running Rust tests..."
cd rust-sdk
cargo test --all-features --workspace || { error "Rust tests failed"; exit 1; }
cd ..

# Commit version changes
info "Committing version changes..."
git add VERSION typescript-sdk/package.json rust-sdk/Cargo.toml rust-sdk/Cargo.lock
git commit -m "chore: bump version to $VERSION

- Update VERSION file
- Sync TypeScript SDK to $VERSION
- Sync Rust SDK to $VERSION"

# Create tag
info "Creating tag $TAG..."
git tag "$TAG"

echo ""
info "Ready to publish!"
echo ""
echo "Tag $TAG created on latest commit"
echo ""
read -p "Push to GitHub and trigger release? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    warn "Release cancelled. To undo:"
    echo "  git reset --hard HEAD~1"
    echo "  git tag -d $TAG"
    exit 1
fi

# Push
info "Pushing to GitHub..."
git push origin main
git push origin "$TAG"

echo ""
echo -e "${GREEN}✅ Release initiated!${NC}"
echo ""
echo "Monitor at: https://github.com/0xCryptoZen/x-sdks/actions"
echo ""
echo "Packages:"
echo "  📦 npm: @zen_tools/x-sdk@$VERSION"
echo "  📦 crates.io: x-sdk@$VERSION"
echo ""
