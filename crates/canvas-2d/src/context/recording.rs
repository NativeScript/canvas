//! Draws recorded into pictures and replayed elsewhere (the render thread). The transform and clip
//! live on the Skia canvas, so they are re-applied at every picture boundary.

use skia_safe::{
    AlphaType, Canvas, ClipOp, ColorType, Contains, Data, IVector, ImageInfo, Picture, PictureRecorder,
    Rect, M44,
};

use crate::context::{Context, State};

/// Recordings open with a save of their own: a bottom-level clip can't be undone, and `reset()` must.
pub(crate) const RECORDING_BASE: usize = 2;

pub(crate) struct Recording {
    recorder: PictureRecorder,
    bounds: Rect,
    ops: Vec<FrameOp>,
    /// Began with a full-canvas clear, so no earlier frame shows through.
    cleared: bool,
}

impl Recording {
    fn new(bounds: Rect) -> Self {
        let mut recorder = PictureRecorder::new();
        recorder.begin_recording(bounds, false).save();
        Self {
            recorder,
            bounds,
            ops: Vec::new(),
            cleared: false,
        }
    }

    #[inline]
    pub(crate) fn canvas(&mut self) -> &Canvas {
        // `restart` begins the next recording before returning.
        self.recorder
            .recording_canvas()
            .expect("recording canvas is always live")
    }

    fn cut(&mut self, stack: &[State], current: &State) {
        if let Some(picture) = self.restart(stack, current) {
            self.ops.push(FrameOp::Picture(picture));
        }
    }

    fn discard(&mut self, stack: &[State], current: &State) {
        let _ = self.restart(stack, current);
        self.ops.clear();
        self.cleared = true;
    }

    fn restart(&mut self, stack: &[State], current: &State) -> Option<Picture> {
        let matrix = self.canvas().local_to_device();
        let picture = self.recorder.finish_recording_as_picture(Some(&self.bounds));
        self.recorder.begin_recording(self.bounds, false).save();
        let canvas = self.recorder.recording_canvas().expect("just begun");
        apply_state(canvas, stack, current, &matrix);
        picture
    }
}

#[derive(Default)]
pub struct Frame {
    ops: Vec<FrameOp>,
    cleared: bool,
}

impl Frame {
    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
    }

    pub fn len(&self) -> usize {
        self.ops.len()
    }

    /// A later frame that clears the whole canvas hides this one, so it replaces it.
    pub fn append(&mut self, later: Frame) {
        if later.cleared {
            *self = later;
        } else {
            self.ops.extend(later.ops);
        }
    }
}

enum FrameOp {
    /// Sets up its own matrix and clip.
    Picture(Picture),
    /// `putImageData`, which ignores the clip and transform.
    Pixels(PixelWrite),
    /// An image that only exists on the rasterizing thread's GPU context (a video frame).
    External(ExternalDraw),
}

/// Runs on the rasterizing thread; the release step runs once the image has been drawn.
pub type ExternalImage =
    Box<dyn FnOnce(&mut Context) -> Option<(skia_safe::Image, ExternalRelease)> + Send>;
pub type ExternalRelease = Box<dyn FnOnce(&mut Context) + Send>;

struct ExternalDraw {
    state: StateSnapshot,
    src: Rect,
    dst: Rect,
    paint: skia_safe::Paint,
    sampling: skia_safe::SamplingOptions,
    make: ExternalImage,
}

struct PixelWrite {
    width: i32,
    height: i32,
    data: Data,
    row_bytes: usize,
    origin: IVector,
}

/// In device space, so it can be reapplied under any matrix.
#[derive(Clone)]
pub(crate) struct DeviceClip {
    pub(crate) path: skia_safe::Path,
    pub(crate) anti_alias: bool,
}

fn apply_clips(canvas: &Canvas, clips: &[DeviceClip]) {
    if clips.is_empty() {
        return;
    }
    canvas.reset_matrix();
    for c in clips {
        canvas.clip_path(&c.path, Some(ClipOp::Intersect), Some(c.anti_alias));
    }
}

fn apply_state(canvas: &Canvas, stack: &[State], current: &State, matrix: &M44) {
    for level in stack {
        apply_clips(canvas, &level.clips);
        canvas.set_matrix(&level.saved_matrix);
        canvas.save();
    }
    apply_clips(canvas, &current.clips);
    canvas.set_matrix(matrix);
}

/// Save levels flattened: a single draw only needs the net clip.
struct StateSnapshot {
    clips: Vec<DeviceClip>,
    matrix: M44,
}

// SkPath shares an immutable, atomically counted path ref.
unsafe impl Send for StateSnapshot {}

impl StateSnapshot {
    fn take(stack: &[State], current: &State, matrix: M44) -> Self {
        let clips = stack
            .iter()
            .chain(std::iter::once(current))
            .flat_map(|level| level.clips.iter().cloned())
            .collect();
        Self { clips, matrix }
    }

    fn apply(&self, canvas: &Canvas) {
        apply_clips(canvas, &self.clips);
        canvas.set_matrix(&self.matrix);
    }
}

impl Context {
    pub fn is_recording(&self) -> bool {
        self.recording.is_some()
    }

    pub fn begin_recording(&mut self) {
        if self.recording.is_some() {
            return;
        }
        let matrix = self.surface.canvas().local_to_device();
        let mut recording = Recording::new(self.surface_data.bounds);
        apply_state(recording.canvas(), &self.state_stack, &self.state, &matrix);
        self.recording = Some(recording);
    }

    /// The surface picks up the transform and clip, not the frame: replay that first.
    pub fn end_recording(&mut self) -> Frame {
        let Some(mut recording) = self.recording.take() else {
            return Frame::default();
        };
        let matrix = recording.canvas().local_to_device();
        recording.cut(&self.state_stack, &self.state);
        let canvas = self.surface.canvas();
        canvas.restore_to_count(1);
        apply_state(canvas, &self.state_stack, &self.state, &matrix);
        Frame {
            ops: recording.ops,
            cleared: recording.cleared,
        }
    }

    pub fn resize_recording(&mut self, width: f32, height: f32) {
        if self.recording.is_none() {
            return;
        }
        self.surface_data.bounds = Rect::from_wh(width, height);
        self.reset_state();
        self.path = Default::default();
        self.recording = Some(Recording::new(self.surface_data.bounds));
    }

    /// Frames must be replayed in order.
    pub fn take_frame(&mut self) -> Frame {
        let Some(recording) = self.recording.as_mut() else {
            return Frame::default();
        };
        recording.cut(&self.state_stack, &self.state);
        Frame {
            ops: std::mem::take(&mut recording.ops),
            cleared: std::mem::take(&mut recording.cleared),
        }
    }

    /// A full-canvas clear hides everything recorded before it in this frame.
    pub(crate) fn discard_if_cleared(&mut self, rect: &Rect) {
        let Some(recording) = self.recording.as_mut() else {
            return;
        };
        if !self.state.clips.is_empty() || self.state_stack.iter().any(|s| !s.clips.is_empty()) {
            return;
        }
        let matrix = recording.canvas().local_to_device_as_3x3();
        if !matrix.rect_stays_rect() || !matrix.map_rect(rect).0.contains(recording.bounds) {
            return;
        }
        recording.discard(&self.state_stack, &self.state);
    }

    /// Copies the bytes: the caller may change them before the frame is replayed.
    pub(crate) fn record_pixels(
        &mut self,
        info: &ImageInfo,
        bytes: &[u8],
        row_bytes: usize,
        origin: IVector,
    ) -> bool {
        let Some(recording) = self.recording.as_mut() else {
            return false;
        };
        recording.cut(&self.state_stack, &self.state);
        let len = (row_bytes * info.height().max(0) as usize).min(bytes.len());
        recording.ops.push(FrameOp::Pixels(PixelWrite {
            width: info.width(),
            height: info.height(),
            data: Data::new_copy(&bytes[..len]),
            row_bytes,
            origin,
        }));
        true
    }

    pub fn gpu_context(&mut self) -> Option<&mut skia_safe::gpu::DirectContext> {
        self.direct_context.as_mut()
    }

    /// For images that only exist on the rasterizing thread's GPU context, such as video frames.
    pub fn draw_external_image(
        &mut self,
        width: i32,
        height: i32,
        src: Rect,
        dst: Rect,
        make: ExternalImage,
    ) {
        let (src, dst) =
            crate::utils::fit_bounds(width as f32, height as f32, src, dst);
        self.state
            .paint
            .image_smoothing_quality_set(self.state.image_filter_quality());
        let paint = self.state.paint.image_paint().clone();
        let sampling: skia_safe::SamplingOptions = self.state.image_smoothing_quality.into();

        let Some(recording) = self.recording.as_mut() else {
            self.bind_surface();
            if let Some((image, release)) = make(self) {
                self.draw_image_src_xywh_dst_xywh(
                    &image,
                    src.x(),
                    src.y(),
                    src.width(),
                    src.height(),
                    dst.x(),
                    dst.y(),
                    dst.width(),
                    dst.height(),
                );
                release(self);
            }
            return;
        };
        let matrix = recording.canvas().local_to_device();
        recording.cut(&self.state_stack, &self.state);
        recording.ops.push(FrameOp::External(ExternalDraw {
            state: StateSnapshot::take(&self.state_stack, &self.state, matrix),
            src,
            dst,
            paint,
            sampling,
            make,
        }));
        self.surface_state = self.surface_state | crate::context::SurfaceState::Pending;
    }

    /// Copied while recording: the render thread reads it after the asset's lock is released.
    pub(crate) fn asset_image(
        &self,
        bytes: &[u8],
        width: i32,
        height: i32,
    ) -> Option<skia_safe::Image> {
        if self.recording.is_some() {
            crate::utils::image::from_image_slice(bytes, width, height)
        } else {
            crate::utils::image::from_image_slice_no_copy(bytes, width, height)
        }
    }

    pub fn replay(&mut self, frame: Frame) {
        if frame.ops.is_empty() {
            return;
        }
        self.bind_surface();
        for op in frame.ops {
            let canvas = self.surface.canvas();
            match op {
                FrameOp::Picture(picture) => {
                    let count = canvas.save_count();
                    canvas.save();
                    canvas.reset_matrix();
                    picture.playback(canvas);
                    canvas.restore_to_count(count);
                }
                FrameOp::Pixels(write) => {
                    let info = ImageInfo::new(
                        (write.width, write.height),
                        ColorType::RGBA8888,
                        AlphaType::Unpremul,
                        None,
                    );
                    let _ = canvas.write_pixels(&info, write.data.as_bytes(), write.row_bytes, write.origin);
                }
                FrameOp::External(draw) => {
                    let Some((image, release)) = (draw.make)(self) else {
                        continue;
                    };
                    let canvas = self.surface.canvas();
                    let count = canvas.save_count();
                    canvas.save();
                    draw.state.apply(canvas);
                    canvas.draw_image_rect_with_sampling_options(
                        &image,
                        Some((&draw.src, skia_safe::canvas::SrcRectConstraint::Strict)),
                        draw.dst,
                        draw.sampling,
                        &draw.paint,
                    );
                    canvas.restore_to_count(count);
                    drop(image);
                    release(self);
                }
            }
        }
        self.surface_state = self.surface_state | crate::context::SurfaceState::Pending;
    }
}
