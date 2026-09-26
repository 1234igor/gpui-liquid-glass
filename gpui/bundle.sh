#!/bin/sh
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
cd "$script_dir"

profile=${1:-release}
case "$profile" in
    debug) cargo_args="" ;;
    release) cargo_args="--release" ;;
    *) echo "usage: bundle.sh [debug|release]" >&2; exit 2 ;;
esac

cargo build $cargo_args

bundle="$script_dir/target/$profile/Liquid Glass GPUI.app"
rm -rf "$bundle"
mkdir -p "$bundle/Contents/MacOS"
cp "target/$profile/gpui-liquid-glass" "$bundle/Contents/MacOS/gpui-liquid-glass"
cp Info.plist "$bundle/Contents/Info.plist"
codesign --force --deep --sign - "$bundle" >/dev/null
printf '%s\n' "$bundle"
