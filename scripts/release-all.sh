#!/bin/bash
# Unified release script for both TypeScript and Rust SDKs

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

info() { echo -e "${GREEN}[INFO]${NC} $1"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
error() { echo -e "${RED}[ERROR]${NC} $1"; }
step() { echo -e "${BLUE}[STEP]${NC} $1"; }

# Check if we're in the repository root
if [ ! -f "VERSION" ]; then
    error "Must be run from repository root (VERSION file not found)"
    exit 1
fi

# Check for uncommitted changes
if ! git diff-index --quiet HEAD --; then
    error "You have uncommitted changes. Please commit or stash them first."
    git status --short
    exit 1
fi

# Check current branch
current_branch=$(git branch --show-current)
if [ "$current_branch" != "main" ]; then
    warn "You are not on the main branch (current: $current_branch)"
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Get current version
current_version=$(cat VERSION | tr -d '[:space:]')
info "Current version: $current_version"

# Ask for version bump type
echo ""
echo "Select version bump type:"
echo "  1) patch  (e.g., $current_version -> $(echo $current_version | awk -F. '{$NF = $NF + 1;} 1' | sed 's/ /./g'))"
echo "  2) minor  (e.g., $current_version -> $(echo $current_version | awk -F. '{$(NF-1) = $(NF-1) + 1; $NF = 0;} 1' | sed 's/ /./g'))"
echo "  3) major  (e.g., $current_version -> $(echo $current_version | awk -F. '{$(NF-2) = $(NF-2) + 1; $(NF-1) = 0; $NF = 0;} 1' | sed 's/ /./g'))"
echo "  4) custom (enter version manually)"
echo ""

read -p "Enter choice (1-4): " choice

case $choice in
    1)
        new_version=$(echo $current_version | awk -F. '{$NF = $NF + 1;} 1' | sed 's/ /./g')
        ;;
    2)
        new_version=$(echo $current_version | awk -F. '{$(NF-1) = $(NF-1) + 1; $NF = 0;} 1' | sed 's/ /./g')
        ;;
    3)
        new_version=$(echo $current_version | awk -F. '{$(NF-2) = $(NF-2) + 1; $(NF-1) = 0; $NF = 0;} 1' | sed 's/ /./g')
        ;;
    4)
        read -p "Enter new version: " new_version
        ;;
    *)
        error "Invalid choice"
        exit 1
        ;;
esac

info "New version: $new_version"
echo ""

# Update VERSION file
step "Updating VERSION file..."
echo "$new_version" > VERSION

# Sync version to both SDKs
step "Syncing version to TypeScript and Rust SDKs..."
./scripts/sync-version.sh

# Run TypeScript tests
step "Running TypeScript SDK tests..."
cd typescript-sdk
if npm test; then
    info "TypeScript tests passed ✓"
else
    error "TypeScript tests failed ✗"
    cd ..
    exit 1
fi
cd ..

# Run Rust tests (skip clippy and rustfmt for now)
step "Running Rust SDK tests..."
cd rust-sdk
if cargo test --all-features --workspace; then
    info "Rust tests passed ✓"
else
    error "Rust tests failed ✗"
    cd ..
    exit 1
fi
cd ..

# Build TypeScript SDK
step "Building TypeScript SDK..."
cd typescript-sdk
if npm run build; then
    info "TypeScript build succeeded ✓"
else
    error "TypeScript build failed ✗"
    cd ..
    exit 1
fi
cd ..

# Commit version changes
step "Committing version changes..."
git add VERSION typescript-sdk/package.json rust-sdk/Cargo.toml rust-sdk/Cargo.lock
git commit -m "chore: bump version to $new_version

- Update VERSION file
- Sync TypeScript SDK (package.json)
- Sync Rust SDK (Cargo.toml)
- Ready for dual SDK release"

# Create tag
step "Creating release tag v$new_version..."
git tag "v$new_version"

echo ""
info "Ready to publish both SDKs!"
echo ""
echo "The following will happen:"
echo "  1. Push commits to GitHub"
echo "  2. Push tag v$new_version"
echo "  3. GitHub Actions will automatically publish:"
echo "     - TypeScript SDK → npm (@zen_tools/x-sdk)"
echo "     - Rust SDK → crates.io (x-sdk)"
echo ""

read -p "Proceed with dual SDK publishing? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    warn "Publishing cancelled. To undo changes:"
    echo "  git reset --hard HEAD~1"
    echo "  git tag -d v$new_version"
    exit 1
fi

# Push to GitHub
step "Pushing to GitHub..."
git push origin "$current_branch"
git push origin "v$new_version"

echo ""
echo -e "${GREEN}✅ Dual SDK release initiated!${NC}"
echo ""
echo "Monitor progress at:"
echo "  https://github.com/0xCryptoZen/x-sdks/actions"
echo ""
echo "Once published:"
echo "  📦 TypeScript: npm install @zen_tools/x-sdk@$new_version"
echo "  📦 Rust: cargo add x-sdk@$new_version"
echo ""
echo "Package URLs:"
echo "  📦 npm: https://www.npmjs.com/package/@zen_tools/x-sdk"
echo "  📦 crates.io: https://crates.io/crates/x-sdk"
echo ""
