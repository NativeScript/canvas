use napi::*;
use napi_derive::napi;
use std::ffi::{c_void, CString};
#[napi]
pub struct WebGLRenderingContext {
    state: *mut canvas_c::WebGLState,
}


#[napi]
impl WebGLRenderingContext {

    // #[napi]
    // pub fn render(&self) {
    //     canvas_c::canvas_native_context_render(self.state)
    // }


    #[napi(getter)]
    pub fn get_drawing_buffer_width(&self) -> i32 {
        canvas_c::canvas_native_webgl_state_get_drawing_buffer_width(self.state)
    }

    #[napi(getter)]
    pub fn get_drawing_buffer_height(&self) -> i32 {
        canvas_c:: canvas_native_webgl_state_get_drawing_buffer_height(self.state)
    }

    #[napi(factory)]
    pub fn with_view(
        view: i64,
        version: i32,
        alpha: bool,
        antialias: bool,
        depth: bool,
        fail_if_major_performance_caveat: bool,
        power_preference: i32,
        premultiplied_alpha: bool,
        preserve_drawing_buffer: bool,
        stencil: bool,
        desynchronized: bool,
        xr_compatible: bool
    ) -> Result<WebGLRenderingContext> {
        let ret = canvas_c::canvas_native_webgl_create(
            view as _,
            version,
            alpha,
            antialias,
            depth,
            fail_if_major_performance_caveat,
            power_preference,
            premultiplied_alpha,
            preserve_drawing_buffer,
            stencil,
            desynchronized,
            xr_compatible
        );

        if ret.is_null() {
            return Err(napi::Error::from_reason("Invalid parameter"));
        }

        Ok(WebGLRenderingContext {
            state: ret
        })
    }

    #[napi(factory)]
    pub fn offscreen(
        width: i32,
        height: i32,
        version: i32,
        alpha: bool,
        antialias: bool,
        depth: bool,
        fail_if_major_performance_caveat: bool,
        power_preference: i32,
        premultiplied_alpha: bool,
        preserve_drawing_buffer: bool,
        stencil: bool,
        desynchronized: bool,
        xr_compatible: bool,
        is_canvas: bool,
    ) -> Result<WebGLRenderingContext> {
        let ret = canvas_c::canvas_native_webgl_create_no_window(
            width,
            height,
            version,
            alpha,
            antialias,
            depth,
            fail_if_major_performance_caveat,
            power_preference,
            premultiplied_alpha,
            preserve_drawing_buffer,
            stencil,
            desynchronized,
            xr_compatible,
            is_canvas,
        );

        if ret.is_null() {
            return Err(napi::Error::from_reason("Invalid parameter"));
        }

        Ok(WebGLRenderingContext {
            state: ret
        })
    }

    #[napi]
    pub fn clear_color(&self,
                       red: f64,
                       green: f64,
                       blue: f64,
                       alpha: f64) {
        canvas_c::canvas_native_webgl_clear_color(red as f32, green as f32, blue as f32, alpha as f32, self.state);
    }

    #[napi]
    pub fn clear(&self, mask: u32) {
        canvas_c::canvas_native_webgl_clear(mask, self.state);
    }

    #[napi]
    pub fn flush(&self) {
        canvas_c::canvas_native_webgl_flush(self.state);
    }


    #[napi(js_name = "toDataURL")]
    pub fn to_data_url(&self, format: String, encoderOptions: Option<f64>) -> String {
        let c_str = CString::new(format).unwrap();
        let quality = encoderOptions
            .map(|v| v as f32)
            .unwrap_or(0.92)
            .try_into()
            .unwrap_or(0.92);
        let quality: u32 = (quality * 100.) as u32;
        let ret = canvas_c::canvas_native_webgl_to_data_url(self.state, c_str.as_ptr(), quality);
        unsafe { CString::from_raw(ret as _).to_string_lossy().to_string() }
    }

    #[napi(getter, js_name = "COLOR_BUFFER_BIT")]
    pub fn COLOR_BUFFER_BIT(&self) -> u32 {
        0x00004000
    }
}