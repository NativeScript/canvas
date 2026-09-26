//! `Path2D`, mirroring `canvas2d/Path2D.cpp` in the V8 bindings.

use std::ptr;

use canvas_c::Path;
use napi::sys;

use crate::c2d::matrix::DOMMatrix;
use crate::util::class::ClassDef;
use crate::util::native::{Native, NativeType};
use crate::util::ret;
use crate::{constructor, method};

pub struct Path2D {
    pub(crate) path: *mut Path,
}

impl Native for Path2D {
    const KIND: NativeType = NativeType::Path2D;
}

impl Drop for Path2D {
    fn drop(&mut self) {
        canvas_c::canvas_native_path_release(self.path);
    }
}

constructor!(ctor, Path2D, 1, |cx| {
    let path = if cx.len() == 0 {
        canvas_c::canvas_native_path_create()
    } else if let Some(d) = cx.str(0) {
        canvas_c::canvas_native_path_create_with_string(d.as_ptr())
    } else if let Some(other) = cx.native::<Path2D>(0) {
        canvas_c::canvas_native_path_create_with_path(other.path)
    } else {
        canvas_c::canvas_native_path_create()
    };
    (!path.is_null()).then_some(Path2D { path })
});

method!(add_path, Path2D, 2, |cx, this| {
    if let Some(other) = cx.native::<Path2D>(0) {
        let matrix = cx
            .native::<DOMMatrix>(1)
            .map_or(ptr::null(), |m| m.matrix as *const _);
        canvas_c::canvas_native_path_add_path_with_matrix(this.path, other.path, matrix);
    }
    ret::undefined()
});

method!(arc, Path2D, 6, |cx, this| {
    let anticlockwise = cx.len() == 6 && cx.bool(5);
    canvas_c::canvas_native_path_arc(this.path, cx.f32(0), cx.f32(1), cx.f32(2), cx.f32(3), cx.f32(4), anticlockwise);
    ret::undefined()
});

method!(arc_to, Path2D, 5, |cx, this| {
    canvas_c::canvas_native_path_arc_to(this.path, cx.f32(0), cx.f32(1), cx.f32(2), cx.f32(3), cx.f32(4));
    ret::undefined()
});

method!(bezier_curve_to, Path2D, 6, |cx, this| {
    canvas_c::canvas_native_path_bezier_curve_to(
        this.path,
        cx.f32(0),
        cx.f32(1),
        cx.f32(2),
        cx.f32(3),
        cx.f32(4),
        cx.f32(5),
    );
    ret::undefined()
});

method!(close_path, Path2D, 0, |cx, this| {
    canvas_c::canvas_native_path_close_path(this.path);
    ret::undefined()
});

method!(ellipse, Path2D, 8, |cx, this| {
    let anticlockwise = cx.len() > 7 && cx.bool(7);
    canvas_c::canvas_native_path_ellipse(
        this.path,
        cx.f32(0),
        cx.f32(1),
        cx.f32(2),
        cx.f32(3),
        cx.f32(4),
        cx.f32(5),
        cx.f32(6),
        anticlockwise,
    );
    ret::undefined()
});

method!(line_to, Path2D, 2, |cx, this| {
    canvas_c::canvas_native_path_line_to(this.path, cx.f32(0), cx.f32(1));
    ret::undefined()
});

method!(move_to, Path2D, 2, |cx, this| {
    canvas_c::canvas_native_path_move_to(this.path, cx.f32(0), cx.f32(1));
    ret::undefined()
});

method!(quadratic_curve_to, Path2D, 4, |cx, this| {
    canvas_c::canvas_native_path_quadratic_curve_to(this.path, cx.f32(0), cx.f32(1), cx.f32(2), cx.f32(3));
    ret::undefined()
});

method!(rect, Path2D, 4, |cx, this| {
    canvas_c::canvas_native_path_rect(this.path, cx.f32(0), cx.f32(1), cx.f32(2), cx.f32(3));
    ret::undefined()
});

method!(round_rect, Path2D, 5, |cx, this| {
    if cx.len() == 5 {
        let (x, y, w, h) = (cx.f32(0), cx.f32(1), cx.f32(2), cx.f32(3));
        match cx.value_type(4) {
            sys::ValueType::napi_object => {
                if let Some(radii) = cx.f32_list(4) {
                    if !radii.is_empty() {
                        canvas_c::canvas_native_path_round_rect(this.path, x, y, w, h, radii.as_ptr(), radii.len());
                    }
                }
            }
            _ => {
                let r = cx.f32(4);
                canvas_c::canvas_native_path_round_rect_tl_tr_br_bl(this.path, x, y, w, h, r, r, r, r);
            }
        }
    }
    ret::undefined()
});

method!(trim, Path2D, 2, |cx, this| {
    canvas_c::canvas_native_path_trim(this.path, cx.f32(0), cx.f32(1));
    ret::undefined()
});

method!(to_svg, Path2D, 0, |cx, this| {
    ret::c_string(cx.env, canvas_c::canvas_native_path_to_string(this.path))
});

pub unsafe fn init(env: sys::napi_env, exports: sys::napi_value) {
    ClassDef::new(c"Path2D", ctor)
        .method(c"addPath", add_path)
        .method(c"arc", arc)
        .method(c"arcTo", arc_to)
        .method(c"bezierCurveTo", bezier_curve_to)
        .method(c"closePath", close_path)
        .method(c"ellipse", ellipse)
        .method(c"lineTo", line_to)
        .method(c"moveTo", move_to)
        .method(c"quadraticCurveTo", quadratic_curve_to)
        .method(c"rect", rect)
        .method(c"roundRect", round_rect)
        .method(c"trim", trim)
        .method(c"__toSVG", to_svg)
        .define(env, exports, NativeType::Path2D);
}
