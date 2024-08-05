pub use wgpu_core;
pub use wgt;


// Using this as the wgpu-core returns empty ....
#[macro_use]
pub mod macros {
    #[macro_export]
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    macro_rules! gfx_select {
        ($id:expr => $p0:ident.$p1:tt.$method:ident $params:tt) => {
          gfx_select!($id => {$p0.$p1}, $method $params)
        };

        ($id:expr => $p0:ident.$method:ident $params:tt) => {
          gfx_select!($id => {$p0}, $method $params)
        };

        ($id:expr => {$($c:tt)*}, $method:ident $params:tt) => {
            $($c)*.$method::<wgpu_core::api::Metal> $params
        };
      }

    #[macro_export]
    #[cfg(any(target_os = "android"))]
    macro_rules! gfx_select {
        ($id:expr => $p0:ident.$p1:tt.$method:ident $params:tt) => {
          gfx_select!($id => {$p0.$p1}, $method $params)
        };

        ($id:expr => $p0:ident.$method:ident $params:tt) => {
          gfx_select!($id => {$p0}, $method $params)
        };

        ($id:expr => {$($c:tt)*}, $method:ident $params:tt) => {
             $($c)*.$method::<wgpu_core::api::Vulkan> $params
        };
      }
}



pub mod enums;
pub mod error;
pub mod gpu;
pub mod gpu_adapter;
pub mod gpu_adapter_info;
pub mod gpu_bind_group;
pub mod gpu_bind_group_layout;
pub mod gpu_buffer;
pub mod gpu_canvas_context;
pub mod gpu_command_buffer;
pub mod gpu_command_encoder;
pub mod gpu_compute_pass_encoder;
pub mod gpu_compute_pipeline;
pub mod gpu_device;
pub mod gpu_pipeline_layout;
pub mod gpu_query_set;
pub mod gpu_queue;
pub mod gpu_render_bundle;
pub mod gpu_render_bundle_encoder;
pub mod gpu_render_pass_encoder;
pub mod gpu_render_pipeline;
pub mod gpu_sampler;
pub mod gpu_shader_module;
pub mod gpu_supported_limits;
pub mod gpu_texture;
pub mod gpu_texture_view;
pub mod prelude;
pub mod structs;
