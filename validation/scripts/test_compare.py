#!/usr/bin/env python3
import unittest

from PIL import Image

from compare import artifact_metrics, metrics, validate_capture_geometry, validation_failures


class CompareTests(unittest.TestCase):
    def test_accepts_equal_three_by_two_captures(self) -> None:
        reference = Image.new("RGB", (1200, 800))
        replica = Image.new("RGB", (1200, 800))
        self.assertEqual(validate_capture_geometry(reference, replica), (1200, 800))

    def test_rejects_different_capture_sizes(self) -> None:
        reference = Image.new("RGB", (1200, 800))
        replica = Image.new("RGB", (1199, 800))
        with self.assertRaisesRegex(ValueError, "capture size mismatch"):
            validate_capture_geometry(reference, replica)

    def test_rejects_wrong_capture_aspect_ratio(self) -> None:
        reference = Image.new("RGB", (1200, 799))
        replica = Image.new("RGB", (1200, 799))
        with self.assertRaisesRegex(ValueError, "expected a 3:2 capture"):
            validate_capture_geometry(reference, replica)

    def test_local_artifact_metric_counts_severe_pixels(self) -> None:
        reference = Image.new("RGB", (10, 10), "black")
        replica = reference.copy()
        for y in range(2):
            for x in range(5):
                replica.putpixel((x, y), (255, 255, 255))
        self.assertEqual(artifact_metrics(reference, replica)["pixels_over_64_percent"], 10.0)

    def test_fidelity_thresholds_reject_localized_regression(self) -> None:
        report = {
            "whole_window": {"similarity_percent": 100.0},
            "glass_crop": {"similarity_percent": 100.0},
            "material_interior": {"similarity_percent": 100.0},
            "control_bounds": {"similarity_percent": 100.0},
            "glass_artifacts": {"pixels_over_64_percent": 4.0},
        }
        self.assertEqual(
            validation_failures(report),
            ["glass crop contains too many severe local errors"],
        )


if __name__ == "__main__":
    unittest.main()
