# Integrating Liquid Glass into a GPUI application

This repository is a reference implementation and a source dependency. It is
not published on crates.io. Both full-window glass and component glass extend
GPUI itself, so the consuming application must use this repository's vendored
GPUI fork. Mixing it with another `gpui` source creates incompatible Rust types.

## Dependency setup

Point both dependencies at this checkout. Relative paths are shown for sibling
repositories under the same parent directory:

```toml
[dependencies]
gpui = { path = "../gpui-liquid-glass/vendor/gpui", features = ["runtime_shaders"] }
gpui-liquid-glass = { path = "../gpui-liquid-glass/gpui" }
```

Or depend on it over git, pinned to a commit — both entries must point at the
same revision, or Cargo builds two incompatible copies of GPUI's types:

```toml
[dependencies]
gpui = { git = "https://github.com/1234igor/gpui-liquid-glass", rev = "<commit>", features = ["runtime_shaders"] }
gpui-liquid-glass = { git = "https://github.com/1234igor/gpui-liquid-glass", rev = "<commit>" }
```

`gpui-liquid-glass` supplies the material variants, CPU oracle, and examples.
The patched `gpui` supplies the native full-window API and live Metal scene
primitive. An application that only needs full-window glass may omit the
`gpui-liquid-glass` dependency and use the `gpui` API directly.

Keep the fork pinned to a reviewed commit in reproducible builds. If the fork
is copied into another repository, preserve `vendor/gpui` as one unit: the
public types, scene primitive, shaders, and macOS platform code change together.

## Full-window glass

Set the background when opening the window:

```rust
use gpui::{px, WindowBackgroundAppearance, WindowGlassAppearance, WindowOptions};

let options = WindowOptions {
    window_background: WindowBackgroundAppearance::LiquidGlass(
        WindowGlassAppearance::regular().corner_radius(px(0.0)),
    ),
    ..Default::default()
};
```

The available native materials are:

```rust
use gpui::{px, rgb, WindowBackgroundAppearance, WindowGlassAppearance};

let regular = WindowGlassAppearance::regular().corner_radius(px(0.0));
let clear = WindowGlassAppearance::clear().corner_radius(px(0.0));
let regular_tinted = regular.tint(rgb(0xff3847));
let clear_tinted = clear.tint(rgb(0xff3847));
let identity = WindowBackgroundAppearance::Transparent;
```

Wrap the first four values in `WindowBackgroundAppearance::LiquidGlass(...)`.
Change a live window without recreating it:

```rust
window.set_background_appearance(
    WindowBackgroundAppearance::LiquidGlass(clear_tinted),
);
```

Always use `corner_radius(0)` for a full application window. `NSWindow` already
clips the outer silhouette; a nonzero glass-view radius produces a second,
inset corner contour. Do not paint an opaque root-sized GPUI background over the
window, because it will hide the compositor material. Opaque controls and local
surfaces above the glass are fine.

Full-window glass does not require `gpui::enable_liquid_glass()`. That switch is
only needed by the custom component renderer described below.

On macOS 26 or later, GPUI's existing Metal view is installed once as the
`NSGlassEffectView.contentView`. There is no framebuffer copy, CPU readback, or
second GPUI render pass. Desktop changes behind an otherwise idle window are
handled by the system compositor and do not cause GPUI to render. Earlier macOS
versions fall back to system blur; non-macOS targets use a transparent window.

## Component glass

Enable the renderer before creating a glass-bearing window, then paint after
the dynamic backdrop and before sharp foreground content:

```rust
use gpui::{canvas, px, PaintLiquidGlass};
use gpui_liquid_glass::liquid_glass::{GlassVariant, InteractionState};

gpui::enable_liquid_glass();

let glass = canvas(
    |_, _, _| {},
    move |bounds, _, window, _| {
        window.paint_liquid_glass(PaintLiquidGlass {
            bounds,
            corner_radius: px(34.0),
            variant: GlassVariant::Clear.shader_variant(),
            interaction: InteractionState::Rest.energy(),
        });
    },
);
```

The live renderer samples the current drawable on the GPU every frame, so it
works over video, games, scrolling, and other rapidly changing content. Draw
content that should remain sharp after the glass primitive. Reuse identical
overlapping bounds and parameters when possible; the renderer coalesces those
surfaces in a paint layer.

## Integration checklist

1. Use the vendored GPUI fork as the application's only `gpui` dependency.
2. Choose full-window native glass, component Metal glass, or both.
3. Keep full-window glass at zero inner corner radius and the root transparent.
4. Call `enable_liquid_glass()` only when painting component primitives.
5. Test Regular, Clear, both tinted variants, and Identity over light, dark,
   detailed, and moving backgrounds.
6. Benchmark the real dynamic scene against its no-glass baseline. Use the
   commands and interpretation in [`../PERFORMANCE.md`](../PERFORMANCE.md).
7. For pixel comparison, follow [`../validation/README.md`](../validation/README.md);
   window-only screenshots do not contain the composited desktop material.

The runnable sources are `gpui/src/main.rs` for component glass and
`gpui/examples/window_glass.rs` plus `gpui/src/window_glass.rs` for full-window
glass.
