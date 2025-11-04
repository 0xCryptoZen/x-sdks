#!/bin/bash
# Setup git hooks for automatic version management

set -e

GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

info() { echo -e "${GREEN}[INFO]${NC} $1"; }
step() { echo -e "${BLUE}[STEP]${NC} $1"; }

step "Setting up git hooks..."

# Configure git to use .githooks directory
git config core.hooksPath .githooks

info "Git hooks configured successfully!"
echo ""
echo "Available hooks:"
echo "  - pre-push: Auto-updates version files when pushing version tags"
echo ""
echo "Usage:"
echo "  1. Create a tag: git tag v2.0.3"
echo "  2. Push the tag: git push origin v2.0.3"
echo "  3. Hook will auto-update VERSION files if needed"
echo ""
