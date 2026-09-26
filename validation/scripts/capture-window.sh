#!/bin/sh
set -eu

if [ "$#" -ne 2 ]; then
    echo "usage: capture-window.sh <owner-or-title-fragment> <output.png>" >&2
    exit 2
fi

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
module_cache=/private/tmp/gpui-liquid-glass-capture-modules
attempt=0
while :; do
    if window_id=$(
        CLANG_MODULE_CACHE_PATH="$module_cache" \
        SWIFT_MODULECACHE_PATH="$module_cache" \
        xcrun swift "$script_dir/window-id.swift" "$1" 2>/dev/null
    ); then
        break
    fi
    attempt=$((attempt + 1))
    if [ "$attempt" -ge 300 ]; then
        echo "no visible window matched $1 after 30 seconds" >&2
        exit 1
    fi
    sleep 0.1
done
screencapture -x -o -l "$window_id" "$2"
