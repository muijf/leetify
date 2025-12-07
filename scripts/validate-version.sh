#!/bin/bash
# Validate that the version in README.md matches Cargo.toml

set -e

# Extract version from Cargo.toml
CARGO_VERSION=$(grep -E '^version\s*=' Cargo.toml | sed -E 's/^version\s*=\s*"([^"]+)".*/\1/')

if [ -z "$CARGO_VERSION" ]; then
    echo "ERROR: Could not extract version from Cargo.toml"
    exit 1
fi

# Check if README.md contains the version
if ! grep -q "leetify = \"$CARGO_VERSION\"" README.md; then
    echo "ERROR: README.md does not contain version '$CARGO_VERSION' from Cargo.toml"
    echo "Expected: leetify = \"$CARGO_VERSION\""
    echo "Please update README.md to match Cargo.toml version"
    exit 1
fi

echo "✓ README.md version matches Cargo.toml version: $CARGO_VERSION"
exit 0