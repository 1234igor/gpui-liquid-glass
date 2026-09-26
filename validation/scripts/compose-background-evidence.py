#!/usr/bin/env python3
import json
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont


VALIDATION = Path(__file__).resolve().parent.parent
RAW = VALIDATION / "captures" / "raw" / "backgrounds"
OUT = VALIDATION / "captures"
BACKGROUNDS = ["harbour", "city-night", "prism", "facade"]
VARIANTS = ["regular", "clear", "regular-tinted", "clear-tinted"]
BACKGROUND_LABELS = {
    "harbour": "Bright harbour",
    "city-night": "Dark city",
    "prism": "Saturated prism",
    "facade": "Fine facade",
}
VARIANT_LABELS = {
    "regular": "Regular",
    "clear": "Clear",
    "regular-tinted": "Regular + tint",
    "clear-tinted": "Clear + tint",
}


def text(draw: ImageDraw.ImageDraw, xy: tuple[int, int], value: str, fill: str, size: int) -> None:
    draw.text(xy, value, fill=fill, font=ImageFont.load_default(size=size))


def main() -> None:
    cell_width = 470
    image_height = 160
    label_height = 58
    gutter = 12
    row_label_width = 154
    header_height = 52
    width = row_label_width + len(VARIANTS) * (cell_width + gutter) - gutter
    height = header_height + len(BACKGROUNDS) * (label_height + image_height + gutter) - gutter
    sheet = Image.new("RGB", (width, height), "#111114")
    draw = ImageDraw.Draw(sheet)

    for column, variant in enumerate(VARIANTS):
        x = row_label_width + column * (cell_width + gutter)
        text(draw, (x + 12, 17), VARIANT_LABELS[variant], "#ffffff", 18)

    report = {
        "reference": "SwiftUI Glass in Xcode 27 beta on macOS 27.0",
        "capture_size_points": [1200, 800],
        "matrix": {},
    }
    for row, background in enumerate(BACKGROUNDS):
        y = header_height + row * (label_height + image_height + gutter)
        text(draw, (8, y + 64), BACKGROUND_LABELS[background], "#ffffff", 18)
        report["matrix"][background] = {}
        for column, variant in enumerate(VARIANTS):
            root = RAW / background / variant
            metrics = json.loads((root / "metrics.json").read_text())
            report["matrix"][background][variant] = metrics
            score = metrics["glass_crop"]["similarity_percent"]
            crop = Image.open(root / "glass-comparison.png").convert("RGB")
            crop.thumbnail((cell_width, image_height), Image.Resampling.LANCZOS)
            x = row_label_width + column * (cell_width + gutter)
            text(draw, (x + 10, y + 8), f"SwiftUI | GPUI   {score:.2f}%", "#b8b8c0", 15)
            sheet.paste(crop, (x, y + label_height))

    sheet.save(OUT / "background-matrix.png", optimize=True)
    (OUT / "background-metrics.json").write_text(json.dumps(report, indent=2) + "\n")


if __name__ == "__main__":
    main()
