#!/bin/bash
# ANSI Verification Script
# Compares rich-rust output with Python Rich at the byte level

set -e

echo "=== ANSI Deep Verification ==="
echo ""

# Create output directory
mkdir -p tests/ansi_output

# Step 1: Generate Python reference outputs
echo "Step 1: Generating Python Rich reference outputs..."
python3 tests/ansi_verification.py
echo ""

# Step 2: Generate Rust outputs and compare
echo "Step 2: Generating rich-rust outputs and comparing..."
cargo run --bin ansi_comparison
echo ""

# Step 3: Show hexdump comparison for first test
echo "Step 3: Hexdump comparison (basic_styles)..."
echo "--- Python Rich ---"
xxd tests/ansi_output/python_basic_styles.txt | head -20
echo ""
echo "--- rich-rust ---"
xxd tests/ansi_output/rust_basic_styles.txt | head -20
echo ""

# Step 4: Use diff to show any differences
echo "Step 4: Detailed diff analysis..."
for test in basic_styles colors table panel align; do
    echo "Checking $test..."
    if diff -u tests/ansi_output/python_${test}.txt tests/ansi_output/rust_${test}.txt > /dev/null 2>&1; then
        echo "  ✓ $test matches exactly"
    else
        echo "  ✗ $test has differences:"
        diff -u tests/ansi_output/python_${test}.txt tests/ansi_output/rust_${test}.txt | head -30 || true
    fi
done

echo ""
echo "=== Verification Complete ==="
