//! Zero-copy import of a decoded video frame as a GPU texture.
//!
//! `copyExternalImageToTexture` used to take a video frame through the CPU: lock the
//! platform pixel buffer, swizzle BGRA into a freshly allocated staging buffer, hand that
//! to JS as a typed array, then upload it with `write_texture`. At 1080p60 that is roughly
//! 25 MB/frame of memcpy plus an 8 MB allocation and free per frame, all on the thread
//! that called it.
//!
//! Here the frame never leaves the GPU. The platform texture — an `MTLTexture` backed by
//! the same IOSurface the decoder wrote into — is wrapped as a wgpu texture and drawn into
//! the destination.
//!
//! It is a blit rather than a `copy_texture_to_texture` because the source is BGRA and
//! callers overwhelmingly allocate `rgba8unorm` destinations, which are not copy
//! compatible. Sampling sorts out component order for free, and the same pass absorbs
//! `flipY`, the source sub-rect and the destination origin. The WebGPU spec requires
//! `RENDER_ATTACHMENT` usage on `copyExternalImageToTexture` destinations, so the
//! attachment we need is always there.

use std::borrow::Cow;
use std::collections::HashMap;
use std::os::raw::c_void;
use std::sync::Arc;

use super::gpu_command_encoder::CanvasImageCopyTexture;
use super::gpu_queue::CanvasGPUQueue;
use super::structs::CanvasExtent3d;

/// The format the platform hands us for a decoded frame.
///
/// Apple: `AVPlayerItemVideoOutput` is configured for `kCVPixelFormatType_32BGRA` (see
/// `NSCVideoHelper.m`), which `CVMetalTextureCacheCreateTextureFromImage` maps to
/// `MTLPixelFormat.bgra8Unorm`.
///
/// Android: the `ImageReader` feeding the WebGPU path is configured for `RGBA_8888` (see
/// `VideoHelper.setupGpuSurface`), giving an `AHardwareBuffer` that Vulkan imports as
/// `VK_FORMAT_R8G8B8A8_UNORM`.
///
/// Either way the shader samples logical RGBA, so component order needs no fixing up.
#[cfg(any(
    target_os = "ios",
    target_os = "macos",
    target_os = "visionos",
    target_os = "tvos"
))]
const FRAME_FORMAT: wgt::TextureFormat = wgt::TextureFormat::Bgra8Unorm;

#[cfg(target_os = "android")]
const FRAME_FORMAT: wgt::TextureFormat = wgt::TextureFormat::Rgba8Unorm;

/// Fullscreen-triangle blit.
///
/// `uv_transform` is `(scale.x, scale.y, offset.x, offset.y)`, applied to the triangle's
/// UVs. It selects the source sub-rect and, with a negative y scale, performs `flipY`.
const BLIT_WGSL: &str = r#"
struct Blit {
    uv_transform: vec4<f32>,
};

@group(0) @binding(0) var src_texture: texture_2d<f32>;
@group(0) @binding(1) var src_sampler: sampler;
@group(0) @binding(2) var<uniform> blit: Blit;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    // (0,0), (2,0), (0,2) — one oversized triangle covering the viewport.
    let corner = vec2<f32>(f32((vertex_index << 1u) & 2u), f32(vertex_index & 2u));

    var out: VertexOutput;
    // y is negated: texture v grows downwards, clip space y grows upwards.
    out.position = vec4<f32>(corner.x * 2.0 - 1.0, 1.0 - corner.y * 2.0, 0.0, 1.0);
    out.uv = corner * blit.uv_transform.xy + blit.uv_transform.zw;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSampleLevel(src_texture, src_sampler, in.uv, 0.0);
}
"#;

/// Size of the `Blit` uniform: one `vec4<f32>`.
const BLIT_UNIFORM_SIZE: u64 = 16;

/// Lazily built blit resources, cached for the life of the queue.
///
/// Everything but the pipelines is format independent; pipelines are keyed by destination
/// format because the colour target has to match it.
pub(crate) struct BlitCache {
    shader: Arc<wgpu_core::pipeline::ShaderModule>,
    sampler: Arc<wgpu_core::resource::Sampler>,
    uniforms: Arc<wgpu_core::resource::Buffer>,
    pipelines: HashMap<wgt::TextureFormat, Arc<wgpu_core::pipeline::RenderPipeline>>,
}

impl BlitCache {
    fn new(device: &Arc<wgpu_core::device::Device>) -> Self {
        let shader = device.create_shader_module(
            &wgpu_core::pipeline::ShaderModuleDescriptor {
                label: Some(Cow::Borrowed("videoBlit:Shader")),
                runtime_checks: Default::default(),
            },
            wgpu_core::pipeline::ShaderModuleSource::Wgsl(Cow::Borrowed(BLIT_WGSL)),
        );

        let sampler = device.create_sampler(&wgpu_core::resource::SamplerDescriptor {
            label: Some(Cow::Borrowed("videoBlit:Sampler")),
            address_modes: [wgt::AddressMode::ClampToEdge; 3],
            mag_filter: wgt::FilterMode::Linear,
            min_filter: wgt::FilterMode::Linear,
            mipmap_filter: wgt::MipmapFilterMode::Nearest,
            lod_min_clamp: 0.0,
            lod_max_clamp: 0.0,
            compare: None,
            anisotropy_clamp: 1,
            border_color: None,
        });

        let uniforms = device.create_buffer(&wgt::BufferDescriptor {
            label: Some(Cow::Borrowed("videoBlit:Uniforms")),
            size: BLIT_UNIFORM_SIZE,
            usage: wgt::BufferUsages::UNIFORM | wgt::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            shader,
            sampler,
            uniforms,
            pipelines: HashMap::new(),
        }
    }

    fn pipeline(
        &mut self,
        device: &Arc<wgpu_core::device::Device>,
        format: wgt::TextureFormat,
    ) -> Arc<wgpu_core::pipeline::RenderPipeline> {
        if let Some(pipeline) = self.pipelines.get(&format) {
            return Arc::clone(pipeline);
        }

        let descriptor = wgpu_core::pipeline::RenderPipelineDescriptor {
            label: Some(Cow::Borrowed("videoBlit:Pipeline")),
            // `None` means an auto layout derived from the shader, so the bind group
            // layout comes back from `get_bind_group_layout(0)` below.
            layout: None,
            vertex: wgpu_core::pipeline::VertexState {
                stage: wgpu_core::pipeline::ProgrammableStageDescriptor {
                    module: Arc::clone(&self.shader),
                    entry_point: Some(Cow::Borrowed("vs_main")),
                    constants: Default::default(),
                    zero_initialize_workgroup_memory: false,
                },
                buffers: Cow::Borrowed(&[]),
            },
            primitive: wgt::PrimitiveState {
                topology: wgt::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgt::MultisampleState::default(),
            fragment: Some(wgpu_core::pipeline::FragmentState {
                stage: wgpu_core::pipeline::ProgrammableStageDescriptor {
                    module: Arc::clone(&self.shader),
                    entry_point: Some(Cow::Borrowed("fs_main")),
                    constants: Default::default(),
                    zero_initialize_workgroup_memory: false,
                },
                targets: Cow::Owned(vec![Some(wgt::ColorTargetState {
                    format,
                    // The frame replaces whatever is under it; the caller asked for a copy.
                    blend: None,
                    write_mask: wgt::ColorWrites::ALL,
                })]),
            }),
            multiview_mask: None,
            cache: None,
        };

        let pipeline = device.create_render_pipeline(descriptor.into());
        self.pipelines.insert(format, Arc::clone(&pipeline));
        pipeline
    }
}

/// Wrap a platform texture handle as a wgpu texture without copying its contents.
///
/// Returns `None` when the platform cannot hand us an importable texture, which leaves the
/// caller on the CPU upload path rather than dropping the frame.
#[cfg(any(
    target_os = "ios",
    target_os = "macos",
    target_os = "visionos",
    target_os = "tvos"
))]
pub(crate) unsafe fn import_platform_texture(
    device: &Arc<wgpu_core::device::Device>,
    handle: *mut c_void,
    width: u32,
    height: u32,
) -> Option<Arc<wgpu_core::resource::Texture>> {
    use objc2::rc::Retained;
    use objc2::runtime::ProtocolObject;
    use objc2_metal::{MTLTexture, MTLTextureType};

    // The caller owns its reference for the duration of this call only, so take one of our
    // own for the hal texture to hold. `Retained` releases it when the texture is dropped.
    let raw = Retained::retain(handle as *mut ProtocolObject<dyn MTLTexture>)?;

    let hal_texture = wgpu_hal::metal::Device::texture_from_raw(
        raw,
        FRAME_FORMAT,
        MTLTextureType::Type2D,
        1,
        1,
        wgpu_hal::CopyExtent {
            width,
            height,
            depth: 1,
        },
        None,
    );

    let descriptor = wgpu_core::resource::TextureDescriptor {
        label: Some(Cow::Borrowed("videoBlit:Frame")),
        size: wgt::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgt::TextureDimension::D2,
        format: FRAME_FORMAT,
        usage: wgt::TextureUsages::TEXTURE_BINDING,
        view_formats: vec![],
    };

    let (texture, error) = device.create_texture_from_hal(
        Box::new(hal_texture),
        &descriptor,
        wgt::TextureUses::RESOURCE,
        // `cleared: true` — the decoder already wrote the frame. Saying otherwise marks
        // the texture uninitialized, and wgpu would zero it before the first read.
        true,
    );

    if let Some(error) = error {
        log::error!("copyExternalImageToTexture: importing video frame failed: {error:?}");
        return None;
    }

    Some(texture)
}

/// Wrap an `AHardwareBuffer` as a wgpu texture without copying its contents.
///
/// `handle` is an `AHardwareBuffer *`. The caller keeps its reference for the duration of
/// the call; the Vulkan import takes one of its own, so the caller may release theirs as
/// soon as this returns.
#[cfg(target_os = "android")]
pub(crate) unsafe fn import_platform_texture(
    device: &Arc<wgpu_core::device::Device>,
    handle: *mut c_void,
    width: u32,
    height: u32,
) -> Option<Arc<wgpu_core::resource::Texture>> {
    let size = wgt::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };

    let hal_descriptor = wgpu_hal::TextureDescriptor {
        label: Some("videoBlit:Frame"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgt::TextureDimension::D2,
        format: FRAME_FORMAT,
        usage: wgt::TextureUses::RESOURCE,
        memory_flags: wgpu_hal::MemoryFlags::empty(),
        view_formats: vec![],
    };

    let hal_texture = {
        let hal_device = Arc::clone(device).as_hal::<wgpu_hal::api::Vulkan>()?;

        match hal_device.texture_from_android_hardware_buffer(handle.cast(), &hal_descriptor) {
            Ok(texture) => texture,
            Err(error) => {
                log::error!("copyExternalImageToTexture: importing video frame failed: {error:?}");
                return None;
            }
        }
    };

    let descriptor = wgpu_core::resource::TextureDescriptor {
        label: Some(Cow::Borrowed("videoBlit:Frame")),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgt::TextureDimension::D2,
        format: FRAME_FORMAT,
        usage: wgt::TextureUsages::TEXTURE_BINDING,
        view_formats: vec![],
    };

    let (texture, error) = device.create_texture_from_hal(
        Box::new(hal_texture),
        &descriptor,
        // The imported image really is in VK_IMAGE_LAYOUT_UNDEFINED — say so, and wgpu
        // emits the transition to shader-read before the blit samples it. Claiming
        // RESOURCE instead would skip that barrier and sample an undefined layout.
        wgt::TextureUses::UNINITIALIZED,
        // `cleared: true` — the decoder already wrote the frame. Saying otherwise marks
        // the texture uninitialized, and wgpu would zero it before the first read.
        true,
    );

    if let Some(error) = error {
        log::error!("copyExternalImageToTexture: importing video frame failed: {error:?}");
        return None;
    }

    Some(texture)
}

/// Platforms with no zero-copy path: callers stay on the CPU upload path.
#[cfg(not(any(
    target_os = "ios",
    target_os = "macos",
    target_os = "visionos",
    target_os = "tvos",
    target_os = "android"
)))]
pub(crate) unsafe fn import_platform_texture(
    _device: &Arc<wgpu_core::device::Device>,
    _handle: *mut c_void,
    _width: u32,
    _height: u32,
) -> Option<Arc<wgpu_core::resource::Texture>> {
    None
}

/// Blit an imported frame into `destination`, entirely on the GPU.
///
/// Returns `false` if the frame could not be imported or the blit could not be set up, in
/// which case the caller should fall back to the CPU upload path.
unsafe fn blit_frame_into_texture(
    queue: &CanvasGPUQueue,
    handle: *mut c_void,
    frame_width: u32,
    frame_height: u32,
    source_origin_x: u32,
    source_origin_y: u32,
    flip_y: bool,
    destination: &CanvasImageCopyTexture,
    size: wgt::Extent3d,
) -> bool {
    let device = &queue.device_id;

    let Some(source_texture) = import_platform_texture(device, handle, frame_width, frame_height)
    else {
        return false;
    };

    let destination_texture = &*destination.texture;
    let destination_descriptor = destination_texture.texture.descriptor();
    let destination_format = destination_descriptor.format;

    // The blit renders into the destination, so it needs RENDER_ATTACHMENT. The WebGPU
    // spec requires it on `copyExternalImageToTexture` destinations, but a texture made
    // without it would only produce a validation error here — refuse instead, so the
    // caller can fall back to the upload path that just needs COPY_DST.
    if !destination_descriptor
        .usage
        .contains(wgt::TextureUsages::RENDER_ATTACHMENT)
    {
        return false;
    }

    let source_view = source_texture.create_view(&wgpu_core::resource::TextureViewDescriptor {
        label: Some(Cow::Borrowed("videoBlit:FrameView")),
        ..Default::default()
    });

    // Render into the destination's requested mip level and array layer only.
    let destination_view =
        destination_texture
            .texture
            .create_view(&wgpu_core::resource::TextureViewDescriptor {
                label: Some(Cow::Borrowed("videoBlit:DestinationView")),
                dimension: Some(wgt::TextureViewDimension::D2),
                range: wgt::ImageSubresourceRange {
                    aspect: wgt::TextureAspect::All,
                    base_mip_level: destination.mip_level,
                    mip_level_count: Some(1),
                    base_array_layer: destination.origin.z,
                    array_layer_count: Some(1),
                },
                ..Default::default()
            });

    // UV transform selecting the source sub-rect, with flipY folded into the y scale.
    let frame_width = frame_width as f32;
    let frame_height = frame_height as f32;
    let scale_x = size.width as f32 / frame_width;
    let scale_y = size.height as f32 / frame_height;
    let offset_x = source_origin_x as f32 / frame_width;
    let offset_y = source_origin_y as f32 / frame_height;

    let uv_transform: [f32; 4] = if flip_y {
        [scale_x, -scale_y, offset_x, offset_y + scale_y]
    } else {
        [scale_x, scale_y, offset_x, offset_y]
    };

    let mut cache = queue.blit.lock();
    let cache = cache.get_or_insert_with(|| BlitCache::new(device));

    queue.queue.id.write_buffer(
        Arc::clone(&cache.uniforms),
        0,
        bytemuck_cast(&uv_transform),
    );

    let pipeline = cache.pipeline(device, destination_format);
    let bind_group_layout = pipeline.get_bind_group_layout(0);

    let bind_group = device.create_bind_group(&wgpu_core::binding_model::BindGroupDescriptor {
        label: Some(Cow::Borrowed("videoBlit:BindGroup")),
        layout: bind_group_layout,
        entries: Cow::Owned(vec![
            wgpu_core::binding_model::BindGroupEntry {
                binding: 0,
                resource: wgpu_core::binding_model::BindingResource::TextureView(Arc::clone(
                    &source_view,
                )),
            },
            wgpu_core::binding_model::BindGroupEntry {
                binding: 1,
                resource: wgpu_core::binding_model::BindingResource::Sampler(Arc::clone(
                    &cache.sampler,
                )),
            },
            wgpu_core::binding_model::BindGroupEntry {
                binding: 2,
                resource: wgpu_core::binding_model::BindingResource::Buffer(
                    wgpu_core::binding_model::BufferBinding {
                        buffer: Arc::clone(&cache.uniforms),
                        offset: 0,
                        size: Some(BLIT_UNIFORM_SIZE),
                    },
                ),
            },
        ]),
    });

    let encoder = device.create_command_encoder(&wgt::CommandEncoderDescriptor {
        label: Some(Cow::Borrowed("videoBlit:Encoder")),
    });

    let mut pass = encoder.begin_render_pass(wgpu_core::command::ResolvedRenderPassDescriptor {
        label: Some(Cow::Borrowed("videoBlit:Pass")),
        color_attachments: Cow::Owned(vec![Some(
            wgpu_core::command::RenderPassColorAttachment {
                view: destination_view,
                depth_slice: None,
                resolve_target: None,
                load_op: wgpu_core::command::LoadOp::Load,
                store_op: wgpu_core::command::StoreOp::Store,
            },
        )]),
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });

    // The triangle deliberately overhangs the viewport, so the scissor — not the viewport
    // — is what keeps the write inside the destination rect.
    pass.set_viewport(
        destination.origin.x as f32,
        destination.origin.y as f32,
        size.width as f32,
        size.height as f32,
        0.0,
        1.0,
    );
    pass.set_scissor_rect(
        destination.origin.x,
        destination.origin.y,
        size.width,
        size.height,
    );
    pass.set_pipeline(pipeline);
    pass.set_bind_group(0, Some(bind_group), &[]);
    pass.draw(3, 1, 0, 0);
    pass.end();

    let command_buffer = encoder.finish(&wgt::CommandBufferDescriptor {
        label: Some(Cow::Borrowed("videoBlit:CommandBuffer")),
    });

    queue.queue.id.submit(&[command_buffer]);

    true
}

/// `[f32; 4]` as bytes, without pulling in a dependency for four floats.
fn bytemuck_cast(values: &[f32; 4]) -> &[u8] {
    // SAFETY: `f32` has no padding and no invalid bit patterns, and the returned slice
    // borrows `values`, so it cannot outlive it.
    unsafe { std::slice::from_raw_parts(values.as_ptr() as *const u8, std::mem::size_of_val(values)) }
}

/// Copy a platform video frame texture into `destination` without touching the CPU.
///
/// `native_texture` is an `MTLTexture*` on Apple platforms. The caller keeps it alive for
/// the duration of the call; this function takes its own reference if it needs one.
///
/// Returns `false` when the frame could not be imported, so the caller can fall back to
/// `canvas_native_webgpu_queue_copy_external_image_to_texture`.
#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_queue_copy_native_texture_to_texture(
    queue: *const CanvasGPUQueue,
    native_texture: *mut c_void,
    frame_width: u32,
    frame_height: u32,
    source_origin_x: u32,
    source_origin_y: u32,
    flip_y: bool,
    destination: *const CanvasImageCopyTexture,
    size: *const CanvasExtent3d,
) -> bool {
    if queue.is_null() || native_texture.is_null() || destination.is_null() || size.is_null() {
        return false;
    }

    if frame_width == 0 || frame_height == 0 {
        return false;
    }

    let queue = &*queue;
    let destination = &*destination;

    if destination.texture.is_null() {
        return false;
    }

    let size: wgt::Extent3d = (*size).into();

    if size.width == 0 || size.height == 0 {
        return false;
    }

    // A sub-rect that runs off the frame would sample outside it; leave those to the CPU
    // path, which clamps them.
    if source_origin_x + size.width > frame_width || source_origin_y + size.height > frame_height {
        return false;
    }

    blit_frame_into_texture(
        queue,
        native_texture,
        frame_width,
        frame_height,
        source_origin_x,
        source_origin_y,
        flip_y,
        destination,
        size,
    )
}

/// The `MTLDevice` wgpu is rendering with.
///
/// A frame texture has to come from a `CVMetalTextureCache` built on *this* device. Using
/// `MTLCreateSystemDefaultDevice()` happens to match on iOS, where there is one GPU, but on
/// a multi-GPU Mac it can pick a different device, and a texture from the wrong device
/// cannot be bound.
///
/// The returned `id<MTLDevice>` is borrowed — it stays valid as long as the `CanvasGPUDevice`
/// does. Callers that outlive the call must retain it.
#[cfg(any(
    target_os = "ios",
    target_os = "macos",
    target_os = "visionos",
    target_os = "tvos"
))]
#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_device_get_metal_device(
    device: *const super::gpu_device::CanvasGPUDevice,
) -> *mut c_void {
    use objc2::rc::Retained;

    if device.is_null() {
        return std::ptr::null_mut();
    }

    let device = &*device;

    let Some(hal_device) =
        Arc::clone(&device.device).as_hal::<wgpu_hal::api::Metal>()
    else {
        return std::ptr::null_mut();
    };

    Retained::as_ptr(hal_device.raw_device()) as *mut c_void
}
