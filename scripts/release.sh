#!/bin/bash
# Release script for X SDKs npm package

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if we're in the repository root
if [ ! -f "package.json" ] && [ ! -d "typescript-sdk" ]; then
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

# Get current version
cd typescript-sdk
current_version=$(node -p "require('./package.json').version")
info "Current version: $current_version"

# Ask for version bump type
echo ""
echo "Select version bump type:"
echo "  1) patch  (e.g., $current_version -> $(npm version patch --no-git-tag-version --dry-run 2>/dev/null | grep -o '[0-9]\+\.[0-9]\+\.[0-9]\+'))"
echo "  2) minor  (e.g., $current_version -> $(npm version minor --no-git-tag-version --dry-run 2>/dev/null | grep -o '[0-9]\+\.[0-9]\+\.[0-9]\+'))"
echo "  3) major  (e.g., $current_version -> $(npm version major --no-git-tag-version --dry-run 2>/dev/null | grep -o '[0-9]\+\.[0-9]\+\.[0-9]\+'))"
echo "  4) custom (enter version manually)"
echo ""

read -p "Enter choice (1-4): " choice

case $choice in
    1)
        bump_type="patch"
        ;;
    2)
        bump_type="minor"
        ;;
    3)
        bump_type="major"
        ;;
    4)
        read -p "Enter new version: " custom_version
        bump_type="$custom_version"
        ;;
    *)
        error "Invalid choice"
        exit 1
        ;;
esac

# Run tests
info "Running tests..."
if ! npm test; then
    error "Tests failed. Please fix them before releasing."
    exit 1
fi

# Build
info "Building package..."
if ! npm run build; then
    error "Build failed."
    exit 1
fi

# Bump version
info "Bumping version to $bump_type..."
if [ "$choice" = "4" ]; then
    npm version $custom_version --no-git-tag-version
else
    npm version $bump_type --no-git-tag-version
fi

new_version=$(node -p "require('./package.json').version")
info "New version: $new_version"

# Go back to root
cd ..

# Commit version bump
info "Committing version bump..."
git add typescript-sdk/package.json
git commit -m "chore(typescript-sdk): bump version to $new_version"

# Create and push tag
info "Creating tag v$new_version..."
git tag "v$new_version"

echo ""
info "Ready to publish!"
echo ""
echo "The following will happen:"
echo "  1. Push commits to GitHub"
echo "  2. Push tag v$new_version"
echo "  3. GitHub Actions will automatically:"
echo "     - Run tests"
echo "     - Build the package"
echo "     - Publish to npm"
echo "     - Create GitHub Release"
echo ""

read -p "Proceed with publishing? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    warn "Publishing cancelled. To undo version bump:"
    echo "  git reset --hard HEAD~1"
    echo "  git tag -d v$new_version"
    exit 1
fi

# Push to GitHub
info "Pushing to GitHub..."
git push origin "$current_branch"
git push origin "v$new_version"

echo ""
echo -e "${GREEN}✅ Release initiated!${NC}"
echo ""
echo "Monitor the progress at:"
echo "  https://github.com/0xCryptoZen/x-sdks/actions"
echo ""
echo "Once published, install with:"
echo "  npm install @x-sdks/typescript@$new_version"
echo ""
