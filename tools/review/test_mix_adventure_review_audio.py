"""Check reconstruction timing, state-driven cues and WAV/report integrity."""

from array import array
import json
from pathlib import Path
import re
import tempfile
import unittest
from unittest.mock import patch
import wave

from mix_adventure_review_audio import (
    CUES, RATE, TRACKS, LAYERS, STREET_EVACUATED_TICK, TRAFFIC_CUES, TRAFFIC_MILESTONES, background_for, load_wavs, probe_video,
    read_telemetry, reconstruct, traffic_gain, write_wav,
)


def row(seconds, stage="Encounter", *, ticks=0, stage_ticks=0, paused=False, awake=False, hit=None, outcome="Active", hp=100, ambience=None, synced_after_skip=False):
    state = {
        "seconds": seconds, "stage": stage, "ticks": ticks, "stage_ticks": stage_ticks,
        "paused": paused, "enemy_awake": awake, "hit": hit, "outcome": outcome,
        "player": {"hp": hp},
    }
    if ambience is not None:
        state["ambience"] = ambience
    if synced_after_skip:
        state["audio_synced_after_skip"] = True
    return state


def street_row(seconds, accident_ticks, *, ambient_ticks=None, **values):
    return row(seconds, awake=True, ambience={
        "ticks": accident_ticks if ambient_ticks is None else ambient_ticks,
        "accident_ticks": accident_ticks,
    }, **values)


def hit(age=0, target="Erratic", blocked=False):
    return {"age": age, "target": target, "blocked": blocked}


def clips(default=0, length=20):
    return {name: array("h", [default] * length) for name in TRACKS + LAYERS + CUES}


class ReconstructionTests(unittest.TestCase):
    def test_traffic_milestones_stay_aligned_with_runtime_constants(self):
        source = (Path(__file__).resolve().parents[2] / "src/adventure/ambient.rs").read_text()
        source += (Path(__file__).resolve().parents[2] / "src/adventure/neighborhood.rs").read_text()
        def constant(name):
            return int(re.search(rf"pub const {name}: u32 = (\d+);", source).group(1))
        self.assertIn("pub const CYCLIST_RUN_TICK: u32 = CYCLIST_BRAKE_TICKS + CYCLIST_DISMOUNT_TICKS;", source)
        self.assertIn("pub const BICYCLE_FALL_TICK: u32 = CYCLIST_RUN_TICK + 16;", source)
        fall_tick = constant("CYCLIST_BRAKE_TICKS") + constant("CYCLIST_DISMOUNT_TICKS") + 16
        timings = [0, constant("DOG_STARTLE_TICK"), constant("CAR_HORN_TICK"), fall_tick,
                   constant("CAR_SKID_TICK"), constant("CAR_IMPACT_TICK"),
                   constant("SHUTTER_START_TICK"), constant("SHUTTER_CLOSED_TICK")]
        self.assertEqual(timings, [tick for tick, _ in TRAFFIC_MILESTONES])
        self.assertIn("pub const STREET_EVACUATED_TICK: u32 = 6 * TICKS_PER_SECOND;", source)
        self.assertEqual(STREET_EVACUATED_TICK, 6 * 60)

    def test_traffic_crossings_play_once_after_uneven_renders(self):
        timeline = [street_row(0, 37), street_row(.1, 40), street_row(.2, 40),
                    street_row(.3, 82), street_row(.4, 116), street_row(.5, 116),
                    street_row(.6, 335)]
        _, report = reconstruct(timeline, clips(), .7, rate=10)
        traffic = [event for event in report["events"] if event["cue"] in TRAFFIC_CUES]
        self.assertEqual([event["cue"] for event in traffic], list(TRAFFIC_CUES))
        self.assertEqual([event["seconds"] for event in traffic], [0, 0, .1, .3, .3, .4, .6, .6])
        self.assertEqual([event["milestone_tick"] for event in traffic], [0, 8, 38, 70, 78, 112, 270, 330])

    def test_late_render_retains_every_crossed_traffic_cue(self):
        _, report = reconstruct([street_row(0, 0), street_row(.1, 338), street_row(.2, 338)], clips(), .3, rate=10)
        traffic = [event for event in report["events"] if event["cue"] in TRAFFIC_CUES]
        self.assertEqual([event["cue"] for event in traffic], list(TRAFFIC_CUES))
        self.assertEqual([event["seconds"] for event in traffic], [0, .1, .1, .1, .1, .1, .1, .1])

    def test_pause_freezes_active_horn_and_defers_pending_skid(self):
        sounds = clips()
        sounds["car_horn"] = array("h", [1000, 2000, 3000, 4000])
        sounds["car_skid"] = array("h", [100, 200])
        timeline = [street_row(0, 38), street_row(.2, 78, paused=True),
                    street_row(.4, 78, paused=True), street_row(.5, 78)]
        pcm, report = reconstruct(timeline, sounds, .8, rate=10)
        self.assertEqual(list(pcm), [300, 600, 0, 0, 0, 930, 1260, 0])
        traffic = [event for event in report["events"] if event["cue"].startswith("car_")]
        self.assertEqual([event["cue"] for event in traffic], ["car_horn", "car_skid"])
        self.assertEqual(traffic[0]["end_seconds"], .7)
        self.assertEqual(traffic[1]["seconds"], .5)

    def test_retry_discards_suspended_crash_and_rearms_traffic(self):
        sounds = clips()
        sounds["car_crash"] = array("h", [1000] * 10)
        timeline = [street_row(0, 112), street_row(.1, 112, paused=True),
                    street_row(.2, 0, paused=True), street_row(.3, 0),
                    street_row(.4, 38)]
        pcm, report = reconstruct(timeline, sounds, .6, rate=10)
        self.assertEqual(list(pcm), [300, 0, 0, 0, 0, 0])
        crashes = [event for event in report["events"] if event["cue"] == "car_crash"]
        self.assertEqual(len(crashes), 1)
        self.assertEqual(crashes[0]["end_seconds"], .2)
        self.assertEqual(crashes[0]["interrupted_by"], "execution_reset")
        horns = [event for event in report["events"] if event["cue"] == "car_horn"]
        self.assertEqual([event["epoch"] for event in horns], [0, 1])
        self.assertEqual([event["seconds"] for event in horns], [0, .4])
        escapes = [event for event in report["events"] if event["cue"] == "traffic_escape"]
        self.assertEqual([event["epoch"] for event in escapes], [0, 1])
        self.assertEqual([event["seconds"] for event in escapes], [0, .3])

    def test_explicit_skip_discards_voices_and_pending_milestones_while_paused(self):
        sounds = clips()
        sounds["car_horn"] = array("h", [1000] * 10)
        timeline = [street_row(0, 38), street_row(.1, 38, paused=True),
                    street_row(.2, 112, paused=True, synced_after_skip=True),
                    street_row(.3, 112)]
        pcm, report = reconstruct(timeline, sounds, .5, rate=10)
        self.assertEqual(list(pcm), [300, 0, 0, 0, 0])
        traffic = [event for event in report["events"] if event["cue"].startswith("car_")]
        self.assertEqual([event["cue"] for event in traffic], ["car_horn"])
        self.assertEqual(traffic[0]["interrupted_by"], "scene_skip")
        self.assertEqual(traffic[0]["end_seconds"], .2)

    def test_skip_into_opening_seeks_score_without_replaying_transition(self):
        sounds = clips()
        sounds["opening"] = array("h", [100, 200, 300, 400, 500])
        timeline = [street_row(0, 38),
                    street_row(.1, 78, stage="Opening", stage_ticks=12, synced_after_skip=True)]
        pcm, report = reconstruct(timeline, sounds, .5, rate=10)
        self.assertEqual(list(pcm), [0, 90, 120, 150, 0])
        self.assertEqual(report["event_counts"], {"transition": 1, "traffic_escape": 1,
                                                "dog_alert": 1, "car_horn": 1})
        self.assertEqual(report["music_seeks"], [{"seconds": .1, "track": "opening", "source_cursor_seconds": .2}])

    def test_aftermath_retains_crash_tail_but_leaving_street_stops_it(self):
        sounds = clips()
        sounds["car_crash"] = array("h", [1000, 2000, 3000, 4000])
        timeline = [street_row(0, 112), street_row(.1, 118, stage="Aftermath"),
                    street_row(.2, 118, stage="Complete")]
        pcm, report = reconstruct(timeline, sounds, .4, rate=10)
        self.assertEqual(list(pcm), [300, 600, 0, 0])
        crashes = [event for event in report["events"] if event["cue"] == "car_crash"]
        self.assertEqual(crashes[0]["end_seconds"], .2)
        self.assertEqual(crashes[0]["interrupted_by"], "street_left")

    def test_legacy_telemetry_without_street_clock_invents_no_traffic(self):
        _, report = reconstruct([row(0, awake=True), row(.2, ticks=400, awake=True)], clips(), .5, rate=10)
        self.assertFalse(any(event["cue"] in TRAFFIC_CUES for event in report["events"]))
        self.assertFalse(report["time_alignment"]["traffic_clock_present_in_all_samples"])

    def test_collective_cues_pause_resume_and_never_loop_after_evacuation(self):
        sounds = clips(length=2)
        sounds["traffic_escape"] = array("h", [1000, 2000, 3000])
        sounds["bicycle_fall"] = array("h", [100, 200])
        timeline = [street_row(0, 0), street_row(.1, 70, paused=True),
                    street_row(.3, 70, paused=True), street_row(.4, 70),
                    street_row(.8, 112), street_row(1.0, 360), street_row(1.5, 3600)]
        pcm, report = reconstruct(timeline, sounds, 2.0, rate=10)
        self.assertEqual(list(pcm[:7]), [300, 0, 0, 0, 630, 960, 0])
        traffic = [event for event in report["events"] if event["cue"] in TRAFFIC_CUES]
        self.assertEqual([event["cue"] for event in traffic], list(TRAFFIC_CUES))
        self.assertEqual([traffic[0]["seconds"], traffic[3]["seconds"]], [0, .4])
        self.assertEqual(traffic[0]["end_seconds"], .6)
        self.assertTrue(all(value == 0 for value in pcm[10:]))

    def test_skip_discards_collective_escape_and_unreached_bicycle_clatter(self):
        sounds = clips()
        sounds["traffic_escape"] = array("h", [1000] * 20)
        timeline = [street_row(0, 0), street_row(.1, 10, paused=True),
                    street_row(.2, 10, stage="Opening", paused=True, synced_after_skip=True),
                    street_row(.3, 10, stage="Opening")]
        pcm, report = reconstruct(timeline, sounds, .6, rate=10)
        traffic = [event for event in report["events"] if event["cue"] in TRAFFIC_CUES]
        self.assertEqual([event["cue"] for event in traffic], ["traffic_escape"])
        self.assertEqual(traffic[0]["interrupted_by"], "scene_skip")
        self.assertEqual(list(pcm), [300, 0, 0, 0, 0, 0])

    def test_background_selection_matches_runtime(self):
        cases = [
            (row(0, "AdaPrologue"), "ada"),
            (row(0, "RustMorning"), "morning_ambience"),
            (row(0), "street_air"),
            (row(0, awake=True), "street_air"),
            (row(0, awake=True, outcome="Defeat"), "street_air"),
            (row(0, awake=True, hp=0), "street_air"),
            (row(0, "Aftermath"), "remorse"),
            (row(0, "Opening"), "opening"),
            (row(0, "Complete"), "remorse"),
        ]
        for state, expected in cases:
            with self.subTest(expected=expected, state=state):
                self.assertEqual(background_for(state), expected)

    def test_calm_traffic_begins_in_arrival_before_combat_ticks_and_freezes_on_pause(self):
        sounds = clips()
        sounds["street_traffic"] = array("h", [100, 200, 300, 400, 500])
        timeline = [row(0, ambience={"ticks": 0, "accident_ticks": None}),
                    row(.2, paused=True, ambience={"ticks": 12, "accident_ticks": None}),
                    row(.5, ambience={"ticks": 12, "accident_ticks": None}),
                    row(.7, ambience={"ticks": 24, "accident_ticks": None})]
        pcm, report = reconstruct(timeline, sounds, .8, rate=10)
        self.assertEqual(list(pcm), [30, 60, 0, 0, 0, 90, 120, 150])
        self.assertEqual(report["events"], [])
        self.assertEqual(report["traffic_layer"]["audible_spans"][1]["source_start_seconds"], .2)

    def test_evacuation_fades_only_traffic_then_never_returns_during_combat_or_defeat(self):
        sounds = clips()
        sounds["street_traffic"] = array("h", [1000])
        sounds["street_air"] = array("h", [100])
        timeline = [street_row(0, 0), street_row(.2, 180),
                    street_row(.4, 360), street_row(.6, 3600),
                    street_row(.8, 3720, outcome="Defeat", hp=0)]
        pcm, report = reconstruct(timeline, sounds, 1.0, rate=10)
        self.assertEqual(list(pcm), [330, 330, 105, 105, 30, 30, 30, 30, 30, 30])
        changes = report["traffic_layer"]["changes"]
        self.assertEqual([change["gain"] for change in changes], [1.0, .25, 0.0])
        self.assertEqual(report["traffic_layer"]["audible_spans"][-1]["end_seconds"], .4)
        self.assertEqual([change["track"] for change in report["tracks"]], ["street_air"])

    def test_retry_restarts_the_departed_traffic_layer_at_its_first_sample(self):
        sounds = clips()
        sounds["street_traffic"] = array("h", [100, 200, 300, 400])
        timeline = [street_row(0, 0), street_row(.2, 360),
                    street_row(.3, 360, paused=True), street_row(.4, 0, paused=True),
                    street_row(.5, 0)]
        pcm, report = reconstruct(timeline, sounds, .7, rate=10)
        self.assertEqual(list(pcm), [30, 60, 0, 0, 0, 30, 60])
        self.assertEqual(report["traffic_layer"]["audible_spans"][-1]["source_start_seconds"], 0)
        self.assertEqual(report["event_counts"]["traffic_escape"], 2)

    def test_camera_skip_seeks_both_street_layers_using_ambient_clock(self):
        sounds = clips()
        sounds["street_air"] = array("h", [100, 200, 300, 400, 500])
        sounds["street_traffic"] = array("h", [1000, 2000, 3000, 4000, 5000])
        timeline = [row(0, ambience={"ticks": 0, "accident_ticks": None}),
                    row(.1, paused=True, stage_ticks=6,
                        ambience={"ticks": 6, "accident_ticks": None}),
                    row(.2, paused=True, synced_after_skip=True, stage_ticks=360,
                        ambience={"ticks": 18, "accident_ticks": None}),
                    row(.3, stage_ticks=360, ambience={"ticks": 18, "accident_ticks": None})]
        pcm, report = reconstruct(timeline, sounds, .5, rate=10)
        self.assertEqual(list(pcm), [330, 0, 0, 1320, 1650])
        self.assertEqual(report["music_seeks"][0]["source_cursor_seconds"], .3)
        self.assertEqual(report["traffic_layer"]["seeks"][0]["source_cursor_seconds"], .3)
        self.assertEqual(report["events"], [])

    def test_shutter_and_dog_are_single_shots_and_paused_closure_is_abandoned_by_skip(self):
        sounds = clips()
        sounds["dog_alert"] = array("h", [100])
        sounds["shutter_roll"] = array("h", [1000, 2000, 3000, 4000])
        sounds["shutter_clack"] = array("h", [10000])
        timeline = [street_row(0, 8), street_row(.1, 270),
                    street_row(.2, 280, paused=True), street_row(.4, 280),
                    street_row(.5, 330, synced_after_skip=True), street_row(.6, 3600)]
        pcm, report = reconstruct(timeline, sounds, .8, rate=10)
        self.assertEqual(list(pcm), [30, 300, 0, 0, 600, 0, 0, 0])
        self.assertEqual(report["event_counts"]["dog_alert"], 1)
        self.assertEqual(report["event_counts"]["shutter_roll"], 1)
        self.assertNotIn("shutter_clack", report["event_counts"])
        shutter = next(event for event in report["events"] if event["cue"] == "shutter_roll")
        self.assertEqual(shutter["interrupted_by"], "scene_skip")

    def test_natural_aftermath_keeps_traffic_fade_and_closes_door_once(self):
        timeline = [street_row(0, 265), street_row(.1, 275, stage="Aftermath"),
                    street_row(.2, 335, stage="Aftermath"), street_row(.3, 360, stage="Aftermath")]
        _, report = reconstruct(timeline, clips(), .5, rate=10)
        doors = [event for event in report["events"] if event["cue"].startswith("shutter_")]
        self.assertEqual([event["cue"] for event in doors], ["shutter_roll", "shutter_clack"])
        self.assertEqual([event["seconds"] for event in doors], [.1, .2])
        self.assertFalse(report["traffic_layer"]["changes"][-1]["active"])

    def test_traffic_gain_never_increases_after_aggro_and_excludes_other_stages(self):
        gains = [traffic_gain(street_row(0, age)) for age in range(420)]
        self.assertEqual(gains[0], 1.0)
        self.assertEqual(gains[360:], [0.0] * 60)
        self.assertTrue(all(a >= b for a, b in zip(gains, gains[1:])))
        for stage in ("AdaPrologue", "RustMorning", "Opening", "Complete"):
            self.assertEqual(traffic_gain(street_row(0, 0, stage=stage)), 0.0)
        self.assertEqual(traffic_gain(row(0, awake=True)), 0.0)

    def test_pause_is_silent_and_resumes_music_and_cue_phase(self):
        sounds = clips()
        sounds["street_air"] = array("h", [100, 200, 300, 400])
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
        sounds["morning_ambience"] = array("h", [500, 600, 700])
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
        self.assertEqual([change["track"] for change in report["tracks"]], ["morning_ambience", "street_air"])

    def test_remorse_music_continues_through_complete(self):
        sounds = clips()
        sounds["remorse"] = array("h", [100, 200, 300, 400, 500])
        pcm, report = reconstruct([row(0, "Aftermath"), row(.2, "Complete")], sounds, .5, rate=10)
        self.assertEqual(list(pcm), [30, 60, 90, 120, 150])
        self.assertEqual(len(report["tracks"]), 1)
        self.assertEqual(len(report["audible_music_spans"]), 1)

    def test_opening_does_not_loop_after_its_authored_ending(self):
        sounds = clips()
        sounds["opening"] = array("h", [100, 200])
        pcm, report = reconstruct([
            row(0, "Opening"), row(.1, "Opening", stage_ticks=1),
        ], sounds, .5, rate=10)
        self.assertEqual(list(pcm), [30, 60, 0, 0, 0])
        self.assertFalse(report["tracks"][0]["looping"])
        self.assertEqual(report["audible_music_spans"][0]["end_seconds"], .2)
        self.assertEqual(report["audible_music_spans"][0]["source_end_seconds"], .2)

    def test_opening_pause_preserves_the_score_position(self):
        sounds = clips()
        sounds["opening"] = array("h", [100, 200, 300, 400])
        pcm, report = reconstruct([
            row(0, "Opening"), row(.2, "Opening", paused=True),
            row(.4, "Opening"),
        ], sounds, .7, rate=10)
        self.assertEqual(list(pcm), [30, 60, 0, 0, 90, 120, 0])
        self.assertEqual(report["audible_music_spans"][1]["source_start_seconds"], .2)

    def test_legacy_telemetry_without_opening_keeps_existing_tracks(self):
        _, report = reconstruct([
            row(0, "AdaPrologue"), row(.1, "RustMorning"), row(.2, "Encounter", awake=True),
            row(.3, "Aftermath"), row(.4, "Complete"),
        ], clips(), .5, rate=10)
        self.assertEqual([change["track"] for change in report["tracks"]], ["ada", "morning_ambience", "street_air", "remorse"])

    def test_complete_leaves_the_opening_track(self):
        _, report = reconstruct([row(0, "Opening"), row(.2, "Complete")], clips(), .5, rate=10)
        self.assertEqual([change["track"] for change in report["tracks"]], ["opening", "remorse"])

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

    def test_invalid_traffic_clock_or_skip_marker_is_rejected(self):
        path = self.directory / "telemetry.jsonl"
        for invalid in ({"ambience": {"ticks": None}},
                        {"ambience": {"accident_ticks": -1}},
                        {"ambience": {"accident_ticks": True}},
                        {"ambience": []},
                        {"audio_synced_after_skip": "yes"}):
            with self.subTest(invalid=invalid):
                path.write_text(json.dumps({**row(0), **invalid}))
                with self.assertRaisesRegex(ValueError, "invalid telemetry"):
                    read_telemetry(path)

    def test_wav_round_trip_preserves_original_pcm(self):
        pcm = array("h", [-32760, -50, 0, 50, 32760])
        for name in TRACKS + LAYERS + CUES:
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
