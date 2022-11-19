use std::os::raw::c_float;

use skia_safe::{BlendMode, Paint, Rect};
use skia_safe::paint::Style;

use crate::common::context::Context;


impl Context {
    pub fn clear_rect(&mut self, x: c_float, y: c_float, width: c_float, height: c_float) {
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(Style::Fill);
        paint.set_blend_mode(BlendMode::Clear);
        self.surface
            .canvas()
            .draw_rect(Rect::from_xywh(x, y, width, height), &paint);
    }

    pub fn fill_rect(&mut self, rect: &Rect) {
        let path = skia_safe::Path::rect(rect, None);
        if let Some(paint) = self.state.paint.fill_shadow_paint(
            self.state.shadow_offset,
            self.state.shadow_color,
            self.state.shadow_blur,
        ) {
            self.surface.canvas().draw_path(&path, &paint);
        }
        self.surface
            .canvas()
           .draw_rect(rect, self.state.paint.fill_paint());
           // .draw_path(&path, self.state.paint.fill_paint());
    }

    pub fn stroke_rect(&mut self, rect: &Rect) {
        let path = skia_safe::Path::rect(rect, None);
        if let Some(paint) = &mut self.state.paint.stroke_shadow_paint(
            self.state.shadow_offset,
            self.state.shadow_color,
            self.state.shadow_blur,
        ) {
            self.surface.canvas().draw_path(&path, &paint);
        }
        self.surface
            .canvas()
            .draw_rect(rect, self.state.paint.fill_paint());
            //.draw_path(&path, self.state.paint.stroke_paint());
    }
}
