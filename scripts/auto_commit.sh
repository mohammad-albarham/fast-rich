#!/bin/bash
set -e

# Usage: ./scripts/auto_commit.sh "commit message"

MSG="$1"
if [ -z "$MSG" ]; then
    echo "Error: Commit message required."
    echo "Usage: ./scripts/auto_commit.sh \"message\""
    exit 1
fi

echo "🔍 Running formatting..."
cargo fmt --all -- --check

echo "Clippy Check..."
cargo clippy -- -D warnings

echo "🧪 Running tests..."
cargo test --workspace

echo "✅ All checks passed. Committing..."
git add .
git commit -m "$MSG"
echo "🚀 Committed successfully: $MSG"
