#!/bin/sh
set -eu

ROOT=$(CDPATH= cd -- "$(dirname "$0")/../.." && pwd)
VALIDATION="$ROOT/validation"
RAW="$VALIDATION/captures/raw/backgrounds"
SWIFT_PROCESS="LiquidGlassReference"
SWIFT_OWNER="Liquid Glass Reference"
GPUI_PROCESS="gpui-liquid-glass"
GPUI_OWNER="Liquid Glass GPUI"

variants=${MATRIX_VARIANTS:-"regular clear regular-tinted clear-tinted"}
backgrounds=${MATRIX_BACKGROUNDS:-"harbour city-night prism facade"}
foreground_capture=${FOREGROUND_CAPTURE:-0}
active_offscreen_capture=${ACTIVE_OFFSCREEN_CAPTURE:-0}
swift_pid=
gpui_pid=
previous_frontmost=

stop_child() {
    pid=$1
    [ -n "$pid" ] || return 0
    if ! kill -0 "$pid" >/dev/null 2>&1; then
        wait "$pid" 2>/dev/null || true
        return 0
    fi
    kill -TERM "$pid" >/dev/null 2>&1 || true
    attempts=0
    while kill -0 "$pid" >/dev/null 2>&1; do
        attempts=$((attempts + 1))
        if [ "$attempts" -ge 50 ]; then
            kill -KILL "$pid" >/dev/null 2>&1 || true
            break
        fi
        sleep 0.1
    done
    wait "$pid" 2>/dev/null || true
}

require_app_absent() {
    if pgrep -x "$1" >/dev/null 2>&1; then
        echo "$1 is already running; close it before starting the capture matrix" >&2
        return 1
    fi
}

activate_app() {
    osascript -e \
        "tell application \"System Events\" to set frontmost of first process whose name is \"$1\" to true"
}

restore_frontmost() {
    [ -n "$previous_frontmost" ] || return 0
    osascript -e "tell application id \"$previous_frontmost\" to activate" >/dev/null 2>&1 || true
}

cleanup() {
    set +e
    restore_frontmost
    stop_child "$swift_pid"
    stop_child "$gpui_pid"
}

verify_assets() {
    for background in harbour city-night prism facade; do
        shared="$VALIDATION/shared/$background.png"
        native="$VALIDATION/reference-swiftui/Sources/LiquidGlassReference/Resources/$background.png"
        if ! cmp -s "$shared" "$native"; then
            echo "background mismatch: $shared and $native" >&2
            return 1
        fi
    done
}

build_apps() {
    developer_dir=${DEVELOPER_DIR:-$(xcode-select -p)}
    module_cache=${TMPDIR:-/private/tmp}/gpui-liquid-glass-swift-modules
    (
        cd "$VALIDATION/reference-swiftui"
        CLANG_MODULE_CACHE_PATH="$module_cache" \
        SWIFTPM_MODULECACHE_OVERRIDE="$module_cache" \
        DEVELOPER_DIR="$developer_dir" \
        swift build -c debug
    )
    "$VALIDATION/reference-swiftui/bundle.sh" debug >/dev/null
    "$ROOT/gpui/bundle.sh" release
}

trap cleanup EXIT
trap 'exit 129' HUP
trap 'exit 130' INT
trap 'exit 143' TERM
verify_assets
build_apps
require_app_absent "$SWIFT_PROCESS"
require_app_absent "$GPUI_PROCESS"
if [ "$active_offscreen_capture" = "1" ]; then
    previous_frontmost=$(osascript -e \
        'tell application "System Events" to get bundle identifier of first application process whose frontmost is true')
fi

capture_pair() {
    variant=$1
    background=$2
    output="$RAW/$background/$variant"
    mkdir -p "$output"

    case "$variant" in
        regular) variant_title="Regular" ;;
        clear) variant_title="Clear" ;;
        regular-tinted) variant_title="Regular Tinted" ;;
        clear-tinted) variant_title="Clear Tinted" ;;
    esac
    case "$background" in
        harbour) background_title="Harbour" ;;
        city-night) background_title="City Night" ;;
        prism) background_title="Prism" ;;
        facade) background_title="Facade" ;;
    esac

    swift_launch_mode="background-run"
    gpui_launch_mode="background-run"
    if [ "$foreground_capture" = "1" ]; then
        swift_launch_mode=""
        gpui_launch_mode=""
    elif [ "$active_offscreen_capture" = "1" ]; then
        swift_launch_mode="active-offscreen"
    fi
    "$VALIDATION/reference-swiftui/.build/out/Products/Debug/Liquid Glass Reference.app/Contents/MacOS/LiquidGlassReference" \
        "$variant" "$background" $swift_launch_mode >/dev/null 2>&1 &
    swift_pid=$!
    sleep 2
    xcrun swift "$VALIDATION/scripts/move-pointer.swift"
    if [ "$foreground_capture" = "1" ]; then
        activate_app "$SWIFT_PROCESS"
    fi
    sleep 1
    "$VALIDATION/scripts/capture-window.sh" \
        "$SWIFT_OWNER - $variant_title - $background_title" \
        "$output/swiftui.png"
    restore_frontmost
    stop_child "$swift_pid"
    swift_pid=

    "$ROOT/gpui/target/release/$GPUI_OWNER.app/Contents/MacOS/$GPUI_PROCESS" \
        "$variant" rest "$background" $gpui_launch_mode >/dev/null 2>&1 &
    gpui_pid=$!
    sleep 2
    xcrun swift "$VALIDATION/scripts/move-pointer.swift"
    if [ "$foreground_capture" = "1" ]; then
        activate_app "$GPUI_PROCESS"
    fi
    sleep 1
    "$VALIDATION/scripts/capture-window.sh" \
        "Horizon - $variant_title Glass - $background_title" \
        "$output/gpui.png"
    restore_frontmost
    stop_child "$gpui_pid"
    gpui_pid=

    "$VALIDATION/scripts/compare.py" \
        "$output/swiftui.png" \
        "$output/gpui.png" \
        "$output"
}

for background in $backgrounds; do
    for variant in $variants; do
        capture_pair "$variant" "$background"
    done
done
