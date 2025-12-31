#!/bin/bash
# Automated verification script for all rich-rust examples

echo "=== Rich-Rust Example Verification ==="
echo ""

EXAMPLES=(
    "prompt_demo"
    "export_demo"
    "pretty_demo"
    "align_demo"
    "padding_demo"
    "bar_demo"
    "theme_demo"
    "highlighter_demo"
    "group_demo"
    "measure_demo"
)

PASSED=0
FAILED=0
TOTAL=${#EXAMPLES[@]}

for example in "${EXAMPLES[@]}"; do
    echo -n "Testing $example... "
    if cargo run --example "$example" --quiet > /dev/null 2>&1; then
        echo "✓ PASS"
        ((PASSED++))
    else
        echo "✗ FAIL"
        ((FAILED++))
    fi
done

echo ""
echo "=== Results ==="
echo "Passed: $PASSED/$TOTAL"
echo "Failed: $FAILED/$TOTAL"

if [ $FAILED -eq 0 ]; then
    echo "✓ All examples working!"
    exit 0
else
    echo "✗ Some examples failed"
    exit 1
fi
