#!/bin/bash

# Git hooks setup script
# Run this script to install git hooks for the project

set -e

echo "Setting up Git hooks for Rust project..."

# Check if we're in a git repository
if [ ! -d ".git" ]; then
    echo "Error: This script must be run from the root of a Git repository"
    exit 1
fi

# Create hooks directory if it doesn't exist
mkdir -p .git/hooks

# Copy pre-commit hook
if [ -f "scripts/hooks/pre-commit" ]; then
    cp scripts/hooks/pre-commit .git/hooks/pre-commit
    chmod +x .git/hooks/pre-commit
    echo "✓ Pre-commit hook installed"
else
    echo "Error: scripts/hooks/pre-commit not found"
    exit 1
fi

echo ""
echo "✅ Git hooks setup complete!"
echo ""
echo "The following hooks are now active:"
echo "  - pre-commit: Automatically formats Rust code with rustfmt"
echo ""
echo "All future commits will automatically format your Rust code." 