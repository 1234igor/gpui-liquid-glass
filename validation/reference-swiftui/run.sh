#!/bin/sh
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
cd "$script_dir"

variant=${1:-regular}
profile=${2:-debug}
background=${3:-harbour}
case "$variant" in
    regular|clear|regular-tinted|clear-tinted|identity) ;;
    *) echo "unknown variant: $variant" >&2; exit 2 ;;
esac
case "$profile" in
    debug) configuration=Debug ;;
    release) configuration=Release ;;
    *) echo "usage: run.sh [variant] [debug|release] [background]" >&2; exit 2 ;;
esac
case "$background" in
    harbour|city-night|prism|facade) ;;
    *) echo "unknown background: $background" >&2; exit 2 ;;
esac

app=".build/out/Products/$configuration/Liquid Glass Reference.app"

defaults write com.example.liquid-glass-reference GlassVariant "$variant"
defaults write com.example.liquid-glass-reference GlassBackground "$background"
./bundle.sh "$profile" >/dev/null
open -n "$app" --args "$variant" "$background"
