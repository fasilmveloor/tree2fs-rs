#!/bin/bash

# Simple test script for tree2fs-rs
# This script creates a sample tree file and tests the tool

set -e

echo "🧪 Testing tree2fs-rs..."
echo ""

# Create a temporary directory for testing
TEST_DIR=$(mktemp -d)
TREE_FILE="$TEST_DIR/test_tree.txt"
OUTPUT_DIR="$TEST_DIR/output"

# Cleanup function
cleanup() {
    echo ""
    echo "🧹 Cleaning up..."
    rm -rf "$TEST_DIR"
}
trap cleanup EXIT

# Create a sample tree file
cat > "$TREE_FILE" << 'EOF'
my-app/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   └── utils/
│       └── helper.rs
├── tests/
│   └── integration_test.rs
├── Cargo.toml
└── README.md
EOF

echo "📄 Created test tree file:"
cat "$TREE_FILE"
echo ""

# Test 1: Dry run
echo "🔍 Test 1: Dry run (preview only)"
cargo run -- "$TREE_FILE" --dry-run --verbose --base-dir "$OUTPUT_DIR"
echo ""

# Test 2: Actual creation
echo "✨ Test 2: Creating filesystem structure"
cargo run -- "$TREE_FILE" --base-dir "$OUTPUT_DIR" --verbose
echo ""

# Verify the structure was created
echo "📂 Verifying created structure:"
if command -v tree &> /dev/null; then
    tree "$OUTPUT_DIR"
else
    find "$OUTPUT_DIR" -type f -o -type d | sort
fi
echo ""

echo "✅ All tests passed!"
