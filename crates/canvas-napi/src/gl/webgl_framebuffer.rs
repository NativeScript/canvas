use napi::*;

use napi_derive::napi;

#[napi]
pub struct web_g_l_framebuffer {
  pub(crate) buffer: u32,
}
