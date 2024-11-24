use canvas_c::Path;
use napi::bindgen_prelude::{ClassInstance, Either};
use napi::*;
use napi_derive::napi;
use std::ffi::CString;

#[napi(js_name = "Path2D")]
pub struct JSPath2D {
    pub(crate) path: *mut Path,
}

#[napi]
impl JSPath2D {
    #[napi(constructor)]
    pub fn new(data: Option<Either<ClassInstance<JSPath2D>, JsString>>) -> Self {
        match data {
            Some(Either::A(path)) => {
                Self {
                    path: canvas_c::canvas_native_path_create_with_path(path.path as _)
                }
            }
            Some(Either::B(d)) => {
                if let Some(d) = d.into_utf8().ok() {
                    if let Ok(d) = d.as_str() {
                        let path = Box::into_raw(Box::new(Path::with_d(d)));
                        return Self { path };
                    }
                }

                Self {
                    path: canvas_c::canvas_native_path_create()
                }
            }
            _ => {
                Self {
                    path: canvas_c::canvas_native_path_create()
                }
            }
        }
    }

    #[napi]
    pub fn add_path(&self, path: ClassInstance<JSPath2D>) {
        canvas_c::canvas_native_path_add_path(self.path, path.path)
    }

    #[napi]
    pub fn arc(&self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64, anticlockwise: Option<bool>) {
        canvas_c::canvas_native_path_arc(
            self.path, x as f32, y as f32, radius as f32, start_angle as f32, end_angle as f32, anticlockwise.unwrap_or(false),
        )
    }

    #[napi]
    pub fn arc_to(&self, x1: f64, y1: f64, x2: f64, y2: f64, radius: f64) {
        canvas_c::canvas_native_path_arc_to(
            self.path, x1 as f32, y1 as f32, x2 as f32, y2 as f32, radius as f32,
        )
    }

    #[napi]
    pub fn bezier_curve_to(&self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        canvas_c::canvas_native_path_bezier_curve_to(
            self.path, cp1x as f32, cp1y as f32, cp2x as f32, cp2y as f32, x as f32, y as f32,
        )
    }

    #[napi]
    pub fn close_path(&self) {
        canvas_c::canvas_native_path_close_path(
            self.path)
    }

    #[napi]
    pub fn ellipse(&self, x: f64, y: f64, radius_x: f64, radius_y: f64, rotation: f64, start_angle: f64, end_angle: f64, anticlockwise: Option<bool>) {
        canvas_c::canvas_native_path_ellipse(
            self.path, x as f32, y as f32, radius_x as f32, radius_y as f32, rotation as f32, start_angle as f32, end_angle as f32, anticlockwise.unwrap_or(false),
        )
    }

    #[napi]
    pub fn line_to(&self, x: f64, y: f64) {
        canvas_c::canvas_native_path_line_to(
            self.path, x as f32, y as f32)
    }

    #[napi]
    pub fn move_to(&self, x: f64, y: f64) {
        canvas_c::canvas_native_path_move_to(
            self.path, x as f32, y as f32)
    }


    #[napi]
    pub fn quadratic_curve_to(&self, cpx: f64, cpy: f64, x: f64, y: f64) {
        canvas_c::canvas_native_path_quadratic_curve_to(
            self.path, cpx as f32, cpy as f32, x as f32, y as f32)
    }


    #[napi]
    pub fn rect(&self, x: f64, y: f64, width: f64, height: f64) {
        canvas_c::canvas_native_path_rect(
            self.path, x as f32, y as f32, width as f32, height as f32)
    }

    #[napi]
    pub fn round_rect(&self, x: f64, y: f64, width: f64, height: f64, radii: Either<f64, Vec<f64>>) {
        match radii {
            Either::A(radii) => {
                canvas_c::canvas_native_path_round_rect_tl_tr_br_bl(
                    self.path, x as f32, y as f32, width as f32, height as f32, radii as f32, radii as f32, radii as f32, radii as f32,
                )
            }
            Either::B(radii) => {
                let radii = radii.into_iter().map(|v| v as f32).collect::<Vec<f32>>();
                unsafe {
                    canvas_c::canvas_native_path_round_rect(
                        self.path, x as f32, y as f32, width as f32, height as f32, radii.as_ptr() as _, radii.len(),
                    )
                }
            }
        }
    }


    #[napi]
    pub fn trim(&self, start: f64, end: f64) {
        canvas_c::canvas_native_path_trim(
            self.path, start as f32, end as f32)
    }


    #[napi]
    pub fn __to_svg(&self) -> String {
        let ret = canvas_c::canvas_native_path_to_string(self.path as _);
        if ret.is_null() {
            return "".to_string();
        }
        let ret = unsafe { CString::from_raw(ret as _) };
        ret.to_string_lossy().to_string()
    }
}