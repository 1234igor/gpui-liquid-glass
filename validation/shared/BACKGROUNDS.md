# Background test assets

Four 2400 × 1600 backgrounds. Each one exercises a different way a glass
material can fail, so a change that looks fine over one of them still has three
more to survive.

| File | What it stresses |
|------|------------------|
| `harbour.png` | Bright daylight, fine rigging lines, readable signage, water reflections |
| `city-night.png` | Near-black media, point highlights, wet reflective surfaces |
| `prism.png` | Saturated wide-gamut colour with sharp intersections |
| `facade.png` | Neutral fine lines, grilles and stairs at hard contrast |

## Provenance and licence

`harbour.png` is a crop of [*Hyde Street Pier, San Francisco*][hyde] by Bernard
Spragg, released under [CC0 1.0][cc0] (public domain dedication). No attribution
is required; it is credited here because it is good manners.

[hyde]: https://commons.wikimedia.org/wiki/File:Hyde_Street_Pier._San_Francisco._(37699770496).jpg
[cc0]: https://creativecommons.org/publicdomain/zero/1.0/

The other three were generated with OpenAI's image model and are covered by this
repository's MIT licence along with everything else here. The prompts are kept
so they can be regenerated or extended.

<details>
<summary>Generation prompts</summary>

**city-night.png**

> Use case: photorealistic-natural. Asset type: full-window background for
> testing a translucent glass UI material. Primary request: a dark nighttime
> modern city scene viewed from a rooftop, with dense small lights, deep black
> and navy areas, bright white highlights, and reflective wet surfaces.
> Composition/framing: landscape 3:2, broad scene with detail distributed
> across the full frame, no dominant centered subject. Lighting/mood: genuinely
> dark exposure with high dynamic range point lights. Constraints: no people,
> no UI, no text, no logos, no watermark; crisp detailed photography; fill the
> complete frame.

**prism.png**

> Use case: stylized-concept. Asset type: full-window background for testing a
> translucent glass UI material. Primary request: a richly saturated macro
> photograph of overlapping translucent colored acrylic sheets and prism-like
> glass, producing crisp areas of cyan, magenta, scarlet, yellow, emerald, and
> deep violet with sharp intersections and fine refraction detail.
> Composition/framing: landscape 3:2, edge-to-edge abstract material study,
> balanced detail across the entire frame. Lighting/mood: bright studio
> backlight, wide color gamut, a mix of luminous highlights and saturated
> shadows. Constraints: no people, no UI, no text, no logos, no watermark;
> photographic rather than vector art; fill the complete frame.

**facade.png**

> Use case: stylized-concept. Asset type: full-window background for testing
> refraction and edge distortion in a translucent glass UI material. Primary
> request: an exacting black-and-white architectural photograph of a modern
> facade with repeating narrow window mullions, stairs, railings, grilles, and
> checker-like shadows, creating dense fine geometric lines across the frame.
> Composition/framing: landscape 3:2, front-facing and edge-to-edge, strong
> pattern continuity behind the center and lower third. Lighting/mood: hard
> midday sunlight, pure whites, deep blacks, crisp midtone concrete.
> Constraints: no people, no UI, no readable text, no logos, no watermark;
> photographic; no blur; fill the complete frame.

</details>
