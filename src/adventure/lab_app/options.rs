//! Parses a bounded external lab invocation independently from the window.
//!
//! A character manifest is the entry point; alternate actors use the same path.

use crate::runtime_paths::asset_path;
use std::path::PathBuf;

#[derive(Debug)]
pub(super) struct Options {
    pub actor: PathBuf,
    pub enemy: Option<PathBuf>,
    pub clip: String,
    pub phase: f32,
    pub frames: Option<u32>,
    pub hidden: bool,
    pub review: Option<PathBuf>,
    pub validate: bool,
    pub help: bool,
}

impl Options {
    pub fn parse(args: impl IntoIterator<Item = String>) -> Result<Self, String> {
        let mut args = args.into_iter();
        args.next();
        let mut options = Self {
            actor: asset_path("assets/adventure/actors/cpp/character.json"),
            enemy: None,
            clip: "idle".into(),
            phase: 0.0,
            frames: None,
            hidden: false,
            review: None,
            validate: false,
            help: false,
        };
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--actor" => {
                    options.actor =
                        PathBuf::from(args.next().ok_or("--actor requires a manifest path")?)
                }
                "--enemy" => {
                    options.enemy = Some(PathBuf::from(
                        args.next().ok_or("--enemy requires a manifest path")?,
                    ))
                }
                "--clip" => options.clip = args.next().ok_or("--clip requires a clip ID")?,
                "--phase" => {
                    options.phase = args
                        .next()
                        .ok_or("--phase requires 0..1")?
                        .parse()
                        .map_err(|_| "invalid phase")?
                }
                "--frames" => {
                    options.frames = Some(
                        args.next()
                            .ok_or("--frames requires a positive count")?
                            .parse()
                            .map_err(|_| "invalid frame count")?,
                    )
                }
                "--hidden" => options.hidden = true,
                "--review" => {
                    options.review = Some(PathBuf::from(
                        args.next()
                            .ok_or("--review requires a new output directory")?,
                    ))
                }
                "--validate" => options.validate = true,
                "--help" | "-h" => options.help = true,
                _ => return Err(format!("unknown actor lab argument: {arg}")),
            }
        }
        if !options.phase.is_finite() || !(0.0..=1.0).contains(&options.phase) {
            return Err("phase must be finite and within 0..1".into());
        }
        if options.frames.is_some_and(|n| n == 0 || n > 36000) {
            return Err("frame count must be within 1..36000".into());
        }
        if options.review.is_some() && options.frames.is_none() {
            options.frames = Some(1080);
        }
        if options.hidden && options.frames.is_none() && !options.validate && !options.help {
            return Err("--hidden requires --frames, --review or --validate".into());
        }
        Ok(options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn parse(args: &[&str]) -> Result<Options, String> {
        Options::parse(
            std::iter::once("lab")
                .chain(args.iter().copied())
                .map(str::to_owned),
        )
    }
    #[test]
    fn alternate_manifest_and_bounded_review_are_explicit() {
        let o = parse(&[
            "--actor",
            "/tmp/python/character.json",
            "--clip",
            "spin",
            "--phase",
            "0.5",
            "--review",
            "/tmp/review",
            "--hidden",
        ])
        .unwrap();
        assert_eq!(o.actor, PathBuf::from("/tmp/python/character.json"));
        assert_eq!(o.clip, "spin");
        assert_eq!(o.frames, Some(1080));
        assert_eq!(o.phase, 0.5);
    }
    #[test]
    fn malformed_or_unbounded_invocations_fail_before_a_window_opens() {
        for args in [
            &["--phase", "NaN"][..],
            &["--phase", "1.01"],
            &["--frames", "0"],
            &["--frames", "36001"],
            &["--actor"],
            &["--hidden"],
            &["--typo"],
        ] {
            assert!(parse(args).is_err(), "{args:?}");
        }
        assert!(parse(&["--validate", "--hidden"]).is_ok());
    }
}
