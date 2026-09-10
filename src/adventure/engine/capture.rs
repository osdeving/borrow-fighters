//! Streams adventure frames to a bounded FFmpeg pipe and exports native screenshots.
//!
//! System: Adventure capture. Both the prologue and chapter reuse one framebuffer
//! buffer; recordings are written incrementally instead of retained in memory.

use super::render;
use raylib::prelude::*;
use std::{
    error::Error,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Child, ChildStdin, Command, Stdio},
};

pub(crate) struct Recorder {
    pub(crate) directory: PathBuf,
    process: Child,
    stdin: Option<ChildStdin>,
    pixels: Vec<u8>,
}

impl Recorder {
    pub(crate) fn new(directory: &Path) -> Result<Self, Box<dyn Error>> {
        fs::create_dir_all(directory)?;
        let mut process = Command::new("ffmpeg")
            .args([
                "-y",
                "-loglevel",
                "error",
                "-f",
                "rawvideo",
                "-pixel_format",
                "rgba",
                "-video_size",
                "1280x720",
                "-framerate",
                "30",
                "-i",
                "-",
                "-vf",
                "vflip",
                "-c:v",
                "libx264",
                "-preset",
                "veryfast",
                "-crf",
                "22",
                "-pix_fmt",
                "yuv420p",
            ])
            .arg(directory.join("adventure-silent.mp4"))
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .spawn()?;
        let stdin = process.stdin.take();
        Ok(Self {
            directory: directory.into(),
            process,
            stdin,
            pixels: Vec::with_capacity((render::WIDTH * render::HEIGHT * 4) as usize),
        })
    }
    pub(crate) fn frame(&mut self, target: &RenderTexture2D) -> Result<(), Box<dyn Error>> {
        self.frames(target, 1)
    }
    pub(crate) fn frames(
        &mut self,
        target: &RenderTexture2D,
        copies: usize,
    ) -> Result<(), Box<dyn Error>> {
        let image = target.texture().load_image()?;
        let colors = image.get_image_data();
        let bytes = rgba_bytes(&colors);
        if bytes.len() != (render::WIDTH * render::HEIGHT * 4) as usize {
            return Err("unexpected framebuffer byte count".into());
        }
        self.pixels.clear();
        self.pixels.extend_from_slice(bytes);
        for _ in 0..copies {
            self.stdin
                .as_mut()
                .ok_or("recording input closed")?
                .write_all(&self.pixels)?;
        }
        Ok(())
    }
    pub(crate) fn finish(mut self) -> Result<(), Box<dyn Error>> {
        self.stdin.take();
        if !self.process.wait()?.success() {
            return Err("FFmpeg failed while recording adventure".into());
        }
        Ok(())
    }
}

fn rgba_bytes(colors: &[raylib::ffi::Color]) -> &[u8] {
    const {
        assert!(std::mem::size_of::<raylib::ffi::Color>() == 4);
    }
    const {
        assert!(std::mem::align_of::<raylib::ffi::Color>() == 1);
    }
    // SAFETY: repr(C) Color is four initialized u8 channels without padding.
    // The argument is an actual slice, not Raylib's owning wrapper. Its byte
    // view remains read-only and cannot outlive that slice's allocation.
    unsafe {
        std::slice::from_raw_parts(colors.as_ptr().cast::<u8>(), std::mem::size_of_val(colors))
    }
}

pub(crate) fn export(target: &RenderTexture2D, path: &Path) -> Result<(), Box<dyn Error>> {
    let mut image = target.texture().load_image()?;
    image.flip_vertical();
    image.export_image(&path.to_string_lossy());
    if !path.is_file() {
        return Err(format!("could not export {}", path.display()).into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::rgba_bytes;
    #[test]
    fn encoder_receives_all_rgba_pixels_in_channel_order() {
        let pixels = [
            raylib::ffi::Color {
                r: 1,
                g: 2,
                b: 3,
                a: 4,
            },
            raylib::ffi::Color {
                r: 5,
                g: 6,
                b: 7,
                a: 8,
            },
        ];
        assert_eq!(rgba_bytes(&pixels), &[1, 2, 3, 4, 5, 6, 7, 8]);
        assert!(rgba_bytes(&[]).is_empty());
    }
}
