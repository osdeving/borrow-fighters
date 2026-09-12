"""Checks the reconstructed production mix's pause and explicit reset behavior."""

import json
import unittest
from mix_augusta_review_audio import RATE, SAMPLES, mix


class AugustaAudioTests(unittest.TestCase):
    def setUp(self):
        self.catalog = json.loads((SAMPLES / "catalog.json").read_text())

    def test_pause_suspends_voices_then_resumes_the_same_sample(self):
        continuous, counts = mix([{"seconds": 0, "paused": False, "audio_cues": ["landing"]}], 3000 / RATE, self.catalog)
        paused, paused_counts = mix([
            {"seconds": 0, "paused": False, "audio_cues": ["landing"]},
            {"seconds": 1000 / RATE, "paused": True, "audio_cues": []},
            {"seconds": 2000 / RATE, "paused": False, "audio_cues": []},
        ], 4000 / RATE, self.catalog)
        start, end = 2000, 4000
        self.assertFalse(any(paused[start:end]))
        self.assertEqual(paused[:start] + paused[end:], continuous)
        self.assertEqual(counts, paused_counts)

    def test_reset_discards_abandoned_effect_and_unknown_cue_is_rejected(self):
        empty, _ = mix([{"seconds": 0, "paused": False, "audio_cues": []}], 0.2, self.catalog)
        reset, counts = mix([
            {"seconds": 0, "paused": False, "audio_cues": ["landing"]},
            {"seconds": 0.1, "paused": False, "audio_reset": True, "audio_cues": []},
        ], 0.2, self.catalog)
        self.assertEqual(reset[round(0.1 * RATE) * 2:], empty[round(0.1 * RATE) * 2:])
        self.assertEqual(counts["landing"], 1)
        with self.assertRaises(ValueError):
            mix([{"seconds": 0, "paused": False, "audio_cues": ["missing"]}], 0.2, self.catalog)


if __name__ == "__main__":
    unittest.main()
