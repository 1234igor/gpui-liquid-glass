#!/usr/bin/env python3
"""Build the README image from unmodified GPUI framebuffer crops.

First run capture-background-matrix.sh. Requires Pillow 10.1 or later.
"""
from pathlib import Path
import io

from PIL import Image, ImageCms, ImageDraw, ImageFont, PngImagePlugin

PNG_INFO = PngImagePlugin.PngInfo()
PNG_INFO.add(b"sRGB", b"\x00")
ROOT = Path(__file__).resolve().parents[2]
RAW = ROOT / "validation/captures/raw/backgrounds"
VARIANTS = ["regular", "clear", "regular-tinted", "clear-tinted"]
LABELS = ["Regular", "Clear", "Regular + tint", "Clear + tint"]
BACKGROUNDS = ["harbour", "facade"]
CROP = (700, 1132, 1700, 1460)


def main():
    sheet = Image.new("RGB", (1600, 1280), "#111317")
    draw = ImageDraw.Draw(sheet)
    title = ImageFont.load_default(size=38)
    label = ImageFont.load_default(size=26)
    body = ImageFont.load_default(size=20)
    draw.text((40, 30), "Liquid Glass for GPUI", font=title, fill="white")
    draw.text((40, 84), "Four materials on photographs and fine lines", font=body, fill="#adb6c3")
    for index, variant in enumerate(VARIANTS):
        x = 40 + (index % 2) * 780
        y = 144 + (index // 2) * 556
        draw.text((x, y), LABELS[index], font=label, fill="white")
        for row, background in enumerate(BACKGROUNDS):
            source = RAW / background / variant / "gpui.png"
            with Image.open(source) as frame:
                if frame.size != (2400, 1600):
                    raise ValueError(f"{source}: expected 2400x1600, got {frame.size}")
                rgb = frame.convert("RGB")
                if frame.info.get("icc_profile"):
                    rgb = ImageCms.profileToProfile(rgb,
                        ImageCms.ImageCmsProfile(io.BytesIO(frame.info["icc_profile"])),
                        ImageCms.createProfile("sRGB"), outputMode="RGB")
                crop = rgb.crop(CROP)
            crop = crop.resize((740, 243), Image.Resampling.LANCZOS)
            sheet.paste(crop, (x, y + 42 + row * 249))
    draw.text((40, 1244), "Rendered GPUI captures. Cropped and resized; no added glass effects.", font=body, fill="#adb6c3")
    output = ROOT / "gpui/app.png"
    sheet.save(output, optimize=True, pnginfo=PNG_INFO)
    print(output)


if __name__ == "__main__":
    main()
