"""Check reconstruction timing, state-driven cues and WAV/report integrity."""

from array import array
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch
import wave

from mix_adventure_review_audio import (
    CUES, RATE, TRACKS, background_for, load_wavs, probe_video,
    read_telemetry, reconstruct, write_wav,
)


def row(seconds, stage="Encounter", *, ticks=0, stage_ticks=0, paused=False, awake=False, hit=None, outcome="Active", hp=100):
    return {
        "seconds": seconds, "stage": stage, "ticks": ticks, "stage_ticks": stage_ticks,
        "paused": paused, "enemy_awake": awake, "hit": hit, "outcome": outcome,
        "player": {"hp": hp},
    }


def hit(age=0, target="Erratic", blocked=False):
    return {"age": age, "target": target, "blocked": blocked}


def clips(default=0, length=20):
    return {name: array("h", [default] * length) for name in TRACKS + CUES}


class ReconstructionTests(unittest.TestCase):
    def test_background_selection_matches_runtime(self):
        cases = [
            (row(0, "AdaPrologue"), "ada"),
            (row(0, "RustMorning"), "morning"),
            (row(0), "morning"),
            (row(0, awake=True), "threat"),
            (row(0, awake=True, outcome="Defeat"), "remorse"),
            (row(0, awake=True, hp=0), "remorse"),
            (row(0, "Aftermath"), "remorse"),
            (row(0, "Complete"), "remorse"),
        ]
        for state, expected in cases:
            with self.subTest(expected=expected, state=state):
                self.assertEqual(background_for(state), expected)

    def test_pause_is_silent_and_resumes_music_and_cue_phase(self):
        sounds = clips()
        sounds["morning"] = array("h", [100, 200, 300, 400])
        sounds["strike"] = array("h", [1000, 2000, 3000, 4000])
        timeline = [
            row(0, ticks=10, hit=hit()),
            row(.2, ticks=10, paused=True, hit=hit()),
            row(.5, ticks=10, hit=hit()),
        ]
        pcm, report = reconstruct(timeline, sounds, .8, rate=10)
        self.assertEqual(list(pcm), [330, 660, 0, 0, 0, 990, 1320, 30])
        self.assertEqual(report["event_counts"], {"strike": 1})
        self.assertEqual(report["events"][0]["end_seconds"], .7)
        self.assertEqual(report["pause_intervals"], [{"start_seconds": .2, "end_seconds": .5}])
        self.assertEqual(report["audible_music_spans"][1]["source_start_seconds"], .2)

    def test_track_change_during_pause_starts_at_zero_only_on_resume(self):
        sounds = clips()
        sounds["ada"] = array("h", [100, 200])
        sounds["morning"] = array("h", [500, 600, 700])
        timeline = [row(0, "AdaPrologue"), row(.1, "RustMorning", paused=True), row(.3, "RustMorning")]
        pcm, report = reconstruct(timeline, sounds, .5, rate=10)
        self.assertEqual(list(pcm), [30, 0, 0, 150, 180])
        self.assertEqual(report["tracks"][1]["seconds"], .1)
        self.assertTrue(report["tracks"][1]["paused"])
        self.assertEqual(report["events"][0]["seconds"], .3)

    def test_contact_is_deduplicated_by_tick_minus_age(self):
        timeline = [
            row(0, ticks=20, hit=hit()),
            row(.1, ticks=21, hit=hit(age=1)),
            row(.2, ticks=22, hit=hit(age=2)),
            row(.3, ticks=40, hit=hit()),
        ]
        _, report = reconstruct(timeline, clips(), .6, rate=10)
        self.assertEqual(report["event_counts"], {"strike": 2})
        self.assertEqual([event["contact_tick"] for event in report["events"]], [20, 40])

    def test_contact_first_seen_at_age_one_is_played(self):
        _, report = reconstruct([row(0, ticks=8, hit=hit(age=1))], clips(), .2, rate=10)
        self.assertEqual(report["events"][0]["contact_tick"], 7)

    def test_old_contact_is_not_invented(self):
        _, report = reconstruct([row(0, ticks=8, hit=hit(age=2))], clips(), .2, rate=10)
        self.assertEqual(report["events"], [])

    def test_retry_allows_same_contact_tick_in_new_epoch(self):
        timeline = [row(0, ticks=5, hit=hit()), row(.1, ticks=20), row(.2, ticks=5, hit=hit())]
        _, report = reconstruct(timeline, clips(), .5, rate=10)
        strikes = [event for event in report["events"] if event["cue"] == "strike"]
        self.assertEqual([event["contact_tick"] for event in strikes], [5, 5])
        self.assertEqual([event["epoch"] for event in strikes], [0, 1])
        self.assertEqual(report["event_counts"]["transition"], 1)

    def test_same_stage_clock_reset_produces_transition(self):
        _, report = reconstruct([
            row(0, "AdaPrologue", stage_ticks=100), row(.2, "AdaPrologue", stage_ticks=0),
        ], clips(), .5, rate=10)
        self.assertEqual(report["events"][0]["reasons"], ["execution_reset"])
        self.assertEqual(len(report["tracks"]), 1)

    def test_stage_change_and_enemy_awake_emit_single_transition(self):
        _, report = reconstruct([
            row(0, "RustMorning"), row(.2, "Encounter", awake=True),
        ], clips(), .5, rate=10)
        self.assertEqual(report["event_counts"], {"transition": 1})
        self.assertEqual(report["events"][0]["reasons"], ["stage_changed", "enemy_noticed_rust"])
        self.assertEqual([change["track"] for change in report["tracks"]], ["morning", "threat"])

    def test_remorse_music_continues_through_complete(self):
        sounds = clips()
        sounds["remorse"] = array("h", [100, 200, 300, 400, 500])
        pcm, report = reconstruct([row(0, "Aftermath"), row(.2, "Complete")], sounds, .5, rate=10)
        self.assertEqual(list(pcm), [30, 60, 90, 120, 150])
        self.assertEqual(len(report["tracks"]), 1)
        self.assertEqual(len(report["audible_music_spans"]), 1)

    def test_block_hurt_and_strike_use_the_correct_clips(self):
        timeline = [row(0, ticks=1, hit=hit()), row(.1, ticks=2, hit=hit(target="Player", blocked=True)), row(.2, ticks=3, hit=hit(target="Player"))]
        _, report = reconstruct(timeline, clips(), .5, rate=10)
        self.assertEqual([event["cue"] for event in report["events"]], ["strike", "block", "hurt"])

    def test_same_cue_restarts_one_voice_instead_of_stacking(self):
        sounds = clips()
        sounds["strike"] = array("h", [100] * 10)
        pcm, report = reconstruct([row(0, ticks=1, hit=hit()), row(.2, ticks=2, hit=hit())], sounds, .5, rate=10)
        self.assertEqual(list(pcm), [30] * 5)
        self.assertTrue(report["events"][0]["interrupted_by_same_cue"])
        self.assertEqual(report["events"][0]["end_seconds"], .2)

    def test_first_telemetry_timestamp_maps_to_first_video_frame(self):
        sounds = clips()
        sounds["ada"] = array("h", [100])
        pcm, report = reconstruct([row(17.5, "AdaPrologue")], sounds, .3, rate=10)
        self.assertEqual(list(pcm), [30, 30, 30])
        self.assertEqual(report["time_alignment"]["first_telemetry_seconds"], 17.5)

    def test_overlapping_cues_are_scaled_without_pcm_clipping(self):
        sounds = clips(default=32767)
        timeline = [
            row(0, ticks=1, hit=hit()),
            row(.1, ticks=2, hit=hit(target="Player", blocked=True)),
            row(.2, ticks=3, hit=hit(target="Player")),
            row(.3, ticks=4, awake=True),
        ]
        pcm, report = reconstruct(timeline, sounds, .5, rate=10)
        self.assertLess(report["master_gain_to_avoid_clipping"], 1.0)
        self.assertLessEqual(max(abs(value) for value in pcm), 32760)
        self.assertEqual(report["clipped_samples"], 0)

    def test_final_pause_stays_silent_to_video_end(self):
        sounds = clips(default=100)
        pcm, report = reconstruct([row(0), row(.2, paused=True)], sounds, .5, rate=10)
        self.assertEqual(list(pcm)[2:], [0, 0, 0])
        self.assertEqual(report["pause_intervals"][0]["end_seconds"], .5)

    def test_report_explicitly_disclaims_device_capture(self):
        _, report = reconstruct([row(0)], clips(), .1, rate=10)
        self.assertIn("reconstructed", report["mode"])
        self.assertIn("not captured from the host", report["limitation"])


class FileTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.directory = Path(self.temporary.name)

    def test_telemetry_round_trip(self):
        path = self.directory / "telemetry.jsonl"
        timeline = [row(.016), row(.032, ticks=1)]
        path.write_text("\n".join(json.dumps(state) for state in timeline))
        self.assertEqual(read_telemetry(path), timeline)

    def test_nonmonotonic_telemetry_is_rejected(self):
        path = self.directory / "telemetry.jsonl"
        path.write_text(json.dumps(row(.2)) + "\n" + json.dumps(row(.1)))
        with self.assertRaisesRegex(ValueError, "monotonic"):
            read_telemetry(path)

    def test_unknown_stage_is_rejected(self):
        path = self.directory / "telemetry.jsonl"
        path.write_text(json.dumps(row(0, "Unknown")))
        with self.assertRaisesRegex(ValueError, "unknown stage"):
            read_telemetry(path)

    def test_wav_round_trip_preserves_original_pcm(self):
        pcm = array("h", [-32760, -50, 0, 50, 32760])
        for name in TRACKS + CUES:
            write_wav(self.directory / f"{name}.wav", pcm)
        sounds, metadata = load_wavs(self.directory)
        self.assertEqual(list(sounds["ada"]), list(pcm))
        self.assertEqual(metadata["strike"]["samples"], 5)
        self.assertEqual(len(metadata["strike"]["sha256"]), 64)
        with wave.open(str(self.directory / "ada.wav")) as sound:
            self.assertEqual((sound.getnchannels(), sound.getsampwidth(), sound.getframerate()), (1, 2, RATE))

    def test_wav_with_wrong_rate_is_rejected(self):
        write_wav(self.directory / "ada.wav", array("h", [0]), rate=8000)
        with self.assertRaisesRegex(ValueError, "expected mono PCM16"):
            load_wavs(self.directory)

    def test_ffprobe_video_duration_is_used(self):
        output = json.dumps({"streams": [{"duration": "2.5", "avg_frame_rate": "30/1", "nb_frames": "75"}], "format": {"duration": "9.0"}})
        with patch("mix_adventure_review_audio.subprocess.check_output", return_value=output) as probe:
            result = probe_video(self.directory / "video.mp4")
        self.assertEqual(result["duration_seconds"], 2.5)
        self.assertEqual(result["fps"], 30)
        self.assertEqual(probe.call_args.args[0][0], "ffprobe")

    def test_empty_video_stream_is_rejected(self):
        with patch("mix_adventure_review_audio.subprocess.check_output", return_value='{"streams": []}'):
            with self.assertRaisesRegex(ValueError, "no video stream"):
                probe_video(self.directory / "unfinished.mp4")

    def test_zero_duration_video_is_rejected(self):
        output = json.dumps({"streams": [{"duration": "0", "avg_frame_rate": "30/1"}]})
        with patch("mix_adventure_review_audio.subprocess.check_output", return_value=output):
            with self.assertRaisesRegex(ValueError, "positive-duration"):
                probe_video(self.directory / "empty.mp4")


if __name__ == "__main__":
    unittest.main()
