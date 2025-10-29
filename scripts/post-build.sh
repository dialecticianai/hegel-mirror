#!/usr/bin/env bash
# Post-build hook: Install release binary to ~/.cargo/bin after successful release build

set -e

RELEASE_BIN="./target/release/mirror"
INSTALL_DIR="$HOME/.cargo/bin"

# Only run if release binary exists and is newer than installed version
if [ -f "$RELEASE_BIN" ]; then
    # Check if we should install (binary is newer or doesn't exist in install dir)
    if [ ! -f "$INSTALL_DIR/mirror" ] || [ "$RELEASE_BIN" -nt "$INSTALL_DIR/mirror" ]; then
        echo "📦 Installing mirror to $INSTALL_DIR..."
        mkdir -p "$INSTALL_DIR"
        cp "$RELEASE_BIN" "$INSTALL_DIR/mirror"
        chmod +x "$INSTALL_DIR/mirror"
        echo "✅ Installed: $(mirror --version 2>/dev/null || echo 'mirror (version unknown)')"
    fi
fi
