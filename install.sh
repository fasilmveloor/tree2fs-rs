#!/bin/bash

set -e

REPO="fasilmveloor/tree2fs-rs"
BINARY_NAME="tree2fs-rs"
INSTALL_DIR="/usr/local/bin"

# Detect OS
OS="$(uname -s)"
case "$OS" in
    Linux)
        OS="linux"
        ;;
    Darwin)
        OS="macos"
        ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

# Detect Architecture
ARCH="$(uname -m)"
case "$ARCH" in
    x86_64)
        ARCH="x86_64"
        ;;
    *)
        echo "Unsupported Architecture: $ARCH"
        exit 1
        ;;
esac

# Determine Target
if [ "$OS" = "linux" ]; then
    TARGET="x86_64-unknown-linux-gnu"
    EXT="tar.gz"
elif [ "$OS" = "macos" ]; then
    TARGET="x86_64-apple-darwin"
    EXT="tar.gz"
fi

ASSET_NAME="${BINARY_NAME}-${TARGET}.${EXT}"
DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/${ASSET_NAME}"

# Create temp directory
TMP_DIR=$(mktemp -d)
cleanup() {
    rm -rf "$TMP_DIR"
}
trap cleanup EXIT

echo "Downloading $BINARY_NAME from $DOWNLOAD_URL..."
curl -L -o "$TMP_DIR/$ASSET_NAME" "$DOWNLOAD_URL"

echo "Extracting..."
tar -xzf "$TMP_DIR/$ASSET_NAME" -C "$TMP_DIR"

# Find the binary inside the extracted directory
# The release workflow creates a directory named "tree2fs-<target>" containing the binary
EXTRACTED_DIR="${BINARY_NAME}-${TARGET}"
BINARY_PATH="$TMP_DIR/$EXTRACTED_DIR/$BINARY_NAME"

if [ ! -f "$BINARY_PATH" ]; then
    echo "Error: Binary not found at $BINARY_PATH"
    exit 1
fi

echo "Installing to $INSTALL_DIR..."
if [ -w "$INSTALL_DIR" ]; then
    mv "$BINARY_PATH" "$INSTALL_DIR/$BINARY_NAME"
else
    sudo mv "$BINARY_PATH" "$INSTALL_DIR/$BINARY_NAME"
fi

echo "$BINARY_NAME installed successfully!"
