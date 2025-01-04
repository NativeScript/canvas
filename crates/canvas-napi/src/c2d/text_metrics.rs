use canvas_c::TextMetrics as CTextMetrics;

use napi::bindgen_prelude::ObjectFinalize;
use napi::*;
use napi_derive::napi;

#[napi(custom_finalize)]
pub struct TextMetrics {
    pub(crate) metrics: *mut CTextMetrics,
}

impl ObjectFinalize for TextMetrics {
    fn finalize(self, _: Env) -> Result<()> {
        canvas_c::canvas_native_text_metrics_release(self.metrics);
        Ok(())
    }
}


#[napi]
impl TextMetrics {
    #[napi(getter)]
    pub fn get_width(&self) -> f64 {
        canvas_c::canvas_native_text_metrics_get_width(self.metrics) as f64
    }

    #[napi(getter)]
    pub fn get_actual_bounding_box_left(
        &self
    ) -> f64 {
        canvas_c::canvas_native_text_metrics_get_actual_bounding_box_left(self.metrics) as f64
    }

    #[napi(getter)]
    pub fn get_actual_bounding_box_right(
        &self
    ) -> f64 {
        canvas_c::canvas_native_text_metrics_get_actual_bounding_box_right(self.metrics) as f64
    }

    #[napi(getter)]
    pub fn get_actual_bounding_box_ascent(
        &self
    ) -> f64 {
        canvas_c::canvas_native_text_metrics_get_actual_bounding_box_ascent(self.metrics) as f64
    }

    #[napi(getter)]
    pub fn get_actual_bounding_box_descent(
        &self
    ) -> f64 {
        canvas_c::canvas_native_text_metrics_get_actual_bounding_box_descent(self.metrics) as f64
    }

    #[napi(getter)]
    pub fn get_font_bounding_box_ascent(
        &self
    ) -> f64 {
        canvas_c::canvas_native_text_metrics_get_font_bounding_box_ascent(self.metrics) as f64
    }

    #[napi(getter)]
    pub fn get_font_bounding_box_descent(
        &self
    ) -> f64 {
        canvas_c::canvas_native_text_metrics_get_font_bounding_box_descent(self.metrics) as f64
    }

    #[napi(getter)]
    pub fn get_em_height_ascent(
        &self
    ) -> f64 {
        canvas_c::canvas_native_text_metrics_get_em_height_ascent(self.metrics) as f64
    }

    #[napi(getter)]
    pub fn get_em_height_descent(
        &self
    ) -> f64 {
        canvas_c::canvas_native_text_metrics_get_em_height_descent(self.metrics) as f64
    }

    #[napi(getter)]
    pub fn get_hanging_baseline(
        &self
    ) -> f64 {
        canvas_c::canvas_native_text_metrics_get_hanging_baseline(self.metrics) as f64
    }

    #[napi(getter)]
    pub fn get_alphabetic_baseline(
        &self
    ) -> f64 {
        canvas_c::canvas_native_text_metrics_get_alphabetic_baseline(self.metrics) as f64
    }

    #[napi(getter)]
    pub fn get_ideographic_baseline(
        &self
    ) -> f64 {
        canvas_c::canvas_native_text_metrics_get_ideographic_baseline(self.metrics) as f64
    }
}
