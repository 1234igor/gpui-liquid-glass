# Validation

This directory holds the pixel reference for the material: a small SwiftUI
application that calls Apple's public `.glassEffect`, colour-managed comparison
scripts, four shared stress backgrounds, and the checked-in evidence the README
points at.

The point is that the similarity claims are measurements anyone can repeat, not
assertions.

## What is measured

Four materials × four backgrounds = 16 pairs, captured at 2400 × 1600 and
compared in sRGB. Identity is the no-effect sentinel and is not compared against
a native counterpart. Each pair is scored four ways, and every score is a
**gate**, not a report:

| Check | Threshold |
|-------|-----------|
| Whole-window similarity | ≥ 99.5 % |
| Glass-crop similarity | ≥ 95.0 % |
| Control-bounds similarity | ≥ 91.0 % |
| Glass crop, pixels with severe local channel error | ≤ 3.75 % |

The thresholds live at the top of `scripts/compare.py`.

## Current results

Measured on macOS 27:

| Metric | Range |
|--------|-------|
| Glass crop | 94.42 – 97.17 % |
| Whole window | 99.50 – 99.74 % |
| Severe local error | 0.70 – 4.03 % |

Evidence: [`captures/background-matrix.png`](captures/background-matrix.png),
[`captures/glass-comparison.png`](captures/glass-comparison.png),
[`captures/variants-comparison.png`](captures/variants-comparison.png), and the
two metrics files beside them.

### Known failures

Two of the sixteen pairs miss a gate:

```
city-night / regular   glass crop  94.42% < 95.00%
harbour    / clear     severe       4.03% >  3.75%
```

The material was calibrated against macOS 26 and macOS 27 moved it slightly.
The gates have deliberately **not** been loosened to make the run green — a gate
that moves whenever it fails measures nothing. Pass
`ALLOW_FIDELITY_REGRESSION=1` to record the numbers and keep going instead of
stopping at the first miss; that is how the checked-in evidence above was
regenerated.

## Running it

```sh
ACTIVE_OFFSCREEN_CAPTURE=1 validation/scripts/capture-background-matrix.sh
validation/scripts/compose-evidence.py           # glass-comparison, variants-comparison
validation/scripts/compose-background-evidence.py # the four-background matrix
```

The runner builds the native reference with whatever `xcode-select -p` points
at (override with `DEVELOPER_DIR=...`), bundles the GPUI app, verifies the
background assets are byte-identical on both sides, serializes every GUI launch,
requires the exact material and background in each window title, and kills only
the child PIDs it started. It refuses to run while either app is already open.

macOS renders Liquid Glass differently when its window is **inactive**, so the
default automated launch — which activates nothing — is not what the checked
evidence was made with. `ACTIVE_OFFSCREEN_CAPTURE=1` briefly activates the
native reference beyond the visible desktop, keeps GPUI in `background-run`, and
restores the previously frontmost app after every capture. `FOREGROUND_CAPTURE=1`
exists for interactive diagnosis.

App arguments are `regular`, `clear`, `regular-tinted`, `clear-tinted` or
`identity`, optionally followed by `rest`, `hover` or `pressed`, and one of
`harbour`, `city-night`, `prism` or `facade`.

## Full-window validation

`window-glass-reference/` is a small reference for both direct AppKit
`NSGlassEffectView` and full-window SwiftUI `Glass`:

```sh
validation/window-glass-reference/build.sh
cargo run --release --example window_glass -- clear     # the GPUI equivalent
```

Add `benchmark` to either to make it exit after six seconds; GPUI also prints
its render count.

**Do not** validate a translucent full window with `screencapture -l`. A
window-only capture isolates the window from the compositor and flattens its
transparent material to grey. Capture the on-screen rectangle instead, after
querying the owned window position:

```sh
screencapture -x -R296,181,920,620 output.png
```

That composited method is what confirmed live sampling on all four backgrounds,
and a single native corner contour for every variant.

## Layout

| Path | What it is |
|------|-----------|
| `reference-swiftui/` | The SwiftUI application the GPUI output is compared against |
| `window-glass-reference/` | AppKit and SwiftUI references for the full-window path |
| `shared/` | The four stress backgrounds, and where they came from |
| `scripts/` | Capture, comparison and evidence composition |
| `captures/` | Checked-in evidence; `captures/raw/` is generated and ignored |
