#!/usr/bin/env python3
"""Compose the per-material contact sheet from a captured background.

`capture-background-matrix.sh` leaves one directory per background and material
under `captures/raw/backgrounds/`, each holding the SwiftUI capture, the GPUI
capture, the side-by-side crop and the metrics. This turns one background's
worth of that into the two images the README points at:

    captures/glass-comparison.png    the single pair, for the README
    captures/variants-comparison.png every material over that background

Run it after the matrix:

    validation/scripts/capture-background-matrix.sh
    validation/scripts/compose-evidence.py [background] [hero-material]

Both arguments are optional; they default to `harbour` and `clear`.
"""
import json
import sys
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont


VALIDATION = Path(__file__).resolve().parent.parent
RAW = VALIDATION / "captures" / "raw" / "backgrounds"
OUT = VALIDATION / "captures"
VARIANTS = ["regular", "clear", "regular-tinted", "clear-tinted"]
LABELS = {
    "regular": "Regular",
    "clear": "Clear",
    "regular-tinted": "Regular + tint",
    "clear-tinted": "Clear + tint",
}


def labeled(image: Image.Image, title: str, subtitle: str = "") -> Image.Image:
    font = ImageFont.load_default(size=18)
    band_height = 58
    result = Image.new("RGB", (image.width, image.height + band_height), "#111114")
    result.paste(image.convert("RGB"), (0, band_height))
    draw = ImageDraw.Draw(result)
    draw.text((16, 10), title, fill="white", font=font)
    if subtitle:
        draw.text((16, 34), subtitle, fill="#b8b8c0")
    return result


def main() -> None:
    background = sys.argv[1] if len(sys.argv) > 1 else "harbour"
    hero = sys.argv[2] if len(sys.argv) > 2 else "clear"
    root = RAW / background
    if not root.is_dir():
        raise SystemExit(
            f"no captures for {background!r} — run capture-background-matrix.sh first"
        )

    report = {"background": background, "variants": {}}
    rows = []
    for variant in VARIANTS:
        directory = root / variant
        metrics = json.loads((directory / "metrics.json").read_text())
        report["variants"][variant] = metrics
        score = metrics["glass_crop"]["similarity_percent"]
        comparison = Image.open(directory / "glass-comparison.png").convert("RGB")
        comparison.thumbnail((1800, 400), Image.Resampling.LANCZOS)
        rows.append(
            labeled(comparison, LABELS[variant], f"enlarged crop similarity: {score:.4f}%")
        )
        if variant == hero:
            comparison.save(OUT / "glass-comparison.png")

    width = max(row.width for row in rows)
    sheet = Image.new("RGB", (width, sum(row.height for row in rows)), "#111114")
    y = 0
    for row in rows:
        sheet.paste(row, ((width - row.width) // 2, y))
        y += row.height
    sheet.save(OUT / "variants-comparison.png")

    (OUT / "variant-metrics.json").write_text(json.dumps(report, indent=2) + "\n")
    print(f"composed {background} ({hero} as the hero pair)")


if __name__ == "__main__":
    main()
