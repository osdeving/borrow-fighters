"""Regression cases for actual boundary violations and explicit Cargo discovery.

Run locally with Python 3.11+ (python3.13 in the development environment):
python3.13 -m unittest discover -s tools -p 'test_check_domain_boundaries.py'
"""

from pathlib import Path
import tempfile
import unittest

from check_domain_boundaries import check_repository


MANIFEST = '''[package]
name = "borrow-fighters"
version = "0.1.0"
edition = "2024"
default-run = "borrow-fighters"
autotests = false
autoexamples = false

[features]
default = ["fighting"]
fighting = []
adventure = []

[[bin]]
name = "borrow-fighters"
path = "src/main.rs"
required-features = ["fighting"]

[[bin]]
name = "borrow-adventure"
path = "src/bin/borrow-adventure.rs"
required-features = ["adventure"]

[[bin]]
name = "borrow-story"
path = "src/bin/borrow-story.rs"
required-features = ["fighting", "adventure"]

[[test]]
name = "runtime_paths"
path = "tests/runtime_paths.rs"
required-features = ["fighting"]

[[test]]
name = "adventure_flow"
path = "tests/adventure_flow.rs"
required-features = ["adventure"]

[[example]]
name = "capture_adventure"
path = "examples/capture_adventure.rs"
required-features = ["adventure"]
'''

LIB = '''//! Minimal isolated fixture.
#[cfg(feature = "fighting")]
pub mod game;
#[cfg(feature = "adventure")]
pub mod adventure;
#[cfg(all(feature = "adventure", feature = "fighting"))]
pub mod presentation;
pub mod math;
pub mod runtime_paths;
'''


class DomainBoundaryTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        for path, source in {
            "Cargo.toml": MANIFEST,
            "src/lib.rs": LIB,
            "src/main.rs": "use borrow_fighters::game; fn main() {}",
            "src/bin/borrow-adventure.rs": "use borrow_fighters::adventure; fn main() {}",
            "src/bin/borrow-story.rs": "use borrow_fighters::presentation; fn main() { presentation::run(); }",
            "src/presentation.rs": "use crate::{app::App, adventure::app as story, cli, config}; fn run() { story::run(); }",
            "src/game/mod.rs": "use crate::math;",
            "src/adventure/mod.rs": "pub mod combat;",
            "src/adventure/combat.rs": "use crate::math;",
            "src/math/mod.rs": "pub mod vec2;",
            "src/math/vec2.rs": "pub struct Vec2;",
            "src/runtime_paths.rs": "",
            "tests/runtime_paths.rs": "use borrow_fighters::runtime_paths;",
            "tests/adventure_flow.rs": "use borrow_fighters::adventure;",
            "examples/capture_adventure.rs": "use borrow_fighters::adventure; fn main() {}",
        }.items():
            self.write(path, source)

    def write(self, path, source):
        target = self.root / path
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(source)

    def errors(self):
        return check_repository(self.root)

    def assert_violation(self, expected):
        errors = self.errors()
        self.assertTrue(any(expected in error for error in errors), errors)

    def test_complete_isolated_fixture_passes(self):
        self.assertEqual(self.errors(), [])

    def test_nested_owned_imports_and_aliases_pass(self):
        self.write("src/adventure/combat.rs", '''
use crate::{adventure::{combat::{self as encounters, Actor}}, math::{self as geometry, vec2::Vec2}};
use geometry as points;
fn sample() { encounters::tick(); points::vec2::Vec2; }
''')
        self.assertEqual(self.errors(), [])

    def test_nested_fighting_import_in_adventure_is_rejected(self):
        self.write("src/adventure/combat.rs", "use crate::{math::vec2::Vec2, game::{self as matches, world::{World as Scene}}};")
        self.assert_violation("adventure cannot depend on game")

    def test_import_alias_cannot_hide_crossing(self):
        self.write("src/adventure/combat.rs", "use crate::game as hidden; use hidden::{world::World as W}; fn tick() { W::new(); }")
        self.assert_violation("adventure cannot depend on game::world::World")

    def test_root_alias_cannot_import_the_whole_library(self):
        self.write("src/adventure/combat.rs", "use crate as hidden; use hidden::game::World;")
        self.assert_violation("adventure cannot depend on the entire crate")

    def test_fully_qualified_expression_is_checked(self):
        self.write("src/adventure/combat.rs", "fn tick() { crate::game::World::new(); }")
        self.assert_violation("adventure cannot depend on game::World::new")

    def test_super_cannot_escape_adventure(self):
        self.write("src/adventure/combat.rs", "use super::super::{game::{self as old}};")
        self.assert_violation("adventure cannot depend on game")

    def test_inline_test_module_super_import_stays_in_domain(self):
        self.write("src/adventure/combat.rs", "mod tests { use super::*; use super::super::{combat as encounter}; }")
        self.assertEqual(self.errors(), [])

    def test_core_rejects_domain_import(self):
        self.write("src/math/vec2.rs", "use crate::{math, adventure::{combat::Actor as Other}};")
        self.assert_violation("core cannot depend on adventure::combat::Actor")

    def test_core_rejects_root_glob_in_nested_use(self):
        self.write("src/math/vec2.rs", "use crate::{math::{self as shared}, *};")
        self.assert_violation("core cannot depend on *")

    def test_core_rejects_parent_glob_that_reaches_crate_root(self):
        self.write("src/math/mod.rs", "use super::*;")
        self.assert_violation("core cannot depend on *")

    def test_core_unit_test_super_glob_is_safe(self):
        self.write("src/runtime_paths.rs", "mod tests { use super::*; }")
        self.assertEqual(self.errors(), [])

    def test_local_target_super_glob_is_safe(self):
        self.write("examples/capture_adventure.rs", "use borrow_fighters::adventure; mod tests { use super::*; }")
        self.assertEqual(self.errors(), [])

    def test_fighting_cannot_import_adventure(self):
        self.write("src/game/mod.rs", "use crate::{math, adventure::{combat as other}};")
        self.assert_violation("fighting cannot depend on adventure::combat")

    def test_fighting_target_cannot_glob_entire_library(self):
        self.write("src/main.rs", "use borrow_fighters::*; fn main() {}")
        self.assert_violation("fighting cannot depend on *")

    def test_leading_absolute_path_in_use_is_checked(self):
        self.write("tests/runtime_paths.rs", "use ::borrow_fighters::{adventure::{combat as other}};")
        self.assert_violation("fighting cannot depend on adventure::combat")

    def test_comments_doc_attributes_and_strings_do_not_create_imports(self):
        self.write("src/adventure/combat.rs", '''
//! use crate::game; assets/placeholder/old.png
/* use crate::{game}; /* nested use crate::engine; */ */
#[doc = "assets/lore/story.json and crate::game::World"]
fn sample<'a>(text: &'a str) {
    let _ = "use crate::game::World;";
    let _ = r###"use crate::{game::{World}};"###;
    let _ = b"crate::engine::Thing";
    let _ = '}';
    let _ = '\\'';
}
''')
        self.assertEqual(self.errors(), [])

    def test_owned_asset_literals_are_allowed(self):
        self.write("src/adventure/combat.rs", r'''
const ROOT: &str = "assets/adventure";
const FONT: &str = "assets/adventure/fonts/font.ttf";
const WINDOWS: &str = r"assets\adventure\sprites\rust.png";
const ABSOLUTE: &str = "/assets/adventure/art.png";
''')
        self.assertEqual(self.errors(), [])

    def test_fighting_asset_literals_are_rejected_in_all_string_forms(self):
        for literal in ['"assets/placeholder/rust.png"', '"assets/placeholder/my file.png"', 'r#"assets/lore/story.json"#', 'b"assets/candidates/rust.png"', 'r"C:\\assets\\fonts\\font.ttf"']:
            with self.subTest(literal=literal):
                self.write("src/adventure/combat.rs", f"const PATH: &str = {literal};")
                self.assert_violation("adventure asset must be under assets/adventure")

    def test_asset_parent_traversal_cannot_escape_adventure(self):
        self.write("src/adventure/combat.rs", 'const PATH: &str = "assets/adventure/../placeholder/rust.png";')
        self.assert_violation("adventure asset must be under assets/adventure")

    def test_unregistered_test_cannot_silently_disappear(self):
        self.write("tests/new_combat_case.rs", "#[test] fn new_case() {}")
        self.assert_violation("unregistered test target tests/new_combat_case.rs")

    def test_unregistered_example_cannot_silently_disappear(self):
        self.write("examples/new_capture.rs", "fn main() {}")
        self.assert_violation("unregistered example target examples/new_capture.rs")

    def test_wrong_target_feature_is_rejected(self):
        self.write("Cargo.toml", MANIFEST.replace('path = "tests/adventure_flow.rs"\nrequired-features = ["adventure"]', 'path = "tests/adventure_flow.rs"\nrequired-features = ["fighting"]'))
        self.assert_violation("tests/adventure_flow.rs required-features must be ['adventure']")

    def test_deleted_registered_target_is_rejected(self):
        (self.root / "tests/adventure_flow.rs").unlink()
        self.assert_violation("test target missing on disk: tests/adventure_flow.rs")

    def test_duplicate_target_registration_is_rejected(self):
        self.write("Cargo.toml", MANIFEST + '\n[[test]]\nname = "runtime_paths"\npath = "tests/runtime_paths.rs"\nrequired-features = ["fighting"]\n')
        self.assert_violation("duplicate test target runtime_paths")

    def test_autodiscovery_must_stay_disabled(self):
        self.write("Cargo.toml", MANIFEST.replace("autotests = false", "autotests = true"))
        self.assert_violation("package.autotests must be false")

    def test_library_domain_without_cfg_is_rejected(self):
        self.write("src/lib.rs", LIB.replace('#[cfg(feature = "adventure")]\n', ""))
        self.assert_violation("module adventure must have exactly")

    def test_library_core_must_be_available_without_features(self):
        self.write("src/lib.rs", LIB.replace("pub mod math;", '#[cfg(feature = "fighting")]\npub mod math;'))
        self.assert_violation("module math must be ungated core")

    def test_permissive_test_cfg_cannot_bypass_feature_gate(self):
        self.write("src/lib.rs", LIB.replace('cfg(feature = "adventure")', 'cfg(any(test, feature = "adventure"))'))
        self.assert_violation("module adventure must have exactly")

    def test_inline_domain_children_inherit_root_feature(self):
        self.write("src/lib.rs", LIB.replace("pub mod adventure;", "pub mod adventure { pub mod combat {} }"))
        self.assertEqual(self.errors(), [])

    def test_inline_domain_body_still_cannot_cross_boundary(self):
        self.write("src/lib.rs", LIB.replace("pub mod adventure;", "pub mod adventure { pub mod combat { use crate::game as old; } }"))
        self.assert_violation("adventure cannot depend on game")

    def test_domain_feature_cannot_enable_other_domain(self):
        self.write("Cargo.toml", MANIFEST.replace("adventure = []", 'adventure = ["fighting"]'))
        self.assert_violation("adventure must not enable the other domain")

    def test_feature_alias_cannot_enable_other_domain_transitively(self):
        self.write("Cargo.toml", MANIFEST.replace("adventure = []", 'adventure = ["shared_extra"]\nshared_extra = ["fighting"]'))
        self.assert_violation("adventure must not enable the other domain")

    def test_default_feature_alias_cannot_enable_adventure_transitively(self):
        self.write("Cargo.toml", MANIFEST.replace('default = ["fighting"]', 'default = ["fighting", "surprise"]\nsurprise = ["adventure"]'))
        self.assert_violation("default must not enable adventure transitively")

    def test_composition_can_only_use_app_boundaries_and_neutral_utilities(self):
        self.write("src/presentation.rs", '''
use crate::{adventure::app::{self as story, AppExit}, app::App, cli::LaunchOptions, config::WINDOW_WIDTH, math, runtime_paths};
use std::path::Path;
use raylib::prelude::*;
fn run() { story::run(); crate::app::App::new(); }
mod tests { use super::*; }
''')
        self.assertEqual(self.errors(), [])

    def test_composition_cannot_depend_on_gameplay_or_render_internals(self):
        for internal in ("combat::Fighter", "game::World", "characters::CharacterId", "scenes::Scene", "engine::render", "audio::AudioEvent", "lore::Book", "ui::menu_layout", "adventure::combat::Actor", "adventure::engine::render", "adventure::story::Stage", "adventure"):
            with self.subTest(internal=internal):
                self.write("src/presentation.rs", f"use crate::{internal};")
                self.assert_violation(f"presentation cannot depend on {internal}")

    def test_composition_alias_cannot_hide_a_gameplay_import(self):
        self.write("src/presentation.rs", "use crate::adventure as story; use story::combat::Actor; fn tick() { Actor::new(); }")
        self.assert_violation("presentation cannot depend on adventure::combat::Actor")

    def test_composition_cannot_glob_the_crate_or_adventure(self):
        for source in ("use crate::*;", "use crate as all;", "use crate::adventure::*;"):
            with self.subTest(source=source):
                self.write("src/presentation.rs", source)
                self.assert_violation("presentation cannot depend on")

    def test_composition_cannot_access_gameplay_in_an_expression(self):
        self.write("src/presentation.rs", "fn tick() { crate::adventure::combat::Actor::new(); }")
        self.assert_violation("presentation cannot depend on adventure::combat::Actor::new")

    def test_domains_and_core_cannot_depend_on_composition(self):
        for path, owner in (("src/game/mod.rs", "fighting"), ("src/adventure/combat.rs", "adventure"), ("src/math/vec2.rs", "core"), ("src/runtime_paths.rs", "core")):
            with self.subTest(path=path):
                original = (self.root / path).read_text()
                self.write(path, "use crate::presentation as app; fn run() { app::run(); }")
                self.assert_violation(f"{owner} cannot depend on presentation")
                self.write(path, original)

    def test_standalone_binaries_cannot_call_composition(self):
        for path, owner in (("src/main.rs", "fighting"), ("src/bin/borrow-adventure.rs", "adventure")):
            with self.subTest(path=path):
                original = (self.root / path).read_text()
                self.write(path, "use borrow_fighters::presentation; fn main() { presentation::run(); }")
                self.assert_violation(f"{owner} cannot depend on presentation")
                self.write(path, original)

    def test_story_entrypoint_only_calls_composition(self):
        for internal in ("app::App", "adventure::app", "game::World", "adventure::combat::Actor", "*"):
            with self.subTest(internal=internal):
                self.write("src/bin/borrow-story.rs", f"use borrow_fighters::{internal}; fn main() {{}}")
                self.assert_violation(f"presentation entrypoint cannot depend on {internal}")

    def test_another_binary_cannot_gain_composition_privileges(self):
        self.write("Cargo.toml", MANIFEST + '\n[[bin]]\nname = "another-story"\npath = "src/bin/another-story.rs"\nrequired-features = ["fighting", "adventure"]\n')
        self.write("src/bin/another-story.rs", "use borrow_fighters::presentation; fn main() {}")
        self.assert_violation("fighting cannot depend on presentation")
        self.assert_violation("src/bin/another-story.rs required-features must be ['fighting']")

    def test_composition_child_file_cannot_be_classified_as_a_game_domain(self):
        for filename in ("combat.rs", "adventure.rs", "mod.rs"):
            with self.subTest(filename=filename):
                path = f"src/presentation/{filename}"
                self.write(path, "use crate::game::World; use crate::adventure::combat::Actor;")
                self.assert_violation("presentation cannot depend on game::World")
                self.assert_violation("presentation cannot depend on adventure::combat::Actor")
                (self.root / path).unlink()

    def test_composition_cannot_import_source_with_module_or_include(self):
        for path in ("src/presentation.rs", "src/bin/borrow-story.rs"):
            for source in ('mod combat;', '#[path = "game/mod.rs"] mod hidden;', 'include!("game/mod.rs");'):
                with self.subTest(path=path, source=source):
                    self.write(path, source)
                    self.assert_violation("presentation must keep composition explicit")

    def test_composition_must_delegate_asset_loading_to_apps(self):
        for path in ("src/presentation.rs", "src/bin/borrow-story.rs"):
            for asset in ("assets/adventure/audio/ada.wav", "assets/placeholder/rust.png"):
                with self.subTest(path=path, asset=asset):
                    self.write(path, f'const ASSET: &str = "{asset}";')
                    self.assert_violation("must leave asset loading to each app")

    def test_library_composition_requires_exact_both_feature_gate(self):
        for gate in ('cfg(feature = "fighting")', 'cfg(feature = "adventure")', 'cfg(any(feature = "adventure", feature = "fighting"))', 'cfg(any(test, all(feature = "adventure", feature = "fighting")))', 'cfg(all(test, feature = "adventure", feature = "fighting"))'):
            with self.subTest(gate=gate):
                self.write("src/lib.rs", LIB.replace('cfg(all(feature = "adventure", feature = "fighting"))', gate))
                self.assert_violation("module presentation must have exactly")

    def test_library_composition_gate_accepts_either_feature_order(self):
        self.write("src/lib.rs", LIB.replace('all(feature = "adventure", feature = "fighting")', 'all(feature = "fighting", feature = "adventure")'))
        self.assertEqual(self.errors(), [])

    def test_library_composition_cannot_be_ungated_or_missing(self):
        self.write("src/lib.rs", LIB.replace('#[cfg(all(feature = "adventure", feature = "fighting"))]\n', ""))
        self.assert_violation("module presentation must have exactly")
        self.write("src/lib.rs", LIB.replace('#[cfg(all(feature = "adventure", feature = "fighting"))]\npub mod presentation;\n', ""))
        self.assert_violation("missing domain/core module presentation")

    def test_inline_composition_does_not_bypass_restricted_imports(self):
        self.write("src/lib.rs", LIB.replace("pub mod presentation;", "pub mod presentation { use crate::game::World; }"))
        self.assert_violation("presentation cannot depend on game::World")

    def test_story_binary_requires_both_features_only(self):
        for features in ('["adventure"]', '["fighting"]', '[]', '["adventure", "fighting", "extra"]', '["adventure", "fighting", "fighting"]'):
            with self.subTest(features=features):
                self.write("Cargo.toml", MANIFEST.replace('required-features = ["fighting", "adventure"]', f"required-features = {features}"))
                self.assert_violation("src/bin/borrow-story.rs required-features must be ['adventure', 'fighting']")

    def test_story_binary_accepts_either_feature_order(self):
        self.write("Cargo.toml", MANIFEST.replace('required-features = ["fighting", "adventure"]', 'required-features = ["adventure", "fighting"]'))
        self.assertEqual(self.errors(), [])

    def test_story_binary_registration_cannot_be_removed(self):
        self.write("Cargo.toml", MANIFEST.replace('[[bin]]\nname = "borrow-story"\npath = "src/bin/borrow-story.rs"\nrequired-features = ["fighting", "adventure"]\n', ""))
        self.assert_violation("missing borrow-story binary")

    def test_default_run_must_preserve_standalone_fighting(self):
        self.write("Cargo.toml", MANIFEST.replace('default-run = "borrow-fighters"', 'default-run = "borrow-story"'))
        self.assert_violation('package.default-run must remain "borrow-fighters"')


if __name__ == "__main__":
    unittest.main()
