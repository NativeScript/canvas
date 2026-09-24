//! Recording a frame into an `SkPicture` on the owning thread and replaying it on another.
//! Recording is cheap next to raster, so only raster moves off the document's thread.

use skia_safe::{Canvas, Picture, PictureRecorder, Rect};

/// A finished display list; cloning shares the picture.
/// `Send`/`Sync` hold because `SkPicture` is immutable with an atomic refcount (rust-skia doesn't mark it).
#[derive(Clone)]
pub struct RecordedFrame {
    picture: Picture,
    width: i32,
    height: i32,
    scale: f32,
}

unsafe impl Send for RecordedFrame {}
unsafe impl Sync for RecordedFrame {}

impl RecordedFrame {
    /// Replays the frame. Safe to call from a thread that has never touched the document.
    pub fn replay(&self, canvas: &Canvas) {
        canvas.draw_picture(&self.picture, None, None);
    }

    /// The geometry this frame was recorded for; a surface of another size needs a new one.
    pub fn geometry(&self) -> (i32, i32, f32) {
        (self.width, self.height, self.scale)
    }

    /// Identifies the recording; two frames with the same id replay the same display list.
    pub fn picture_id(&self) -> u32 {
        self.picture.unique_id()
    }

    pub fn approximate_bytes(&self) -> usize {
        self.picture.approximate_bytes_used()
    }
}

/// A recording kept for reuse, with the container size it was laid out against.
pub(crate) struct CachedFrame {
    frame: RecordedFrame,
    container: skia_safe::Size,
}

/// A few sizes of one document. Entries are display lists, not pixels.
const MAX_CACHED_FRAMES: usize = 4;

impl super::SvgDocument {
    /// When on, every view replays one shared recording per frame.
    /// Direct node mutations bypass the document, so callers must `invalidate_frames`.
    pub fn set_frame_sharing(&mut self, enabled: bool) {
        self.frames = enabled.then(Vec::new);
    }

    pub fn frame_sharing(&self) -> bool {
        self.frames.is_some()
    }

    /// Drops every cached recording; the next frame records afresh.
    pub fn invalidate_frames(&mut self) {
        if let Some(frames) = self.frames.as_mut() {
            frames.clear();
        }
    }

    /// The current frame, reusing another view's recording of this geometry when sharing.
    pub fn frame(&mut self, width: i32, height: i32, scale: f32) -> Option<RecordedFrame> {
        if self.frames.is_none() {
            return self.record(width, height, scale);
        }
        let container = self.container_size();
        let hit = self.frames.as_ref().and_then(|frames| {
            frames
                .iter()
                .find(|c| c.container == container && c.frame.geometry() == (width, height, scale))
                .map(|c| c.frame.clone())
        });
        if hit.is_some() {
            return hit;
        }
        let frame = self.record(width, height, scale)?;
        if let Some(frames) = self.frames.as_mut() {
            if frames.len() == MAX_CACHED_FRAMES {
                frames.remove(0);
            }
            frames.push(CachedFrame {
                frame: frame.clone(),
                container,
            });
        }
        Some(frame)
    }

    pub fn draw(&mut self, canvas: &Canvas, width: i32, height: i32, scale: f32) {
        if self.frames.is_none() {
            self.render_frame(canvas, width, height, scale);
        } else if let Some(frame) = self.frame(width, height, scale) {
            frame.replay(canvas);
        }
    }

    /// Paints the document into a display list instead of onto a surface.
    pub fn record(&mut self, width: i32, height: i32, scale: f32) -> Option<RecordedFrame> {
        let mut recorder = PictureRecorder::new();
        let canvas = recorder.begin_recording(Rect::from_wh(width as f32, height as f32), false);
        self.render_frame(canvas, width, height, scale);
        recorder
            .finish_recording_as_picture(None)
            .map(|picture| RecordedFrame {
                picture,
                width,
                height,
                scale,
            })
    }
}

/// Hands the newest frame from the owning thread to the render thread.
/// Unconsumed frames are dropped, not queued, so a slow raster never builds a backlog.
#[derive(Default)]
pub struct FrameSlot {
    latest: std::sync::Mutex<Option<RecordedFrame>>,
}

impl FrameSlot {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn commit(&self, frame: RecordedFrame) {
        if let Ok(mut slot) = self.latest.lock() {
            *slot = Some(frame);
        }
    }

    /// Takes the newest frame, if one was committed since the last call.
    pub fn take(&self) -> Option<RecordedFrame> {
        self.latest.lock().ok().and_then(|mut slot| slot.take())
    }

    pub fn has_frame(&self) -> bool {
        self.latest.lock().map(|slot| slot.is_some()).unwrap_or(false)
    }
}
