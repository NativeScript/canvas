mod attr;
mod css_color;
mod dom;
mod frame;
mod node;
mod smil;

pub use attr::{get_attribute, set_attribute};
pub use dom::SvgDocument;
pub use frame::{FrameSlot, RecordedFrame};
pub use smil::Applied;
pub use node::{
    append_child, create_element, create_text_node, set_text, tag_name, text, SvgElementHandle,
};

use std::io::{Read, Seek, SeekFrom};

use skia_safe::svg::Dom;
use skia_safe::FontMgr;

thread_local! {
    /// `FontMgr::new()` scans the system fonts (on Android it parses fonts.xml and maps every
    /// font file), tens of milliseconds each time. Made per document, a page of svgs froze the
    /// UI thread for hundreds of milliseconds on open.
    static FONT_MGR: FontMgr = FontMgr::new();
}

/// The calling thread's font manager, scanned once.
pub(crate) fn font_mgr() -> FontMgr {
    FONT_MGR.with(|mgr| mgr.clone())
}

/// One-shot rasterization for `Svg.fromSrcSync`/`fromSrc`; the live view uses `SvgDocument`.
pub fn draw_svg_from_path(surface: &mut skia_safe::Surface, path: &str) {
    let file = std::fs::File::open(path);
    match file {
        Ok(file) => {
            let mut reader = std::io::BufReader::new(file);
            let mut bytes = [0; 16];
            let result = reader.read(&mut bytes);
            match result {
                Ok(_) => {
                    let _ = reader.seek(SeekFrom::Start(0));
                    let mut source = Vec::new();
                    let _ = reader.read_to_end(&mut source);
                    let mgr = font_mgr();
                    match Dom::from_bytes(&css_color::normalize_source(&source), mgr) {
                        Ok(mut svg) => {
                            let size = skia_safe::Size::new(
                                surface.width() as f32,
                                surface.height() as f32,
                            );
                            let canvas = surface.canvas();
                            svg.set_container_size(size);
                            svg.render(canvas)
                        }
                        Err(e) => {
                            println!("svg read to string error: {}", e);
                        }
                    }
                    // TODO check bytes to verify it's an svg
                }
                Err(e) => {
                    println!("svg file read error: {}", e);
                }
            }
        }
        Err(e) => {
            println!("svg file open error: {}", e);
        }
    }
}

pub fn draw_svg(surface: &mut skia_safe::Surface, svg: &str) {
    let mgr = font_mgr();
    match Dom::from_bytes(&css_color::normalize_source(svg.as_bytes()), mgr) {
        Ok(mut svg) => {
            let size = skia_safe::Size::new(surface.width() as f32, surface.height() as f32);
            let canvas = surface.canvas();
            svg.set_container_size(size);
            svg.render(canvas)
        }
        Err(e) => {
            println!("svg read to string error: {}", e);
        }
    }
}

pub fn draw_svg_from_bytes(surface: &mut skia_safe::Surface, bytes: &[u8]) {
    let mgr = font_mgr();
    match Dom::from_bytes(&css_color::normalize_source(bytes), mgr) {
        Ok(mut svg) => {
            let size = skia_safe::Size::new(surface.width() as f32, surface.height() as f32);
            let canvas = surface.canvas();
            svg.set_container_size(size);
            svg.render(canvas)
        }
        Err(e) => {
            println!("svg from bytes error: {}", e);
        }
    }
}
