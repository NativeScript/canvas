pub use wgpu_core;
pub use wgpu_types;


#[macro_use]
mod macros {
    macro_rules! gfx_select {
        ($id:expr => $p0:ident.$p1:tt.$method:ident $params:tt) => {
          gfx_select!($id => {$p0.$p1}, $method $params)
        };

        ($id:expr => $p0:ident.$method:ident $params:tt) => {
          gfx_select!($id => {$p0}, $method $params)
        };

        ($id:expr => {$($c:tt)*}, $method:ident $params:tt) => {
          match $id.backend() {
            #[cfg(target_os = "android")]
            wgpu_types::Backend::Vulkan => $($c)*.$method::<wgpu_core::api::Vulkan> $params,
            #[cfg(any(target_os = "ios", target_os = "macos"))]
            wgpu_types::Backend::Metal => $($c)*.$method::<wgpu_core::api::Metal> $params,
            #[cfg(windows)]
            wgpu_types::Backend::Dx12 => $($c)*.$method::<wgpu_core::api::Dx12> $params,
            // #[cfg(not(target_os = "ios", target_os = "macos"))]
            // wgpu_types::Backend::Gl => $($c)*.$method::<wgpu_core::api::Gles> $params,
            other => panic!("Unexpected backend {:?}", other),
          }
        };
      }
}



pub mod gpu;
pub mod gpu_adapter_info;
pub mod gpu_adapter;
pub mod gpu_buffer;
pub mod gpu_command_encoder;
pub mod gpu_compute_pass;
pub mod gpu_device;
pub mod gpu_query_set;
pub mod gpu_queue;
pub mod gpu_supported_limits;
pub mod prelude;
pub mod gpu_render_pass_encoder;
pub mod gpu_shader_module;