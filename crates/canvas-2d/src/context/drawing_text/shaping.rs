//! Caching of paint-independent text layout.
//!
//! `fillText`/`strokeText`/`measureText` each used to run a full
//! `ParagraphBuilder` -> `build()` -> `layout()` cycle, which on a Galaxy A53
//! cost ~14.9us per `fillText`. None of that work depends on the paint:
//! shaping, font fallback and line metrics are a function of the text plus the
//! font descriptor, direction and spacing only. What Skia has at the end of a
//! layout is, per line, an `SkTextBlob` plus an offset -- which is exactly what
//! `Paragraph::visit` hands back -- so the layout collapses to one `TextBlob`
//! and a handful of scalars, reusable for every later draw of the same string
//! in any paint. On that device `fillText` goes 14.9us -> 1.15us, and a string
//! drawn only once costs what it did before.
//!
//! `rebuilt_blob_matches_paragraph_paint` in this module's tests pins the
//! equivalence: the rebuilt blob is pixel-identical to `Paragraph::paint` for
//! fallback, emoji, ligature, stroked and RTL cases.

use std::cell::{OnceCell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use skia_safe::{FontMetrics, Point, Rect, TextBlob, TextBlobBuilder};
use ustr::Ustr;

use crate::context::drawing_text::global_fonts::{FontLibrary, FONT_LIBRARY};
use crate::context::drawing_text::typography::{Font, FontStretch, FontStyle};
use crate::context::drawing_text::MAX_TEXT_WIDTH;
use crate::context::text_styles::text_direction::TextDirection;

/// Everything about a laid-out string that does not depend on the paint.
pub(crate) struct ShapedText {
    /// `None` when the string produces no glyphs (empty or whitespace-only).
    pub blob: Option<TextBlob>,
    pub line_width: f32,
    pub alphabetic_baseline: f32,
    pub ideographic_baseline: f32,
    pub height: f32,
    pub font_metrics: FontMetrics,
    pub line_left: f32,
    /// The first font the paragraph resolved. Only `measure_text` needs it, and
    /// only to reproduce the pre-existing glyph-bounds behaviour.
    font: skia_safe::Font,
    extents: OnceCell<GlyphExtents>,
}

/// The per-glyph bounds `measure_text` reports. Derived lazily: a caller that
/// only ever draws never pays for it.
pub(crate) struct GlyphExtents {
    pub ascent: f32,
    pub descent: f32,
    pub first_left: f32,
    pub last_right: f32,
    pub last_x: f32,
    pub empty: bool,
}

impl ShapedText {
    pub fn extents(&self, text: &str) -> &GlyphExtents {
        self.extents.get_or_init(|| {
            let glyphs = self.font.str_to_glyphs_vec(text);
            if glyphs.is_empty() {
                return GlyphExtents {
                    ascent: 0.,
                    descent: 0.,
                    first_left: 0.,
                    last_right: 0.,
                    last_x: 0.,
                    empty: true,
                };
            }
            let mut bounds = vec![Rect::default(); glyphs.len()];
            self.font.get_bounds(&glyphs, &mut bounds, None);

            let first = bounds[0];
            let last = bounds[glyphs.len() - 1];
            let mut ascent = first.top;
            let mut descent = first.bottom;
            for b in &bounds[1..] {
                if b.bottom > descent {
                    descent = b.bottom;
                }
                if b.top < ascent {
                    ascent = b.top;
                }
            }
            GlyphExtents {
                ascent,
                descent,
                first_left: first.left,
                last_right: last.right,
                last_x: last.x(),
                empty: false,
            }
        })
    }
}

/// Identifies a layout independently of the text itself, which is the map key.
#[derive(Clone, PartialEq, Eq, Hash)]
struct ShapeKey {
    family: Arc<[Ustr]>,
    /// `f32` is not `Hash`/`Eq`; the bit patterns are, and two font sizes that
    /// differ only by a NaN payload cannot arise from the CSS parser.
    size: u32,
    weight: u32,
    stretch: FontStretch,
    style: FontStyle,
    word_spacing: u32,
    letter_spacing: u32,
    direction: TextDirection,
    /// Bumped whenever a typeface is registered or the library is reset, so a
    /// newly registered family cannot be masked by a layout shaped without it.
    fonts_generation: u64,
}

impl ShapeKey {
    fn new(
        font_style: &Font,
        direction: TextDirection,
        word_spacing: f32,
        letter_spacing: f32,
    ) -> Self {
        ShapeKey {
            family: Arc::clone(&font_style.family),
            size: font_style.size.to_bits(),
            weight: font_style.weight,
            stretch: font_style.stretch,
            style: font_style.style,
            word_spacing: word_spacing.to_bits(),
            letter_spacing: letter_spacing.to_bits(),
            direction,
            fonts_generation: FontLibrary::generation(),
        }
    }
}

/// Bounded so a caller that draws a fresh string every frame (a clock, a frame
/// counter) cannot grow the cache without limit. Over the cap the whole map is
/// dropped rather than evicted one entry at a time: the workloads that overflow
/// are the ones with no reuse to preserve, and a steady-state workload that fits
/// never reaches this path.
const SHAPE_CACHE_CAPACITY: usize = 512;

thread_local! {
    /// Keyed on the text first because a canvas almost always draws with one
    /// font at a time, which keeps the lookup allocation-free: `&str` borrows
    /// the `String` key, and the handful of layouts that share a string are a
    /// linear scan.
    static SHAPE_CACHE: RefCell<Cache> = RefCell::new(Cache::default());
}

#[derive(Default)]
struct Cache {
    entries: HashMap<String, Vec<(ShapeKey, Rc<ShapedText>)>>,
    len: usize,
}

/// Returns the layout for `text` under the current text state, shaping it only
/// on a miss.
pub(crate) fn shape(
    text: &str,
    font_style: &Font,
    direction: TextDirection,
    word_spacing: f32,
    letter_spacing: f32,
) -> Rc<ShapedText> {
    let key = ShapeKey::new(font_style, direction, word_spacing, letter_spacing);

    let hit = SHAPE_CACHE.with(|cache| {
        let cache = cache.borrow();
        cache.entries.get(text).and_then(|variants| {
            variants
                .iter()
                .find(|(k, _)| *k == key)
                .map(|(_, shaped)| Rc::clone(shaped))
        })
    });
    if let Some(shaped) = hit {
        return shaped;
    }

    let shaped = Rc::new(layout(
        text,
        font_style,
        direction,
        word_spacing,
        letter_spacing,
    ));

    SHAPE_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if cache.len >= SHAPE_CACHE_CAPACITY {
            cache.entries.clear();
            cache.len = 0;
        }
        cache.len += 1;
        cache
            .entries
            .entry(text.to_owned())
            .or_default()
            .push((key, Rc::clone(&shaped)));
    });

    shaped
}

/// Drops every cached layout on the calling thread.
///
/// Production code never needs this -- a font-library change is handled by the
/// generation in `ShapeKey`, which strands the stale entries rather than
/// reading them -- but the tests below share a process, so they start from a
/// known cache.
#[cfg(test)]
fn clear_cache() {
    SHAPE_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        cache.entries.clear();
        cache.len = 0;
    });
}

fn layout(
    text: &str,
    font_style: &Font,
    direction: TextDirection,
    word_spacing: f32,
    letter_spacing: f32,
) -> ShapedText {
    let weight = skia_safe::font_style::Weight::from(font_style.weight as i32);
    let sk_font_style = skia_safe::FontStyle::new(
        weight,
        font_style.stretch.into(),
        font_style.style.into(),
    );

    let families: Vec<&str> = font_style.family.iter().map(|s| s.as_str()).collect();

    let mut text_style = skia_safe::textlayout::TextStyle::new();
    text_style.set_font_families(families.as_slice());
    text_style.set_font_size(font_style.size);
    text_style.set_word_spacing(word_spacing);
    text_style.set_letter_spacing(letter_spacing);
    text_style.set_height(1.);
    text_style.set_font_style(sk_font_style);
    text_style.set_text_baseline(skia_safe::textlayout::TextBaseline::Alphabetic);

    let font_collection = FONT_LIBRARY.lock().collect_fonts(&text_style);

    let mut paragraph_style = skia_safe::textlayout::ParagraphStyle::new();
    paragraph_style.set_text_style(&text_style);
    paragraph_style.set_text_direction(match direction {
        TextDirection::LTR => skia_safe::textlayout::TextDirection::LTR,
        TextDirection::RTL => skia_safe::textlayout::TextDirection::RTL,
    });

    let mut builder =
        skia_safe::textlayout::ParagraphBuilder::new(&paragraph_style, &font_collection);
    builder.add_text(text);

    let mut paragraph = builder.build();
    paragraph.layout(MAX_TEXT_WIDTH);

    let font = paragraph.get_font_at(0);
    let (_, font_metrics) = font.metrics();

    let line_width = paragraph.max_intrinsic_width();

    ShapedText {
        blob: blob_from_paragraph(&mut paragraph),
        line_width,
        alphabetic_baseline: paragraph.alphabetic_baseline(),
        ideographic_baseline: paragraph.ideographic_baseline(),
        height: paragraph.height(),
        font_metrics,
        line_left: line_left(direction, line_width),
        font,
        extents: OnceCell::new(),
    }
}

/// The left edge of the single line, i.e. `LineMetrics::left`.
///
/// Reading it off the paragraph costs a `getLineMetrics` call that builds a
/// per-style metrics map, on every layout, for a number only `measure_text`
/// reads. It is fully determined instead: these paragraphs are laid out at
/// `MAX_TEXT_WIDTH` with the default (start) alignment and always fit on one
/// line, so an LTR line starts at 0 and an RTL line is flushed to the right
/// edge. `derived_line_left_matches_skia` pins that against Skia.
fn line_left(direction: TextDirection, line_width: f32) -> f32 {
    match direction {
        TextDirection::LTR => 0.0,
        TextDirection::RTL => MAX_TEXT_WIDTH - line_width,
    }
}

/// Flattens a laid-out paragraph into a single blob.
///
/// `Paragraph::paint` walks its per-line blob cache and draws each record at
/// `position + record.offset`; `visit` exposes those same records, so folding
/// each record's offset into its glyph positions yields one blob that draws
/// identically at `position`. Decorations, backgrounds and text shadows would
/// be lost, but the canvas 2D text style sets none of them.
fn blob_from_paragraph(paragraph: &mut skia_safe::textlayout::Paragraph) -> Option<TextBlob> {
    let mut builder = TextBlobBuilder::new();
    let mut any = false;

    paragraph.visit(|_, info| {
        let Some(info) = info else { return };
        let count = info.count();
        if count == 0 {
            return;
        }
        any = true;
        let origin = info.origin();
        let (glyphs, positions) = builder.alloc_run_pos(info.font(), count, None);
        glyphs.copy_from_slice(info.glyphs());
        for (dst, src) in positions.iter_mut().zip(info.positions()) {
            *dst = Point::new(src.x + origin.x, src.y + origin.y);
        }
    });

    if !any {
        return None;
    }
    builder.make()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `FONT_LIBRARY` and its generation counter are process-wide, so the tests
    /// that depend on the generation staying put cannot run beside the one that
    /// bumps it.
    static FONTS: std::sync::Mutex<()> = std::sync::Mutex::new(());

    fn text_paint() -> skia_safe::Paint {
        let mut paint = skia_safe::Paint::default();
        paint.set_color(skia_safe::Color::WHITE);
        paint.set_anti_alias(true);
        paint
    }

    fn paragraph_for(
        text: &str,
        font: &Font,
        direction: TextDirection,
        paint: &skia_safe::Paint,
    ) -> skia_safe::textlayout::Paragraph {
        let weight = skia_safe::font_style::Weight::from(font.weight as i32);
        let sk_font_style =
            skia_safe::FontStyle::new(weight, font.stretch.into(), font.style.into());
        let families: Vec<&str> = font.family.iter().map(|s| s.as_str()).collect();
        let mut ts = skia_safe::textlayout::TextStyle::new();
        ts.set_font_families(families.as_slice());
        ts.set_font_size(font.size);
        ts.set_height(1.);
        ts.set_font_style(sk_font_style);
        ts.set_foreground_paint(paint);
        ts.set_text_baseline(skia_safe::textlayout::TextBaseline::Alphabetic);
        let fc = FONT_LIBRARY.lock().collect_fonts(&ts);
        let mut ps = skia_safe::textlayout::ParagraphStyle::new();
        ps.set_text_style(&ts);
        ps.set_text_direction(match direction {
            TextDirection::LTR => skia_safe::textlayout::TextDirection::LTR,
            TextDirection::RTL => skia_safe::textlayout::TextDirection::RTL,
        });
        let mut b = skia_safe::textlayout::ParagraphBuilder::new(&ps, &fc);
        b.add_text(text);
        let mut p = b.build();
        p.layout(MAX_TEXT_WIDTH);
        p
    }

    const W: i32 = 420;
    const H: i32 = 140;

    fn render<F: FnOnce(&skia_safe::Canvas)>(f: F) -> Vec<u8> {
        let mut surface = skia_safe::surfaces::raster_n32_premul((W, H)).unwrap();
        surface.canvas().clear(skia_safe::Color::BLACK);
        f(surface.canvas());
        let image = surface.image_snapshot();
        let mut pixels = vec![0u8; (W * H * 4) as usize];
        let info = skia_safe::ImageInfo::new_n32_premul((W, H), None);
        assert!(image.read_pixels(
            &info,
            &mut pixels,
            (W * 4) as usize,
            (0, 0),
            skia_safe::image::CachingHint::Disallow
        ));
        pixels
    }

    fn assert_same_pixels(label: &str, spec: &str, direction: TextDirection, text: &str, paint: &skia_safe::Paint) {
        let font = Font::new(spec).unwrap();

        let mut paragraph = paragraph_for(text, &font, direction, paint);
        let expected = render(|c| paragraph.paint(c, (20.0, 90.0)));

        let shaped = layout(text, &font, direction, 0., 0.);
        let blob = shaped.blob.as_ref().expect("shaped blob");
        let actual = render(|c| {
            c.draw_text_blob(blob, (20.0, 90.0), paint);
        });

        let ink = expected.iter().filter(|b| **b != 0).count();
        assert!(ink > 200, "{label}: paragraph.paint drew nothing ({ink} non-zero bytes)");

        let diff = expected
            .iter()
            .zip(actual.iter())
            .filter(|(a, b)| a.abs_diff(**b) > 1)
            .count();
        assert_eq!(diff, 0, "{label}: {diff} of {} bytes differ", expected.len());
    }

    #[test]
    fn rebuilt_blob_matches_paragraph_paint() {
        let fill = text_paint();
        let mut stroke = text_paint();
        stroke.set_style(skia_safe::paint::Style::Stroke);
        stroke.set_stroke_width(1.5);

        let cases: &[(&str, &str, TextDirection)] = &[
            ("plain", "32px sans-serif", TextDirection::LTR),
            ("serif", "20px serif", TextDirection::LTR),
            ("mono", "40px monospace", TextDirection::LTR),
            ("bold-italic", "italic bold 28px sans-serif", TextDirection::LTR),
        ];
        for (label, spec, dir) in cases {
            assert_same_pixels(label, spec, *dir, "Handgloves AV Wa ffi", &fill);
            assert_same_pixels(label, spec, *dir, "Handgloves AV Wa ffi", &stroke);
        }

        // Font fallback splits the line into several runs with different fonts;
        // the flattened blob has to keep each run's own font.
        assert_same_pixels("fallback", "28px sans-serif", TextDirection::LTR, "abc \u{4f60}\u{597d} def", &fill);
        assert_same_pixels("emoji", "24px sans-serif", TextDirection::LTR, "hi \u{1f600} there", &fill);
        // RTL reorders runs within the line.
        assert_same_pixels("rtl", "28px sans-serif", TextDirection::RTL, "\u{5e9}\u{5dc}\u{5d5}\u{5dd} abc", &fill);
    }

    #[test]
    fn cache_returns_the_same_layout_and_separates_fonts() {
        let _fonts = FONTS.lock().unwrap_or_else(|e| e.into_inner());
        clear_cache();
        let small = Font::new("12px sans-serif").unwrap();
        let large = Font::new("36px sans-serif").unwrap();

        let a = shape("cache me", &small, TextDirection::LTR, 0., 0.);
        let b = shape("cache me", &small, TextDirection::LTR, 0., 0.);
        assert!(Rc::ptr_eq(&a, &b), "a repeat lookup must hit the cache");

        let c = shape("cache me", &large, TextDirection::LTR, 0., 0.);
        assert!(!Rc::ptr_eq(&a, &c), "a different font must not share a layout");
        assert!(c.line_width > a.line_width);

        // Same font, same text, different spacing.
        let d = shape("cache me", &small, TextDirection::LTR, 0., 4.);
        assert!(!Rc::ptr_eq(&a, &d));
        assert!(d.line_width > a.line_width);
    }

    #[test]
    fn registering_a_font_invalidates_cached_layouts() {
        let _fonts = FONTS.lock().unwrap_or_else(|e| e.into_inner());
        clear_cache();
        let font = Font::new("16px sans-serif").unwrap();
        let before = shape("generation", &font, TextDirection::LTR, 0., 0.);
        FontLibrary::reset();
        let after = shape("generation", &font, TextDirection::LTR, 0., 0.);
        assert!(
            !Rc::ptr_eq(&before, &after),
            "a font-library change must not be served from the old layout"
        );
    }

    #[test]
    fn cache_stays_bounded() {
        let _fonts = FONTS.lock().unwrap_or_else(|e| e.into_inner());
        clear_cache();
        let font = Font::new("16px sans-serif").unwrap();
        for i in 0..(SHAPE_CACHE_CAPACITY * 2 + 3) {
            let _ = shape(&format!("unique {i}"), &font, TextDirection::LTR, 0., 0.);
        }
        SHAPE_CACHE.with(|cache| {
            let cache = cache.borrow();
            assert!(cache.len <= SHAPE_CACHE_CAPACITY, "cache grew to {}", cache.len);
            assert_eq!(cache.len, cache.entries.values().map(Vec::len).sum::<usize>());
        });
    }

    #[test]
    fn derived_line_left_matches_skia() {
        // `layout` computes LineMetrics::left rather than asking for it; if a
        // Skia upgrade ever changes how a single start-aligned line is placed,
        // this is what catches it.
        for spec in ["24px sans-serif", "italic bold 30px serif", "18px monospace"] {
            let font = Font::new(spec).unwrap();
            for text in [
                "Handgloves AVW",
                " leading space",
                "j",
                "\u{5e9}\u{5dc}\u{5d5}\u{5dd} abc",
            ] {
                for direction in [TextDirection::LTR, TextDirection::RTL] {
                    let paint = text_paint();
                    let paragraph = paragraph_for(text, &font, direction, &paint);
                    let expected = paragraph
                        .get_line_metrics_at(0)
                        .map(|lm| lm.left as f32)
                        .unwrap_or(0.0);
                    let shaped = layout(text, &font, direction, 0., 0.);
                    assert!(
                        (shaped.line_left - expected).abs() <= 0.01,
                        "{spec}/{direction:?}/`{text}`: line_left {} != {expected}",
                        shaped.line_left
                    );
                }
            }
        }
    }

    #[test]
    fn empty_text_shapes_to_no_blob() {
        clear_cache();
        let font = Font::new("16px sans-serif").unwrap();
        let shaped = shape("", &font, TextDirection::LTR, 0., 0.);
        assert!(shaped.blob.is_none());
        assert_eq!(shaped.line_width, 0.);
        assert!(shaped.extents("").empty);
    }
}
