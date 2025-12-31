#!/bin/bash
# Run all fast-rich examples
# Usage: ./scripts/run_all_examples.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_DIR"

echo "╔════════════════════════════════════════════════════════════╗"
echo "║         Fast-Rich: Running All Examples                   ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Get all example files
EXAMPLES=(
    "hello"
    "markup"
    "console_print"
    "pretty_demo"
    "align_demo"
    "padding_demo"
    "bar_demo"
    "theme_demo"
    "highlighter_demo"
    "group_demo"
    "measure_demo"
    "export_demo"
    "nested_progress_demo"
)

PASSED=0
FAILED=0
TOTAL=${#EXAMPLES[@]}

echo "Running $TOTAL examples..."
echo ""

for example in "${EXAMPLES[@]}"; do
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "▶ Running: $example"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    
    if cargo run --quiet --example "$example" 2>&1; then
        echo ""
        echo "✓ $example - PASSED"
        ((PASSED++))
    else
        echo ""
        echo "✗ $example - FAILED"
        ((FAILED++))
    fi
    
    echo ""
done

echo "╔════════════════════════════════════════════════════════════╗"
echo "║                    Summary                                 ║"
echo "╠════════════════════════════════════════════════════════════╣"
echo "║  Total Examples: $TOTAL"
echo "║  ✓ Passed: $PASSED"
echo "║  ✗ Failed: $FAILED"
echo "╚════════════════════════════════════════════════════════════╝"

if [ $FAILED -eq 0 ]; then
    echo ""
    echo "🎉 All examples passed successfully!"
    exit 0
else
    echo ""
    echo "⚠️  Some examples failed. Please review the output above."
    exit 1
fi
