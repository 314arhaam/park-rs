#!/usr/bin/env bash

set -euo pipefail

REPO="314arhaam/park-rs"
BINARY_NAME="park"

OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux)
        PLATFORM="linux"
        EXT="tar.gz"
        ;;
    Darwin)
        PLATFORM="macos"
        EXT="tar.gz"
        ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

case "$ARCH" in
    x86_64)
        ARCH="x86_64"
        ;;
    arm64|aarch64)
        ARCH="aarch64"
        ;;
    *)
        echo "Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

VERSION=$(curl -fsSL \
    "https://api.github.com/repos/${REPO}/releases/latest" \
    | grep '"tag_name":' \
    | sed -E 's/.*"([^"]+)".*/\1/')

ASSET="${BINARY_NAME}-${PLATFORM}-${ARCH}.${EXT}"

URL="https://github.com/${REPO}/releases/download/${VERSION}/${ASSET}"

TMPDIR=$(mktemp -d)

echo "Downloading ${URL}..."

curl -L "$URL" -o "$TMPDIR/$ASSET"

tar -xzf "$TMPDIR/$ASSET" -C "$TMPDIR"

INSTALL_DIR="/usr/local/bin"

if [ ! -w "$INSTALL_DIR" ]; then
    echo "Using sudo to install into $INSTALL_DIR"
    sudo install -m 755 "$TMPDIR/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
else
    install -m 755 "$TMPDIR/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
fi

rm -rf "$TMPDIR"

echo
echo "Installed successfully!"
echo
echo "Run:"
echo "    $BINARY_NAME --help"