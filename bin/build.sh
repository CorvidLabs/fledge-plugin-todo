#!/usr/bin/env bash
set -euo pipefail

if command -v cargo &>/dev/null; then
    echo "  Building fledge-plugin-todo (Rust)..."
    cargo build --release --quiet
    cp -- target/release/fledge-plugin-todo bin/fledge-todo
    echo "  Build complete."
else
    echo "  Cargo not found — using pre-built binary if present."
fi
