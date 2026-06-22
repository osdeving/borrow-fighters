//! Captures rendered Raylib frames to an MP4 file through FFmpeg.
//!
//! System: Platform capture boundary. This module owns process spawning and
//! render-texture piping; gameplay and scenes only ask it to start, stop, or
//! capture the rendered frame.

use std::{
    fmt, fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Child, ChildStdin, Command, Stdio},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use raylib::prelude::*;

/// Directory where local gameplay captures are written.
pub const CAPTURE_DIR: &str = "captures";

const AUDIO_SOURCE_ENV: &str = "BORROW_FIGHTERS_CAPTURE_AUDIO_SOURCE";
const FFMPEG_PATH_ENV: &str = "BORROW_FIGHTERS_FFMPEG";
const DEFAULT_PULSE_MONITOR: &str = "RDPSink.monitor";
const DEFAULT_FRAMERATE: u32 = 60;
const FFMPEG_STOP_TIMEOUT: Duration = Duration::from_secs(4);
const FFMPEG_STOP_POLL: Duration = Duration::from_millis(50);

/// Window recording state for the current app process.
#[derive(Debug, Default)]
pub struct VideoCapture {
    active: Option<ActiveCapture>,
    last_message: Option<String>,
}

impl VideoCapture {
    /// Starts recording the game framebuffer if no capture is active.
    pub fn start(&mut self, width: u32, height: u32) -> Result<&Path, VideoCaptureError> {
        if self.active.is_some() {
            return self
                .output_path()
                .ok_or(VideoCaptureError::AlreadyRecording);
        }

        if width == 0 || height == 0 {
            return Err(VideoCaptureError::InvalidFrameSize { width, height });
        }

        fs::create_dir_all(CAPTURE_DIR).map_err(VideoCaptureError::CreateDirectory)?;
        let output_path = next_capture_path();
        let audio_source =
            std::env::var(AUDIO_SOURCE_ENV).unwrap_or_else(|_| DEFAULT_PULSE_MONITOR.to_owned());
        let mut child = ffmpeg_command(width, height, &audio_source, &output_path)
            .spawn()
            .map_err(VideoCaptureError::SpawnFfmpeg)?;
        let stdin = child.stdin.take().ok_or(VideoCaptureError::FfmpegStdin)?;

        self.last_message = Some(format!("Gravando em {}", output_path.display()));
        self.active = Some(ActiveCapture {
            child,
            stdin: Some(stdin),
            output_path,
        });

        Ok(self.output_path().expect("active capture has output path"))
    }

    /// Captures the current render texture into the active recording.
    pub fn capture_render_texture(
        &mut self,
        target: &RenderTexture2D,
    ) -> Result<(), VideoCaptureError> {
        let Some(active) = self.active.as_mut() else {
            return Ok(());
        };

        let image = target
            .texture()
            .load_image()
            .map_err(|error| VideoCaptureError::ReadFrame(error.to_string()))?;
        let pixels = image.get_image_data_u8(true);
        let Some(stdin) = active.stdin.as_mut() else {
            return Err(VideoCaptureError::FfmpegStdin);
        };
        stdin
            .write_all(&pixels)
            .map_err(VideoCaptureError::WriteFrame)?;
        Ok(())
    }

    /// Stops the active recording, returning the final path when one existed.
    pub fn stop(&mut self) -> Result<Option<PathBuf>, VideoCaptureError> {
        let Some(mut active) = self.active.take() else {
            self.last_message = Some("Nenhuma gravacao ativa.".to_owned());
            return Ok(None);
        };

        drop(active.stdin.take());
        wait_for_ffmpeg(&mut active.child)?;

        self.last_message = Some(format!(
            "Gravacao salva em {}",
            active.output_path.display()
        ));
        Ok(Some(active.output_path))
    }

    /// Polls the FFmpeg process so unexpected exits are reflected in the UI.
    pub fn update(&mut self) -> Result<(), VideoCaptureError> {
        let Some(active) = self.active.as_mut() else {
            return Ok(());
        };

        let Some(status) = active
            .child
            .try_wait()
            .map_err(VideoCaptureError::WaitFfmpeg)?
        else {
            return Ok(());
        };

        let output_path = active.output_path.clone();
        self.active = None;
        if status.success() {
            self.last_message = Some(format!("Gravacao salva em {}", output_path.display()));
        } else {
            self.last_message = Some(format!(
                "Falha na gravacao: ffmpeg encerrou com status {status}"
            ));
        }
        Ok(())
    }

    /// Returns true when an FFmpeg recording process is active.
    pub fn is_recording(&self) -> bool {
        self.active.is_some()
    }

    /// Returns the current output path while recording.
    pub fn output_path(&self) -> Option<&Path> {
        self.active
            .as_ref()
            .map(|active| active.output_path.as_path())
    }

    /// Last user-facing capture status.
    pub fn last_message(&self) -> Option<&str> {
        self.last_message.as_deref()
    }

    /// Stores a failure message that render code can show without owning errors.
    pub fn set_error_message(&mut self, error: &VideoCaptureError) {
        self.last_message = Some(format!("Falha na gravacao: {error}"));
    }
}

impl Drop for VideoCapture {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

#[derive(Debug)]
struct ActiveCapture {
    child: Child,
    stdin: Option<ChildStdin>,
    output_path: PathBuf,
}

/// User-facing failures for local capture.
#[derive(Debug)]
pub enum VideoCaptureError {
    AlreadyRecording,
    InvalidFrameSize { width: u32, height: u32 },
    CreateDirectory(std::io::Error),
    SpawnFfmpeg(std::io::Error),
    FfmpegStdin,
    ReadFrame(String),
    WriteFrame(std::io::Error),
    WaitFfmpeg(std::io::Error),
    KillFfmpeg(std::io::Error),
}

impl fmt::Display for VideoCaptureError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyRecording => write!(formatter, "ja existe uma gravacao ativa"),
            Self::InvalidFrameSize { width, height } => {
                write!(formatter, "tamanho de frame invalido: {width}x{height}")
            }
            Self::CreateDirectory(error) => {
                write!(formatter, "nao foi possivel criar {CAPTURE_DIR}: {error}")
            }
            Self::SpawnFfmpeg(error) => write!(formatter, "ffmpeg nao iniciou: {error}"),
            Self::FfmpegStdin => write!(formatter, "ffmpeg nao abriu stdin para video bruto"),
            Self::ReadFrame(error) => write!(formatter, "frame nao foi lido da textura: {error}"),
            Self::WriteFrame(error) => {
                write!(formatter, "frame nao foi enviado ao ffmpeg: {error}")
            }
            Self::WaitFfmpeg(error) => write!(formatter, "ffmpeg nao encerrou: {error}"),
            Self::KillFfmpeg(error) => write!(formatter, "ffmpeg nao pode ser finalizado: {error}"),
        }
    }
}

impl std::error::Error for VideoCaptureError {}

fn ffmpeg_command(width: u32, height: u32, audio_source: &str, output_path: &Path) -> Command {
    let ffmpeg_path = std::env::var(FFMPEG_PATH_ENV).unwrap_or_else(|_| "ffmpeg".to_owned());
    let mut command = Command::new(ffmpeg_path);
    command
        .arg("-y")
        .arg("-hide_banner")
        .args(["-loglevel", "warning"])
        .args(["-thread_queue_size", "512"])
        .args(["-f", "rawvideo"])
        .args(["-pixel_format", "rgba"])
        .args(["-video_size", &format!("{width}x{height}")])
        .args(["-framerate", &DEFAULT_FRAMERATE.to_string()])
        .args(["-i", "pipe:0"])
        .args(["-thread_queue_size", "512"])
        .args(["-f", "pulse"])
        .args(["-i", audio_source])
        .args(["-c:v", "libx264"])
        .args(["-preset", "veryfast"])
        .args(["-crf", "20"])
        .args(["-c:a", "aac"])
        .args(["-b:a", "160k"])
        .args(["-pix_fmt", "yuv420p"])
        .arg("-shortest")
        .args(["-movflags", "+faststart"])
        .arg(output_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    command
}

fn wait_for_ffmpeg(child: &mut Child) -> Result<(), VideoCaptureError> {
    let deadline = std::time::Instant::now() + FFMPEG_STOP_TIMEOUT;
    loop {
        if child
            .try_wait()
            .map_err(VideoCaptureError::WaitFfmpeg)?
            .is_some()
        {
            return Ok(());
        }

        if std::time::Instant::now() >= deadline {
            child.kill().map_err(VideoCaptureError::KillFfmpeg)?;
            child.wait().map_err(VideoCaptureError::WaitFfmpeg)?;
            return Ok(());
        }

        thread::sleep(FFMPEG_STOP_POLL);
    }
}

fn next_capture_path() -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    Path::new(CAPTURE_DIR).join(format!("borrow-fighters-{timestamp}.mp4"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_zero_sized_capture() {
        let mut capture = VideoCapture::default();
        let error = capture.start(0, 540).unwrap_err();
        assert!(matches!(error, VideoCaptureError::InvalidFrameSize { .. }));
    }
}
