#!/bin/bash
# Release script for Rust SDK to crates.io

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if we're in the repository root
if [ ! -d "rust-sdk" ]; then
    echo -e "${RED}Error: Must be run from repository root${NC}"
    exit 1
fi

# Function to print colored output
info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

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

# Get current version from workspace Cargo.toml
cd rust-sdk
current_version=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
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

# Run tests
info "Running tests..."
if ! cargo test --all-features --workspace; then
    error "Tests failed. Please fix them before releasing."
    exit 1
fi

# Run clippy
info "Running clippy..."
if ! cargo clippy --all-features --workspace -- -D warnings; then
    error "Clippy warnings found. Please fix them before releasing."
    exit 1
fi

# Check formatting
info "Checking code formatting..."
if ! cargo fmt --all -- --check; then
    error "Code is not formatted. Run 'cargo fmt' to format code."
    exit 1
fi

# Update version in workspace Cargo.toml
info "Updating version in Cargo.toml..."
sed -i.bak "s/^version = \".*\"/version = \"$new_version\"/" Cargo.toml
rm Cargo.toml.bak

# Update Cargo.lock
info "Updating Cargo.lock..."
cargo check --quiet

# Go back to root
cd ..

# Commit version bump
info "Committing version bump..."
git add rust-sdk/Cargo.toml rust-sdk/Cargo.lock
git commit -m "chore(rust-sdk): bump version to $new_version"

# Create and push tag
info "Creating tag rust-v$new_version..."
git tag "rust-v$new_version"

echo ""
info "Ready to publish!"
echo ""
echo "The following will happen:"
echo "  1. Push commits to GitHub"
echo "  2. Push tag rust-v$new_version"
echo "  3. GitHub Actions will automatically:"
echo "     - Run tests and linters"
echo "     - Publish to crates.io"
echo "     - Create GitHub Release"
echo ""

read -p "Proceed with publishing? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    warn "Publishing cancelled. To undo version bump:"
    echo "  git reset --hard HEAD~1"
    echo "  git tag -d rust-v$new_version"
    exit 1
fi

# Push to GitHub
info "Pushing to GitHub..."
git push origin "$current_branch"
git push origin "rust-v$new_version"

echo ""
echo -e "${GREEN}✅ Release initiated!${NC}"
echo ""
echo "Monitor the progress at:"
echo "  https://github.com/0xCryptoZen/x-sdks/actions"
echo ""
echo "Once published, install with:"
echo "  cargo add x-sdk@$new_version"
echo ""
echo "View on crates.io:"
echo "  https://crates.io/crates/x-sdk"
echo ""
