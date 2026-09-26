# Third-party notices

This project is [MIT](LICENSE). Everything it bundles is listed here.

## gpui — Apache License 2.0

`vendor/gpui/` is a modified copy of [`gpui`](https://crates.io/crates/gpui)
0.2.2 by Zed Industries, used under the Apache License 2.0. The licence text is
at [`vendor/gpui/LICENSE-APACHE`](vendor/gpui/LICENSE-APACHE), and the changes
— which are the substance of this repository — are listed in
[`vendor/gpui/MODIFICATIONS.md`](vendor/gpui/MODIFICATIONS.md); each changed
file also says so at the top.

No Zed editor source is present. Zed's editor is GPL-licensed and none of it is
copied, vendored or linked here — `gpui` is a separate, Apache-2.0 crate that
Zed publishes.

## Background images

`validation/shared/harbour.png` is a crop of
[*Hyde Street Pier, San Francisco*](https://commons.wikimedia.org/wiki/File:Hyde_Street_Pier._San_Francisco._(37699770496).jpg)
by Bernard Spragg, dedicated to the public domain under
[CC0 1.0](https://creativecommons.org/publicdomain/zero/1.0/). No attribution
is required; it is credited because it is good manners.

The other three backgrounds were generated for this project and are covered by
the MIT licence. See
[`validation/shared/BACKGROUNDS.md`](validation/shared/BACKGROUNDS.md).

## Apple

"Liquid Glass" is Apple's name for its own design language. This is an
independent reimplementation for GPUI. It is not affiliated with or endorsed by
Apple, and it contains none of Apple's code, shaders or assets.

`validation/reference-swiftui/` and `validation/window-glass-reference/` are
small applications written for this project. They call Apple's public
`.glassEffect` and `NSGlassEffectView` APIs so the two can be captured side by
side, in the same way any app would. They are measuring instruments, not copies
of anything.

## Rust crates

Everything in `gpui/Cargo.toml` is MIT, Apache-2.0, or both. `cargo tree` lists
the full graph; `cargo license` or `cargo deny` will re-verify it.

## Fitness for the App Store

Nothing bundled here is copyleft. MIT, Apache-2.0 and CC0 are all compatible
with the App Store's distribution terms — the conflict people remember is with
the GPL, which is not used anywhere in this tree.

Apache-2.0 asks for three things when you ship a binary containing `gpui`:
include the licence, keep the attribution, and state that the files were
changed. This repository does all three, so a build of it can be shipped as-is.
