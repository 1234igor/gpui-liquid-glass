# Modifications to gpui

This directory is a modified copy of [`gpui`](https://crates.io/crates/gpui)
**0.2.2**, published by the Zed Industries team under the Apache License 2.0.
A copy of that licence is in [`LICENSE-APACHE`](LICENSE-APACHE).

As required by section 4(b) of the licence, the files changed relative to the
published 0.2.2 crate are listed here, and each one carries a notice at the top
of the file saying that it was changed.

## What the fork adds

A Liquid Glass scene primitive reads the current render target for refraction
and blur. On macOS 26+, `NSGlassEffectView` can provide the window background
around the Metal content view.

## Documentation changes

`README.md` documents setup for this vendored copy. `docs/contexts.md` has
shorter descriptions and corrected API wording. `docs/key_dispatch.md` corrects
action declarations, handler signatures, and JSON examples.

## Changed files

| File | Change |
|------|--------|
| `build.rs` | Registers the `LiquidGlass` and `LiquidGlassInputIndex` Metal shader symbols |
| `src/platform.rs` | `enable_liquid_glass()`, and the `WindowBackgroundAppearance::LiquidGlass` variant |
| `src/scene.rs` | `Primitive::LiquidGlass`, its batch, ordering and deduplication |
| `src/window.rs` | `PaintLiquidGlass` painting entry point |
| `src/platform/mac/metal_renderer.rs` | The glass draw path: input capture, blit, and draw calls |
| `src/platform/mac/shaders.metal` | The Liquid Glass vertex and fragment shaders |
| `src/platform/mac/window.rs` | `NSGlassEffectView` window background on macOS 26+ |
| `src/platform/blade/blade_renderer.rs` | Ignores the glass batch — the effect is macOS-only |
| `src/platform/windows/directx_renderer.rs` | The same, for the DirectX backend |
| `src/platform/windows/window.rs` | Treats the new background appearance as transparent |
| `src/taffy.rs` | Explicit `f32` literals, to build on current Rust |

## Example image cleanup

The image, image-gallery, GIF-viewer, and opacity examples now use Bernard Spragg's CC0 harbour photograph. `examples/image/app-icon.png` is a copy of that photo; `harbour-pan.gif` and its generator replace the uncredited cat GIF. Remote image URLs use the same credited source instead of random photographs.

`examples/svg/dragon.svg` has an attribution comment; its artwork is unchanged. `LICENSE-LUCIDE` preserves the license notices for the Lucide-derived SVG fixture. See the repository's `IMAGE-LICENSES.md` for authors and reuse terms.

To compare this fork with the published crate:

```sh
curl -L -o gpui-0.2.2.crate https://crates.io/api/v1/crates/gpui/0.2.2/download
tar xzf gpui-0.2.2.crate
diff -r gpui-0.2.2 vendor/gpui
```
