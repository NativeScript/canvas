use std::borrow::Cow;
use std::os::raw::c_float;

use skia_safe::PathEffect;

use crate::context::line_styles::line_cap::LineCap;
use crate::context::line_styles::line_join::LineJoin;
use crate::context::Context;

pub mod line_cap;
pub mod line_join;

impl Context {
    pub fn set_line_width(&mut self, width: c_float) {
        self.state.line_width = width;
        self.state.paint.stroke_paint_mut().set_stroke_width(width);
    }

    pub fn line_width(&self) -> c_float {
        self.state.line_width
    }

    pub fn set_line_cap(&mut self, cap: LineCap) {
        self.state.line_cap = cap;
        self.state
            .paint
            .stroke_paint_mut()
            .set_stroke_cap(cap.into());
    }

    pub fn line_cap(&self) -> LineCap {
        self.state.line_cap
    }

    pub fn line_join(&self) -> LineJoin {
        self.state.line_join
    }

    pub fn set_line_join(&mut self, join: LineJoin) {
        self.state.line_join = join;
        self.state
            .paint
            .stroke_paint_mut()
            .set_stroke_join(join.into());
    }

    pub fn miter_limit(&self) -> c_float {
        self.state.miter_limit
    }

    pub fn set_miter_limit(&mut self, limit: c_float) {
        self.state.miter_limit = limit;
        self.state.paint.stroke_paint_mut().set_stroke_miter(limit);
    }

    pub fn set_line_dash(&mut self, dash: &[c_float]) {
        let line_dash;
        let is_odd = (dash.len() % 2) != 0;
        if is_odd {
            line_dash = [dash, dash].concat();
        } else {
            line_dash = dash.to_vec()
        }
        let mut effect: Option<PathEffect> = None;
        if !line_dash.is_empty() {
            effect = PathEffect::dash(line_dash.as_slice(), self.state.line_dash_offset);
        }
        self.state.line_dash_list = line_dash;
        self.state.paint.stroke_paint_mut().set_path_effect(effect);
    }

    pub fn line_dash(&self) -> &[c_float] {
        self.state.line_dash_list.as_ref()
    }

    pub fn set_line_dash_offset(&mut self, offset: c_float) {
        // TODO ?
        self.state.line_dash_offset = offset;
        let list = self.state.line_dash_list.clone();
        self.set_line_dash(list.as_slice());
    }

    pub fn line_dash_offset(&self) -> c_float {
        self.state.line_dash_offset
    }
}
