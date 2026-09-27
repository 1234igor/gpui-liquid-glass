# Validation

The painted GPUI material is compared with native SwiftUI glass on four backgrounds. Native full-window glass has a separate reference app.

## Painted material

The stored macOS 27 run contains 16 pairs at 2400 × 1600. The original tool compares decoded RGB channels without converting embedded display profiles. Treat these as historical error scores. Each pair has these thresholds:

| Check | Threshold |
| --- | ---: |
| Full-window score | ≥ 99.5 |
| Glass-crop score | ≥ 95.0 |
| Control-bounds score | ≥ 91.0 |
| Glass-crop pixels with severe channel error | ≤ 3.75% |

The score is `100 × (1 − mean absolute RGB error / 255)`. It is not the percentage of matching pixels. Whole-window scores also include the unchanged background. Use the close-ups to judge visible differences.

Two stored cases fail:

| Case | Result |
| --- | --- |
| Dark city / Regular | 94.42 glass-crop score |
| Harbour / Clear | 4.03% severe error |

The thresholds remain unchanged. See the [material comparison](captures/variants-comparison.png), [background matrix](captures/background-matrix.png), and [metrics](captures/background-metrics.json).

## Capture the matrix

From the repository root:

```sh
ACTIVE_OFFSCREEN_CAPTURE=1 validation/scripts/capture-background-matrix.sh
python3 validation/scripts/compose-evidence.py
python3 validation/scripts/compose-background-evidence.py
```

This builds both apps and captures them in sequence. It requires macOS, Xcode, and Screen Recording permission. Set `DEVELOPER_DIR` to choose Xcode. Close existing reference or GPUI demo windows first.

macOS changes glass when its window is inactive. `ACTIVE_OFFSCREEN_CAPTURE=1` briefly activates the native reference offscreen and then restores focus. GPUI stays in the background.

The capture stops on a failed check. To collect all images despite the known failures, add `ALLOW_FIDELITY_REGRESSION=1`. This records failing results; it does not make them pass.

## README image

With Pillow 10.1 or later installed, run this after collecting the matrix:

```sh
python3 validation/scripts/compose-showcase.py
```

The script takes eight GPUI crops: four materials on the harbour and facade backgrounds. It checks dimensions, converts embedded color profiles to sRGB, crops, resizes, and adds labels. It does not alter the glass. The source images live in `captures/raw/backgrounds/`, which is generated and not checked in.

## Native full-window glass

Build the native reference and run the GPUI example:

```sh
validation/window-glass-reference/build.sh
cd gpui
cargo run --release --example window_glass -- clear
```

Add `benchmark` to either app to make it exit after six seconds. GPUI also reports its render count.

Capture the composited screen rectangle for this path. A window-only capture (`screencapture -l`) removes the background and makes transparent glass look grey. Query the window's actual position before using `screencapture -R`.
