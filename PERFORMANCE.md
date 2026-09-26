# Liquid Glass Performance

Measured on 2026-08-24 on an Apple M4 Pro with macOS 27.0 beta, Xcode 27 beta,
and a ProMotion display. Results are animation-driven render-callback intervals,
not GPU command times, so they can show a refresh-rate cliff but cannot resolve
work that still fits inside the same display interval.

## Live dynamic scene

The normal benchmark animates the backdrop continuously and records 480 frames
after a 60-frame warmup.

| Renderer | Material | Median | p95 | Frames over 12.5 ms |
| --- | --- | ---: | ---: | ---: |
| GPUI | omitted, stock layer | 8.343 ms | 10.002 ms | 19 |
| GPUI | Regular | 8.340 ms | 8.849 ms | 2 |
| SwiftUI | identity | 8.330 ms | 8.398 ms | 0 |
| SwiftUI | Regular | 8.329 ms | 8.393 ms | 0 |

The measured GPUI and SwiftUI penalties are both below one 120 Hz interval's
resolution. That means negligible impact in this scene, not zero GPU cost.

## Advanced dynamic scene

The advanced fixture places 2,400 moving 24 px tiles beneath 24 distinct,
viewport-distributed glass surfaces. This paired run was collected after
coalescing and before the later 33-to-17 tap reduction, so it is a conservative
result for the current shader. ProMotion selected 60 Hz because the underlying
scene was already expensive:

| GPUI advanced scene | Median | p95 | Frames over 12.5 ms |
| --- | ---: | ---: | ---: |
| Material omitted | 16.707 ms | 23.382 ms | 479 |
| 24 distinct surfaces | 16.663 ms | 16.913 ms | 480 |

The glass again adds no resolvable callback-interval penalty relative to its
matched control. ProMotion cadence can switch between runs, so comparisons must
use the paired control rather than treating 8.33 ms versus 16.67 ms as GPU
timings.

Before same-layer effect coalescing, 192 repeated overlapping surfaces dropped
to a 16.694 ms median with 479 slow frames. After coalescing, the same case
measured 8.330 ms median, 8.504 ms p95, and four slow frames. This is the main
measured scaling win and prevents grouped controls from multiplying identical
shader work.

## Native full-window glass

Measured on 2026-08-26 with release builds. A separate GPUI window continuously
animated the harbour scene underneath a static 920 x 620 overlay. Each row is the
median of three 480-frame runs after warmup. The Xcode 27 beta reference uses
SwiftUI `Glass` across the full window; the GPUI window uses the public AppKit
`NSGlassEffectView` that backs the new window API.

| Full-window overlay | Scene median | Scene p95 | Frames over 12.5 ms | Overlay GPUI renders |
| --- | ---: | ---: | ---: | ---: |
| None | 8.334 ms | 9.634 ms | 16 | n/a |
| GPUI Identity | 8.338 ms | 15.834 ms | 44 | 2 |
| GPUI Regular | 8.348 ms | 16.704 ms | 92 | 2 |
| GPUI Clear | 8.344 ms | 15.556 ms | 51 | 2 |
| GPUI Regular Tinted | 8.351 ms | 16.698 ms | 92 | 2 |
| GPUI Clear Tinted | 8.345 ms | 15.575 ms | 59 | 2 |
| SwiftUI Identity | 8.340 ms | 15.453 ms | 43 | n/a |
| SwiftUI Regular | 8.349 ms | 16.646 ms | 77 | n/a |
| SwiftUI Clear | 8.340 ms | 15.282 ms | 40 | n/a |

The GPUI-specific delta is below measurement resolution: Regular is 0.001 ms
faster at the median and 0.058 ms slower at p95 than SwiftUI; Clear is 0.004 ms
slower at the median. Relative to each framework's Identity window, Regular
adds 0.010 ms in GPUI and 0.009 ms in SwiftUI. The 2 GPUI renders are initial
window setup, not backdrop-driven redraws.

The transparent overlay itself changes background-window scheduling, which is
why Identity has more long intervals than the no-overlay run. Regular also
causes the native compositor to use more 60 Hz-length intervals in both GPUI
and SwiftUI. This is native material cost, not a GPUI render-loop regression.

With 2,400 moving tiles underneath, paired GPUI runs stayed at 8.337-8.346 ms
without an overlay, 8.346-8.350 ms with Identity, 8.353-8.391 ms with Regular,
and 8.349-8.352 ms with Clear. Every overlay again rendered exactly twice.

## Renderer work

- The current drawable is copied to a persistent private Metal texture; there
  is no CPU readback.
- Blur is a two-pass 17-tap separable Gaussian. The earlier sparse 5x5 sampler
  was faster but produced visible square cells in motion and was removed.
- Horizontal blur runs once into a persistent intermediate texture; the
  vertical blur and material shading are composed in the glass pass.
- Clear and Clear Tinted batches bypass the blur pass and do not allocate the
  blur texture unless a Regular material appears in the window.
- Snapshot and blur allocations only change when the drawable size changes.
- Exact duplicate materials in one paint layer are submitted once.

## CPU reference path

The offline renderer remains useful for deterministic tests and static assets.
On the same machine, rendering all three interaction states from the 2400 x
1600 harbour source measured:

| Build | Three independent states | Shared batch | Reduction |
| --- | ---: | ---: | ---: |
| Development | 3435.019 ms | 22.242 ms | 99.35% |
| Release | 53.292 ms | 20.215 ms | 62.07% |

The live application does not use this CPU path for dynamic content.

## Native context

Three Xcode Instruments launch samples could not resolve SwiftUI Liquid Glass
cost: the median paired foreground delta was +7.779 ms, smaller than launch
noise and with deltas changing sign. Apple's guidance describes Liquid Glass as
GPU intensive and recommends grouping related effects for rendering
performance.

- [Applying Liquid Glass to custom views](https://developer.apple.com/documentation/SwiftUI/Applying-Liquid-Glass-to-custom-views)
- [Adopting Liquid Glass](https://developer.apple.com/documentation/TechnologyOverviews/adopting-liquid-glass)

## Fidelity guardrails

The optimized live renderer is checked against the Xcode reference over
bright harbour, dark city, saturated prism, and fine facade backgrounds. Across
all four non-identity materials, the 16 enlarged glass crops measure 94.42% to
97.17% similarity, and 99.50% to 99.74% over the whole window.

Two of those sixteen no longer clear their gate: the material was calibrated
against macOS 26 and macOS 27 moved it. The current numbers, the gates and the
two failures are in [`validation/README.md`](validation/README.md).

The Clear captures use the undimmed native `Glass.clear` material. The native
reference is briefly made active in an offscreen window, while the GPUI capture
runs in background mode so the validation does not take over the desktop.

Two captures of the animated Gaussian material changed 91.85% of glass-interior
pixels by more than two channel values, confirming that it tracks the live
backdrop rather than reusing a stale image.
