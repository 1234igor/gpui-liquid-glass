#!/bin/sh
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
cd "$script_dir"

profile=${1:-debug}
case "$profile" in
    debug) configuration=Debug ;;
    release) configuration=Release ;;
    *) echo "usage: bundle.sh [debug|release]" >&2; exit 2 ;;
esac

products="$script_dir/.build/out/Products/$configuration"
executable="$products/LiquidGlassReference"
resources="$products/LiquidGlassReference_LiquidGlassReference.bundle"
app="$products/Liquid Glass Reference.app"

if [ ! -x "$executable" ] || [ ! -d "$resources" ]; then
    echo "missing SwiftPM $profile products; build with Xcode beta first" >&2
    exit 1
fi

rm -rf "$app"
mkdir -p "$app/Contents/MacOS" "$app/Contents/Resources"
cp "$executable" "$app/Contents/MacOS/LiquidGlassReference"
cp -R "$resources" "$app/Contents/Resources/"
cp AppInfo.plist "$app/Contents/Info.plist"
codesign --force --deep --sign - "$app" >/dev/null
printf '%s\n' "$app"
