#!/usr/bin/env python3
"""Make the GIF test fixture from the project's credited CC0 photograph.

Requires Pillow. The source, author and license are in IMAGE-LICENSES.md.
"""
from pathlib import Path
from PIL import Image

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]


def main():
    with Image.open(ROOT / "validation/shared/harbour.png") as source:
        source.convert("RGB").resize((600, 400), Image.Resampling.LANCZOS).save(HERE / "app-icon.png", optimize=True)
        still = source.convert("RGB").resize((300, 200), Image.Resampling.LANCZOS)
    # A shared palette avoids color flicker. Each frame pans over the same image.
    palette = still.quantize(colors=128)
    positions = [0, 20, 40, 60, 80, 100, 80, 60, 40, 20]
    frames = [palette.crop((x, 0, x + 200, 200)) for x in positions]
    frames[0].save(HERE / "harbour-pan.gif", save_all=True, append_images=frames[1:],
                   duration=120, loop=0, disposal=2, optimize=False)


if __name__ == "__main__":
    main()
