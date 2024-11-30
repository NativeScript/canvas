use canvas_c::WebGLActiveInfo as ActiveInfo;
use napi::bindgen_prelude::ObjectFinalize;
use napi::*;
use napi_derive::napi;

#[napi(js_name = "WebGLActiveInfo", custom_finalize)]
pub struct WebGLActiveInfo(pub(crate) *mut ActiveInfo);

impl ObjectFinalize for WebGLActiveInfo {
    fn finalize(self, _: Env) -> Result<()> {
        canvas_c::canvas_native_webgl_active_info_destroy(self.0);
        Ok(())
    }
}


#[napi]
impl WebGLActiveInfo {
    #[napi(getter)]
    pub fn get_name(&self) -> &str {
        let info = unsafe { &*self.0 };
        info.get_name()
    }

    #[napi(getter)]
    pub fn get_size(&self) -> i32 {
        let info = unsafe { &*self.0 };
        info.get_size()
    }

    #[napi(getter)]
    pub fn get_type(&self) -> u32 {
        let info = unsafe { &*self.0 };
        info.get_type()
    }

    #[napi(getter)]
    pub fn get_is_empty(&self) -> bool {
        let info = unsafe { &*self.0 };
        info.get_is_empty()
    }
}