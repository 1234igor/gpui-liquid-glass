#!/bin/sh
set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
DEVELOPER_DIR=${DEVELOPER_DIR:-$(xcode-select -p)}
MODULE_CACHE=${TMPDIR:-/private/tmp}/gpui-window-glass-reference-modules

CLANG_MODULE_CACHE_PATH="$MODULE_CACHE" \
SWIFT_MODULECACHE_PATH="$MODULE_CACHE" \
DEVELOPER_DIR="$DEVELOPER_DIR" \
    xcrun --sdk macosx swiftc \
    -O "$SCRIPT_DIR/main.swift" \
    -o "$SCRIPT_DIR/WindowGlassReference"
