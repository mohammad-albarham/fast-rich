#!/bin/bash
set -e

# ANSI Comparison Automation Script

echo "=== 1. Generating Python Reference Outputs ==="
python3 tests/ansi_verification.py

echo -e "\n=== 2. Running Rust ANSI Byte-Level Tests ==="
cargo test --test ansi_byte_tests

echo -e "\n=== 3. Summary ==="
echo "✅ All ANSI byte-level tests passed!"
