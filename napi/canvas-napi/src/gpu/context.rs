use crate::gpu::adapter::g_p_u_adapter;
use crate::gpu::device::g_p_u_device;
use crate::gpu::enums::{
  GPUCanvasAlphaMode, GPUCanvasPresentMode, GPUTextureFormat, PredefinedColorSpaceEnum,
};
use crate::gpu::g_p_u;
use crate::gpu::objects::GPUExtent3DDict;
use crate::gpu::texture::g_p_u_texture;
use canvas_c::webgpu::gpu_texture::CanvasGPUTexture;
use canvas_c::StringBuffer;
use napi::bindgen_prelude::{ClassInstance, ObjectFinalize, This};
use napi::*;
use napi_derive::napi;
use std::ffi::CString;
use std::sync::Arc;

#[napi]
pub struct g_p_u_canvas_context {
  context: Arc<canvas_c::webgpu::gpu_canvas_context::CanvasGPUCanvasContext>,
}

#[napi(object)]
#[derive(Clone, Debug, Default)]
pub struct CanvasSurfaceCapabilities {
  pub formats: Vec<String>,
  pub usages: u32,
  pub alpha_modes: Vec<String>,
  pub present_modes: Vec<String>,
}

impl From<canvas_c::webgpu::structs::CanvasSurfaceCapabilities> for CanvasSurfaceCapabilities {
  fn from(value: canvas_c::webgpu::structs::CanvasSurfaceCapabilities) -> Self {
    let formats = unsafe { *Box::from_raw(value.formats as *mut StringBuffer) };
    let alpha_modes = unsafe { *Box::from_raw(value.alpha_modes as *mut StringBuffer) };
    let present_modes = unsafe { *Box::from_raw(value.present_modes as *mut StringBuffer) };
    Self {
      usages: value.usages,
      formats: formats.into(),
      alpha_modes: alpha_modes.into(),
      present_modes: present_modes.into(),
    }
  }
}

#[napi(object)]
pub struct GPUCanvasContextConfigOptions {
  pub device: ClassInstance<g_p_u_device>,
  pub format: Option<GPUTextureFormat>,
  pub usage: Option<u32>,
  pub view_formats: Option<Vec<GPUTextureFormat>>,
  pub color_space: Option<PredefinedColorSpaceEnum>,
  pub alpha_mode: Option<GPUCanvasAlphaMode>,
  pub present_mode: Option<GPUCanvasPresentMode>,
  pub size: Option<Either<Vec<u32>, GPUExtent3DDict>>,
}

#[napi]
impl g_p_u_canvas_context {
  #[napi(factory)]
  pub fn with_layer(gpu: &g_p_u, layer: i64, width: u32, height: u32) -> Self {
    unsafe {
      let context = canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_create(
        Arc::as_ptr(&gpu.instance),
        layer as _,
        width,
        height,
      );

      if context.is_null() {
        // todo throw
      }

      g_p_u_canvas_context {
        context: Arc::from_raw(context),
      }
    }
  }

  #[napi(factory)]
  pub fn with_view(gpu: &g_p_u, layer: i64, width: u32, height: u32) -> Self {
    unsafe {
      let context =
        canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_create_nsview(
          Arc::as_ptr(&gpu.instance),
          layer as _,
          width,
          height,
        );
      if context.is_null() {
        // todo throw
      }
      g_p_u_canvas_context {
        context: Arc::from_raw(context),
      }
    }
  }

  #[napi]
  pub fn resize(&self, layer: i64, width: u32, height: u32) {
    unsafe {
      canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_resize_layer(
        Arc::as_ptr(&self.context),
        layer as _,
        width,
        height,
      )
    }
  }

  #[napi]
  pub fn present_surface(&self) {
    let context = Arc::as_ptr(&self.context);
    let texture =
      canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_get_current_texture(
        context,
      );

    if texture.is_null() {
      return;
    }
    unsafe {
      canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_present_surface(
        context, texture,
      )
    }
  }

  #[napi]
  pub fn configure(&self, options: GPUCanvasContextConfigOptions) {
    let mut alphaMode = canvas_c::webgpu::gpu_canvas_context::CanvasGPUSurfaceAlphaMode::Auto;
    let mut presentMode = canvas_c::webgpu::gpu_canvas_context::CanvasGPUPresentMode::Mailbox;
    if let Some(alpha_mode) = options.alpha_mode {
      alphaMode = match alpha_mode {
        GPUCanvasAlphaMode::opaque => {
          canvas_c::webgpu::gpu_canvas_context::CanvasGPUSurfaceAlphaMode::Opaque
        }
        GPUCanvasAlphaMode::premultiplied => {
          canvas_c::webgpu::gpu_canvas_context::CanvasGPUSurfaceAlphaMode::PreMultiplied
        }
        GPUCanvasAlphaMode::postmultiplied => {
          canvas_c::webgpu::gpu_canvas_context::CanvasGPUSurfaceAlphaMode::PostMultiplied
        }
        GPUCanvasAlphaMode::inherit => {
          canvas_c::webgpu::gpu_canvas_context::CanvasGPUSurfaceAlphaMode::Inherit
        }
      };
    }

    if let Some(present_mode) = options.present_mode {
      presentMode = match present_mode {
        GPUCanvasPresentMode::autoVsync => {
          canvas_c::webgpu::gpu_canvas_context::CanvasGPUPresentMode::AutoVsync
        }
        GPUCanvasPresentMode::autoNoVsync => {
          canvas_c::webgpu::gpu_canvas_context::CanvasGPUPresentMode::AutoNoVsync
        }
        GPUCanvasPresentMode::fifo => {
          canvas_c::webgpu::gpu_canvas_context::CanvasGPUPresentMode::Fifo
        }
        GPUCanvasPresentMode::fifoRelaxed => {
          canvas_c::webgpu::gpu_canvas_context::CanvasGPUPresentMode::FifoRelaxed
        }
        GPUCanvasPresentMode::immediate => {
          canvas_c::webgpu::gpu_canvas_context::CanvasGPUPresentMode::Immediate
        }
        GPUCanvasPresentMode::mailbox => {
          canvas_c::webgpu::gpu_canvas_context::CanvasGPUPresentMode::Mailbox
        }
      };
    }

    let mut view_formats_ptr: *const canvas_c::webgpu::enums::CanvasGPUTextureFormat =
      std::ptr::null();
    let mut view_formats_size = 0;

    let view_formats = options.view_formats.map(|formats| {
      formats
        .into_iter()
        .map(|format| format.into())
        .collect::<Vec<canvas_c::webgpu::enums::CanvasGPUTextureFormat>>()
    });

    if let Some(view_formats) = &view_formats {
      view_formats_ptr = view_formats.as_ptr();
      view_formats_size = view_formats.len();
    }

    let mut size_ptr: *const canvas_c::webgpu::structs::CanvasExtent3d = std::ptr::null();

    let mut size: Option<canvas_c::webgpu::structs::CanvasExtent3d> = None;

    if let Some(value) = options.size {
      size = Some(match value {
        Either::A(array) => {
          let width = *array.get(0).unwrap_or(&0);
          let height = *array.get(1).unwrap_or(&1);
          let depth_or_array_layers = *array.get(2).unwrap_or(&1);
          canvas_c::webgpu::structs::CanvasExtent3d {
            width,
            height,
            depth_or_array_layers,
          }
        }
        Either::B(dict) => match (dict.height, dict.depth_or_array_layers) {
          (Some(height), Some(depth_or_array_layers)) => {
            canvas_c::webgpu::structs::CanvasExtent3d {
              width: dict.width,
              height,
              depth_or_array_layers,
            }
          }
          (Some(height), None) => canvas_c::webgpu::structs::CanvasExtent3d {
            width: dict.width,
            height,
            depth_or_array_layers: 1,
          },
          (None, Some(depth_or_array_layers)) => canvas_c::webgpu::structs::CanvasExtent3d {
            width: dict.width,
            height: 1,
            depth_or_array_layers,
          },
          (None, None) => canvas_c::webgpu::structs::CanvasExtent3d {
            width: dict.width,
            height: 1,
            depth_or_array_layers: 1,
          },
        },
      });

      if let Some(size) = &size {
        size_ptr = size;
      }
    }

    let format = options
      .format
      .map(|format| canvas_c::webgpu::enums::CanvasOptionalGPUTextureFormat::Some(format.into()))
      .unwrap_or(canvas_c::webgpu::enums::CanvasOptionalGPUTextureFormat::None);

    let usage = options.usage.unwrap_or(
      canvas_c::webgpu::enums::CanvasGPUTextureUsageRenderAttachment
        | canvas_c::webgpu::enums::CanvasGPUTextureUsageCopySrc
        | canvas_c::webgpu::enums::CanvasGPUTextureUsageCopyDst,
    );

    let config = canvas_c::webgpu::gpu_canvas_context::CanvasGPUSurfaceConfiguration {
      alphaMode,
      usage,
      presentMode,
      view_formats: view_formats_ptr,
      view_formats_size,
      size: size_ptr,
      format,
    };
    unsafe {
      canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_configure(
        Arc::as_ptr(&self.context),
        Arc::as_ptr(&options.device.device),
        &config,
      );
    }
  }
  #[napi]
  pub fn unconfigure(&self) {
    unsafe {
      canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_unconfigure(Arc::as_ptr(
        &self.context,
      ))
    }
  }
  #[napi]
  pub fn get_current_texture(&self) -> Option<g_p_u_texture> {
    let ret =
      canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_get_current_texture(
        Arc::as_ptr(&self.context),
      );

    if ret.is_null() {
      None
    } else {
      Some(g_p_u_texture {
        texture: unsafe { Arc::from_raw(ret) },
      })
    }
  }

  #[napi(
    ts_return_type = "{usages: number, formats: string[], alphaModes: string[], presentModes: string[]}"
  )]
  pub fn get_capabilities(&self, adapter: &g_p_u_adapter) -> CanvasSurfaceCapabilities {
    let cap = unsafe {
      canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_get_capabilities(
        Arc::as_ptr(&self.context),
        Arc::as_ptr(&adapter.adapter),
      )
    };
    let cap = unsafe { *Box::from_raw(cap) };
    CanvasSurfaceCapabilities::from(cap)
  }

  #[napi(js_name = "toDataURL")]
  pub fn to_data_url(&self, format: Option<String>, encoderOptions: Option<f64>) -> String {
    let c_str = CString::new(format.unwrap_or("image/png".to_string())).unwrap();
    let quality = encoderOptions
      .map(|v| v as f32)
      .unwrap_or(0.92)
      .try_into()
      .unwrap_or(0.92);
    let quality: u32 = (quality * 100.) as u32;
    let has_texture =
      canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_has_current_texture(
        Arc::as_ptr(&self.context),
      );
    let ret = unsafe {
      if has_texture {
        let texture =
          canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_context_get_current_texture(
            Arc::as_ptr(&self.context),
          );
        let ret =
          canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_to_data_url_with_texture(
            Arc::as_ptr(&self.context),
            texture,
            c_str.as_ptr(),
            quality,
          );
        canvas_c::webgpu::gpu_texture::canvas_native_webgpu_texture_release(texture);
        ret
      } else {
        canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_to_data_url(
          Arc::as_ptr(&self.context),
          c_str.as_ptr(),
          quality,
        )
      }
    };
    if ret.is_null() {
      return "data:,".to_string();
    }
    unsafe { CString::from_raw(ret as _).to_string_lossy().to_string() }
  }
}
