# Modifications to gpui

This directory is a modified copy of [`gpui`](https://crates.io/crates/gpui)
**0.2.2**, published by the Zed Industries team under the Apache License 2.0.
A copy of that licence is in [`LICENSE-APACHE`](LICENSE-APACHE).

As required by section 4(b) of the licence, the files changed relative to the
published 0.2.2 crate are listed here, and each one carries a notice at the top
of the file saying that it was changed.

## What the fork adds

A live Liquid Glass material, as a first-class scene primitive rather than a
post-processing pass. Content behind the surface is read from the render target
in the same frame it is drawn, so anything moving underneath refracts correctly.

On macOS 26 and later the window background can instead be handed to the system
`NSGlassEffectView`, which makes GPUI's existing Metal view the glass content
view — no framebuffer copy, no CPU readback, no second render pass.

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

Nothing else in this directory differs from the published crate. To confirm
that, fetch the original and diff it:

```sh
curl -L -o gpui-0.2.2.crate https://crates.io/api/v1/crates/gpui/0.2.2/download
tar xzf gpui-0.2.2.crate
diff -r gpui-0.2.2 vendor/gpui
```
