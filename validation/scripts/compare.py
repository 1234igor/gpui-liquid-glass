#!/usr/bin/env python3
import json
import os
import sys
from pathlib import Path

from PIL import Image, ImageChops, ImageDraw, ImageEnhance, ImageFont, ImageStat

MIN_WHOLE_WINDOW_SIMILARITY = 99.5
MIN_GLASS_CROP_SIMILARITY = 95.0
MIN_CONTROL_SIMILARITY = 91.0
MAX_GLASS_SEVERE_ERROR_PERCENT = 3.75


def label(image: Image.Image, text: str) -> Image.Image:
    band = Image.new("RGB", (image.width, 42), "#111114")
    draw = ImageDraw.Draw(band)
    draw.text((14, 11), text, fill="white", font=ImageFont.load_default(size=16))
    result = Image.new("RGB", (image.width, image.height + band.height), "#111114")
    result.paste(band, (0, 0))
    result.paste(image.convert("RGB"), (0, band.height))
    return result


def side_by_side(images: list[Image.Image]) -> Image.Image:
    result = Image.new("RGB", (sum(i.width for i in images), max(i.height for i in images)))
    x = 0
    for image in images:
        result.paste(image, (x, 0))
        x += image.width
    return result


def metrics(reference: Image.Image, replica: Image.Image) -> dict[str, float]:
    difference = ImageChops.difference(reference.convert("RGB"), replica.convert("RGB"))
    stat = ImageStat.Stat(difference)
    mae = sum(stat.mean) / 3.0
    rms = (sum(value * value for value in stat.rms) / 3.0) ** 0.5
    return {
        "mae_0_255": round(mae, 4),
        "rms_0_255": round(rms, 4),
        "similarity_percent": round((1.0 - mae / 255.0) * 100.0, 4),
    }


def artifact_metrics(reference: Image.Image, replica: Image.Image) -> dict[str, float]:
    difference = ImageChops.difference(reference.convert("RGB"), replica.convert("RGB"))
    severe_pixels = sum(max(pixel) > 64 for pixel in difference.get_flattened_data())
    pixel_count = difference.width * difference.height
    return {
        "pixels_over_64_percent": round(severe_pixels * 100.0 / pixel_count, 4),
    }


def validation_failures(report: dict) -> list[str]:
    checks = [
        (
            report["whole_window"]["similarity_percent"] >= MIN_WHOLE_WINDOW_SIMILARITY,
            f"whole-window similarity is below {MIN_WHOLE_WINDOW_SIMILARITY}%",
        ),
        (
            report["glass_crop"]["similarity_percent"] >= MIN_GLASS_CROP_SIMILARITY,
            f"glass-crop similarity is below {MIN_GLASS_CROP_SIMILARITY}%",
        ),
        (
            report["control_bounds"]["similarity_percent"] >= MIN_CONTROL_SIMILARITY,
            f"control similarity is below {MIN_CONTROL_SIMILARITY}%",
        ),
        (
            report["glass_artifacts"]["pixels_over_64_percent"]
            <= MAX_GLASS_SEVERE_ERROR_PERCENT,
            "glass crop contains too many severe local errors",
        ),
    ]
    return [message for passed, message in checks if not passed]


def validate_capture_geometry(reference: Image.Image, replica: Image.Image) -> tuple[int, int]:
    if reference.size != replica.size:
        raise ValueError(
            f"capture size mismatch: SwiftUI is {reference.size}, GPUI is {replica.size}"
        )
    width, height = reference.size
    if width * 2 != height * 3:
        raise ValueError(f"expected a 3:2 capture, got {width}x{height}")
    return reference.size


def main() -> None:
    if len(sys.argv) != 4:
        raise SystemExit("usage: compare.py <swiftui.png> <gpui.png> <output-dir>")
    reference = Image.open(sys.argv[1]).convert("RGBA")
    replica = Image.open(sys.argv[2]).convert("RGBA")
    try:
        capture_size = validate_capture_geometry(reference, replica)
    except ValueError as error:
        raise SystemExit(str(error)) from error

    output = Path(sys.argv[3])
    output.mkdir(parents=True, exist_ok=True)
    scale = reference.width / 1200.0
    glass_box = tuple(
        round(value * scale)
        for value in (350, 566, 850, 730)
    )
    native_glass = reference.crop(glass_box)
    gpui_glass = replica.crop(glass_box)
    control_box = tuple(round(value * scale) for value in (380, 602, 820, 704))
    native_control = reference.crop(control_box)
    gpui_control = replica.crop(control_box)
    material_box = tuple(round(value * scale) for value in (590, 612, 730, 688))
    native_material = reference.crop(material_box)
    gpui_material = replica.crop(material_box)
    difference = ImageChops.difference(reference.convert("RGB"), replica.convert("RGB"))
    amplified = ImageEnhance.Contrast(difference).enhance(4.0)

    label(reference, "SwiftUI native glass").save(output / "native-labeled.png")
    label(replica, "GPUI recreation").save(output / "gpui-labeled.png")
    side_by_side([
        label(reference, "SwiftUI native glass"),
        label(replica, "GPUI recreation"),
    ]).save(output / "comparison.png")
    side_by_side([
        label(native_glass, "SwiftUI glass crop"),
        label(gpui_glass, "GPUI glass crop"),
    ]).save(output / "glass-comparison.png")
    label(amplified, "Absolute RGB difference, contrast x4").save(output / "difference-x4.png")

    report = {
        "capture_size": capture_size,
        "whole_window": metrics(reference, replica),
        "glass_crop": metrics(native_glass, gpui_glass),
        "material_interior": metrics(native_material, gpui_material),
        "control_bounds": metrics(native_control, gpui_control),
        "glass_artifacts": artifact_metrics(native_glass, gpui_glass),
        "glass_crop_pixels": glass_box,
        "control_bounds_pixels": control_box,
        "material_interior_pixels": material_box,
    }
    (output / "metrics.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))
    failures = validation_failures(report)
    if failures and os.environ.get("ALLOW_FIDELITY_REGRESSION") != "1":
        raise SystemExit("fidelity validation failed: " + "; ".join(failures))


if __name__ == "__main__":
    main()
