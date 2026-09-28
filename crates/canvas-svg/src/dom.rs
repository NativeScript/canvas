use crate::smil::Timeline;
use skia_safe::svg::{Dom, LoadError, Node};
use skia_safe::{Canvas, FontMgr, Image, Matrix, Size};
use std::collections::HashMap;

/// `Dom` has no constructor that skips parsing.
pub const INITIAL_SVG: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>";

/// Raster of everything except the promoted node; reused only while the geometry matches.
struct Backdrop {
    image: Image,
    width: i32,
    height: i32,
    scale: f32,
}

/// The promoted subtree's raster. Only translation is served from it: scale or rotation would
/// resample, so those re-render to stay pixel-identical.
struct LayerRaster {
    image: Image,
    width: i32,
    height: i32,
    scale: f32,
    /// Transform baked into the raster; its delta to the current one is the draw offset.
    captured: Matrix,
}

/// A live, mutable SVG document backed directly by Skia's own node tree.
pub struct SvgDocument {
    dom: Dom,
    id_registry: HashMap<String, Node>,
    layer_id: Option<String>,
    backdrop: Option<Backdrop>,
    layer_raster: Option<LayerRaster>,
    /// Set when the timeline's only change inside the layer was its transform. A JS
    /// `set_attribute` is not seen here, so an unarmed frame always renders.
    layer_composite_armed: bool,
    /// SMIL animations, lifted out of the source before Skia's parser drops them.
    timeline: Timeline,
    container: Size,
    /// Shared recordings of the current state, one per geometry. `None` when sharing is off.
    pub(crate) frames: Option<Vec<crate::frame::CachedFrame>>,
}

impl SvgDocument {
    pub fn new() -> Self {
        Self::from_bytes(INITIAL_SVG.as_bytes())
            .expect("the bootstrap svg document is always well-formed")
    }

    /// SMIL elements are extracted first, since Skia's parser silently drops them.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, LoadError> {
        let extracted = crate::smil::extract(bytes);
        let mgr = FontMgr::new();
        let dom = Dom::from_bytes(&extracted.source, mgr)?;
        Ok(Self {
            dom,
            id_registry: HashMap::new(),
            layer_id: None,
            backdrop: None,
            layer_raster: None,
            layer_composite_armed: false,
            timeline: Timeline::new(extracted.animations),
            container: Size::new(0.0, 0.0),
            frames: None,
        })
    }

    pub fn root(&self) -> Node {
        self.dom.root().into_node()
    }

    pub fn render(&self, canvas: &Canvas) {
        self.dom.render(canvas);
    }

    pub fn set_container_size(&mut self, width: f32, height: f32) {
        let size = Size::new(width, height);
        if self.container == size {
            return;
        }
        self.container = size;
        self.dom.set_container_size(size);
        self.backdrop = None;
    }

    pub fn container_size(&self) -> Size {
        self.container
    }

    /// Also registers with Skia so `<use>`/`clip-path`/`mask`/`filter` resolve; Skia only
    /// builds that map while parsing.
    pub fn register_id(&mut self, id: impl Into<String>, node: Node) {
        let id = id.into();
        self.dom.set_node_by_id(&id, Some(&node));
        self.id_registry.insert(id, node);
    }

    pub fn unregister_id(&mut self, id: &str) {
        self.dom.set_node_by_id(id, None);
        self.id_registry.remove(id);
    }

    /// Falls back to Skia's map, the only one populated for a parsed document.
    pub fn get_element_by_id(&mut self, id: &str) -> Option<Node> {
        if let Some(node) = self.id_registry.get(id) {
            return Some(node.clone());
        }
        self.dom.find_node_by_id(id)
    }

    /// Adds CSS `@keyframes` animations from a stylesheet outside the document. Only `#id`
    /// selectors that exist in the document apply. Returns whether the timeline is now running,
    /// so a caller whose loop had stopped knows to restart it.
    pub fn add_stylesheet(&mut self, css: &str) -> bool {
        self.timeline.extend(crate::smil::extract_from_css(css));
        self.timeline.is_running()
    }

    pub fn has_animations(&self) -> bool {
        !self.timeline.is_empty()
    }

    pub fn animation_count(&self) -> usize {
        self.timeline.len()
    }

    /// In seconds.
    pub fn current_time(&self) -> f64 {
        self.timeline.time()
    }

    /// When every animation has finished, in seconds, or `None` if any of them repeats forever.
    pub fn animation_duration(&self) -> Option<f64> {
        self.timeline.duration()
    }

    /// Returns false once every animation has ended or frozen, so the caller can stop scheduling frames.
    pub fn set_current_time(&mut self, seconds: f64) -> bool {
        self.advance(seconds).running
    }

    /// As [`set_current_time`], but also reports whether anything changed so idle frames skip redrawing.
    pub fn advance(&mut self, seconds: f64) -> crate::smil::Applied {
        if self.timeline.is_empty() {
            return crate::smil::Applied {
                changed: false,
                changed_outside_layer: false,
                changed_layer_content: false,
                running: false,
            };
        }
        let applied = self
            .timeline
            .apply(&mut self.dom, seconds, self.layer_id.as_deref());
        // Only when something moved: frozen animations would otherwise re-capture every frame.
        if applied.changed_outside_layer {
            self.backdrop = None;
        }
        if applied.changed_layer_content {
            self.layer_raster = None;
        }
        if applied.changed {
            self.invalidate_frames();
        }
        self.layer_composite_armed = !applied.changed_layer_content;
        applied
    }

    /// The promoted node is composited last over a cached raster, so content meant to paint
    /// over it ends up beneath it. Promote the topmost animating subtree.
    pub fn set_layer(&mut self, id: Option<&str>) {
        let id = id.map(str::to_owned);
        if self.layer_id == id {
            return;
        }
        self.layer_id = id;
        self.backdrop = None;
        self.layer_raster = None;
        self.invalidate_frames();
    }

    pub fn layer(&self) -> Option<&str> {
        self.layer_id.as_deref()
    }

    /// Call when anything outside the promoted subtree changes.
    pub fn invalidate_backdrop(&mut self) {
        self.backdrop = None;
        self.layer_raster = None;
        self.invalidate_frames();
    }

    pub fn has_backdrop(&self) -> bool {
        self.backdrop.is_some()
    }


    fn backdrop_matches(&self, width: i32, height: i32, scale: f32) -> bool {
        matches!(
            &self.backdrop,
            Some(b) if b.width == width && b.height == height && b.scale == scale
        )
    }

    fn capture_backdrop(&mut self, width: i32, height: i32, scale: f32) {
        let Some(id) = self.layer_id.clone() else {
            return;
        };
        let Some(node) = self.get_element_by_id(&id) else {
            // Stale id: fall back to whole-document rendering.
            self.layer_id = None;
            return;
        };

        // `display` isn't inherited and defaults to `inline`, so restoring `inline` equals
        // never having set it.
        let mut typed = node.typed();
        let previous = crate::get_attribute(&typed, "display");
        crate::set_attribute(&mut typed, "display", "none");

        let info = skia_safe::ImageInfo::new_n32_premul(skia_safe::ISize::new(width, height), None);
        let captured = skia_safe::surfaces::raster(&info, None, None).map(|mut surface| {
            let canvas = surface.canvas();
            canvas.clear(skia_safe::Color::TRANSPARENT);
            if scale != 1.0 {
                canvas.scale((scale, scale));
            }
            self.dom.render(canvas);
            surface.image_snapshot()
        });

        crate::set_attribute(
            &mut typed,
            "display",
            previous.as_deref().unwrap_or("inline"),
        );

        self.backdrop = captured.map(|image| Backdrop {
            image,
            width,
            height,
            scale,
        });
    }

    fn layer_transform(&mut self) -> Option<Matrix> {
        let id = self.layer_id.clone()?;
        let node = self.get_element_by_id(&id)?;
        crate::node::transform_of(&node.typed())
    }

    fn layer_raster_matches(&self, width: i32, height: i32, scale: f32) -> bool {
        matches!(&self.layer_raster, Some(r) if r.width == width && r.height == height && r.scale == scale)
    }

    fn capture_layer(&mut self, width: i32, height: i32, scale: f32) {
        let Some(id) = self.layer_id.clone() else { return };
        let Some(captured) = self.layer_transform() else {
            // Not a transformable node, so no fast path.
            return;
        };

        // Rendered with its transform; a later frame differing only by translation reuses it.
        let info = skia_safe::ImageInfo::new_n32_premul(skia_safe::ISize::new(width, height), None);
        let image = skia_safe::surfaces::raster(&info, None, None).map(|mut surface| {
            let canvas = surface.canvas();
            canvas.clear(skia_safe::Color::TRANSPARENT);
            if scale != 1.0 {
                canvas.scale((scale, scale));
            }
            self.dom.render_node(canvas, &id);
            surface.image_snapshot()
        });

        self.layer_raster = image.map(|image| LayerRaster {
            image,
            width,
            height,
            scale,
            captured,
        });
    }

    /// Device-space offset for the cached raster if the node was only translated; `None` re-renders.
    fn layer_translation(&mut self, scale: f32) -> Option<(f32, f32)> {
        let raster = self.layer_raster.as_ref()?;
        let captured = raster.captured;
        let current = self.layer_transform()?;
        let unchanged = |a: f32, b: f32| (a - b).abs() < 1e-6;
        if !(unchanged(current.scale_x(), captured.scale_x())
            && unchanged(current.scale_y(), captured.scale_y())
            && unchanged(current.skew_x(), captured.skew_x())
            && unchanged(current.skew_y(), captured.skew_y())
            && unchanged(current.persp_x(), captured.persp_x())
            && unchanged(current.persp_y(), captured.persp_y()))
        {
            return None;
        }
        Some((
            (current.translate_x() - captured.translate_x()) * scale,
            (current.translate_y() - captured.translate_y()) * scale,
        ))
    }

    /// Must restore `canvas`: GPU backends reuse one surface, so a leaked scale compounds each
    /// frame (3, 9, 27...) until the view goes blank.
    pub fn render_frame(&mut self, canvas: &Canvas, width: i32, height: i32, scale: f32) {
        let restore = canvas.save();
        self.render_frame_inner(canvas, width, height, scale);
        canvas.restore_to_count(restore);
    }

    fn render_frame_inner(&mut self, canvas: &Canvas, width: i32, height: i32, scale: f32) {
        let Some(id) = self.layer_id.clone() else {
            if scale != 1.0 {
                canvas.scale((scale, scale));
            }
            self.dom.render(canvas);
            return;
        };

        if !self.backdrop_matches(width, height, scale) {
            self.capture_backdrop(width, height, scale);
        }

        match &self.backdrop {
            Some(b) => {
                // The backdrop is already at device scale; only the node needs scaling.
                canvas.draw_image(&b.image, (0, 0), None);

                // Only moved: draw the earlier raster at its new offset instead of re-rendering.
                let armed = std::mem::take(&mut self.layer_composite_armed);
                if armed {
                    if !self.layer_raster_matches(width, height, scale) {
                        self.capture_layer(width, height, scale);
                    }
                    if let Some((dx, dy)) = self.layer_translation(scale) {
                        if let Some(raster) = self.layer_raster.as_ref() {
                            canvas.draw_image(&raster.image, (dx, dy), None);
                            return;
                        }
                    }
                } else {
                    self.layer_raster = None;
                }

                let restore = canvas.save();
                if scale != 1.0 {
                    canvas.scale((scale, scale));
                }
                self.dom.render_node(canvas, &id);
                canvas.restore_to_count(restore);
            }
            None => {
                if scale != 1.0 {
                    canvas.scale((scale, scale));
                }
                self.dom.render(canvas);
            }
        }
    }
}

impl Default for SvgDocument {
    fn default() -> Self {
        Self::new()
    }
}
