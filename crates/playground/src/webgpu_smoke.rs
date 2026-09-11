//! One-shot exercise of the WebGPU paths the spinning-cube demo never reaches.
//!
//! The cube covers render pipelines, render passes, surface acquire/present and
//! queue submit. It does not touch compute, render bundles, `toDataURL`
//! readback, or `copyExternalImageToTexture`, which left those compile-verified
//! only after the wgpu `Global` removal. This runs each of them once against a
//! live device and prints a PASS/FAIL line, so a single `playground` run says
//! whether they actually work rather than merely link.
//!
//! Results go to stdout; validation errors reach stdout too, via the demo's
//! uncaptured-error handler.

use std::ffi::{CStr, CString};
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};

use canvas_c::webgpu::enums::{
    CanvasBindGroupEntry, CanvasBindGroupEntryResource, CanvasBufferBinding, CanvasGPUTextureFormat,
    CanvasGPUTextureUsageCopyDst, CanvasGPUTextureUsageCopySrc,
    CanvasGPUTextureUsageRenderAttachment, CanvasGPUTextureUsageTextureBinding,
    CanvasOptionalGPUTextureFormat, CanvasTextureAspect,
};
use canvas_c::webgpu::gpu_buffer::{
    canvas_native_webgpu_buffer_get_mapped_range, canvas_native_webgpu_buffer_map_async,
    canvas_native_webgpu_buffer_unmap,
};
use canvas_c::webgpu::gpu_canvas_context::canvas_native_webgpu_to_data_url;
use canvas_c::webgpu::gpu_command_encoder::{
    canvas_native_webgpu_command_encoder_begin_compute_pass,
    canvas_native_webgpu_command_encoder_copy_buffer_to_buffer,
    canvas_native_webgpu_command_encoder_finish,
};
use canvas_c::webgpu::gpu_compute_pass_encoder::{
    canvas_native_webgpu_compute_pass_encoder_dispatch_workgroups,
    canvas_native_webgpu_compute_pass_encoder_end,
    canvas_native_webgpu_compute_pass_encoder_set_bind_group,
    canvas_native_webgpu_compute_pass_encoder_set_pipeline,
};
use canvas_c::webgpu::gpu_device::{
    canvas_native_webgpu_device_create_buffer, canvas_native_webgpu_device_create_command_encoder,
    canvas_native_webgpu_device_create_compute_pipeline,
    canvas_native_webgpu_device_create_shader_module, CanvasGPUAutoLayoutMode,
    CanvasGPUPipelineLayoutOrGPUAutoLayoutMode, CanvasProgrammableStage,
};
use canvas_c::webgpu::gpu_queue::canvas_native_webgpu_queue_submit;

use canvas_c::webgpu::gpu_canvas_context::CanvasGPUCanvasContext;
use canvas_c::webgpu::gpu_device::CanvasGPUDevice;
use canvas_c::webgpu::gpu_queue::CanvasGPUQueue;

static RAN: AtomicBool = AtomicBool::new(false);

const COMPUTE_SHADER: &str = r#"
@group(0) @binding(0)
var<storage, read_write> values: array<u32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    if (id.x < arrayLength(&values)) {
        values[id.x] = values[id.x] * 2u + 1u;
    }
}
"#;

const ELEMENTS: u64 = 64;
const BUFFER_BYTES: u64 = ELEMENTS * 4;

fn report(name: &str, outcome: Result<String, String>) {
    match outcome {
        Ok(detail) => println!("webgpu_smoke: PASS {name} ({detail})"),
        Err(reason) => println!("webgpu_smoke: FAIL {name} -- {reason}"),
    }
}

/// Runs once per process; later calls are no-ops so the per-frame render loop
/// can call this unconditionally.
pub unsafe fn run_once(
    device: *const CanvasGPUDevice,
    queue: *const CanvasGPUQueue,
    context: *const CanvasGPUCanvasContext,
) {
    if RAN.swap(true, Ordering::SeqCst) {
        return;
    }

    println!("webgpu_smoke: exercising the paths the cube demo does not reach");

    report("compute pass", compute_roundtrip(device, queue));
    report("render bundle", render_bundle(device, queue));
    report("copyExternalImageToTexture", external_image(device, queue));
    report("toDataURL readback", data_url(context));
}

/// Dispatches `values[i] = values[i] * 2 + 1` over a storage buffer, copies the
/// result into a MAP_READ buffer and checks the numbers came back. This covers
/// create_compute_pipeline, begin_compute_pass, set_bind_group, set_pipeline,
/// dispatch_workgroups, end, copy_buffer_to_buffer, finish, submit, map_async
/// and get_mapped_range in one pass.
unsafe fn compute_roundtrip(
    device: *const CanvasGPUDevice,
    queue: *const CanvasGPUQueue,
) -> Result<String, String> {
    let shader_text = CString::new(COMPUTE_SHADER).map_err(|_| "shader CString".to_string())?;
    let module =
        canvas_native_webgpu_device_create_shader_module(device, ptr::null(), shader_text.as_ptr());
    if module.is_null() {
        return Err("compute shader module is null".into());
    }

    let entry = c"main";
    let stage = CanvasProgrammableStage {
        module,
        entry_point: entry.as_ptr(),
        constants: ptr::null(),
    };

    let pipeline = canvas_native_webgpu_device_create_compute_pipeline(
        device,
        ptr::null(),
        CanvasGPUPipelineLayoutOrGPUAutoLayoutMode::Auto(CanvasGPUAutoLayoutMode::Auto),
        &stage,
    );
    if pipeline.is_null() {
        return Err("compute pipeline is null".into());
    }

    // STORAGE | COPY_SRC | COPY_DST
    let storage = canvas_native_webgpu_device_create_buffer(
        device,
        ptr::null(),
        BUFFER_BYTES,
        0x80 | 0x4 | 0x8,
        false,
    );
    // MAP_READ | COPY_DST
    let readback =
        canvas_native_webgpu_device_create_buffer(device, ptr::null(), BUFFER_BYTES, 0x1 | 0x8, false);
    if storage.is_null() || readback.is_null() {
        return Err("buffer creation returned null".into());
    }

    // Seed the storage buffer with 0..ELEMENTS so the result is checkable.
    let seed: Vec<u32> = (0..ELEMENTS as u32).collect();
    canvas_c::webgpu::gpu_queue::canvas_native_webgpu_queue_write_buffer(
        queue,
        storage,
        0,
        seed.as_ptr() as *const u8,
        BUFFER_BYTES as usize,
        0,
    );

    let layout =
        canvas_c::webgpu::gpu_compute_pipeline::canvas_native_webgpu_compute_pipeline_get_bind_group_layout(
            pipeline, 0,
        );
    if layout.is_null() {
        return Err("bind group layout is null".into());
    }

    let entries = [CanvasBindGroupEntry {
        binding: 0,
        resource: CanvasBindGroupEntryResource::Buffer(CanvasBufferBinding {
            buffer: storage,
            offset: 0,
            size: -1,
        }),
    }];

    let bind_group = canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_create_bind_group(
        device,
        ptr::null(),
        layout,
        entries.as_ptr(),
        entries.len(),
    );
    if bind_group.is_null() {
        return Err("bind group is null".into());
    }

    let encoder = canvas_native_webgpu_device_create_command_encoder(device, ptr::null());
    if encoder.is_null() {
        return Err("command encoder is null".into());
    }

    let pass = canvas_native_webgpu_command_encoder_begin_compute_pass(
        encoder,
        ptr::null(),
        ptr::null(),
        -1,
        -1,
    );
    if pass.is_null() {
        return Err("compute pass encoder is null".into());
    }

    canvas_native_webgpu_compute_pass_encoder_set_pipeline(pass, pipeline);
    canvas_native_webgpu_compute_pass_encoder_set_bind_group(
        pass,
        0,
        bind_group,
        ptr::null(),
        0,
        0,
        0,
    );
    canvas_native_webgpu_compute_pass_encoder_dispatch_workgroups(pass, 1, 1, 1);
    canvas_native_webgpu_compute_pass_encoder_end(pass);

    canvas_native_webgpu_command_encoder_copy_buffer_to_buffer(
        encoder,
        storage,
        0,
        readback,
        0,
        BUFFER_BYTES as i64,
    );

    let command_buffer = canvas_native_webgpu_command_encoder_finish(encoder, ptr::null());
    if command_buffer.is_null() {
        return Err("command buffer is null".into());
    }

    let buffers = [command_buffer];
    canvas_native_webgpu_queue_submit(queue, buffers.as_ptr(), buffers.len());

    // map_async completes off the submit; the poller started by the binding
    // layer drives it, so just wait for the mapping to land.
    // Wait on the map callback rather than polling get_mapped_range: calling it
    // before the mapping lands is itself a validation error, which would show up
    // in the log as a failure of the thing we are testing.
    extern "C" fn on_mapped(
        error: canvas_c::webgpu::error::CanvasGPUErrorType,
        _message: *mut std::os::raw::c_char,
        data: *mut std::ffi::c_void,
    ) {
        let done = unsafe { &*(data as *const (AtomicBool, AtomicBool)) };
        done.1.store(
            matches!(error, canvas_c::webgpu::error::CanvasGPUErrorType::None),
            Ordering::SeqCst,
        );
        done.0.store(true, Ordering::SeqCst);
    }

    let signal: (AtomicBool, AtomicBool) = (AtomicBool::new(false), AtomicBool::new(false));
    canvas_native_webgpu_buffer_map_async(
        readback,
        canvas_c::webgpu::gpu_buffer::GPUMapMode::Read,
        0,
        BUFFER_BYTES as i64,
        on_mapped,
        &signal as *const _ as *mut std::ffi::c_void,
    );

    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    while !signal.0.load(Ordering::SeqCst) {
        if std::time::Instant::now() > deadline {
            return Err("map_async never completed within 5s".into());
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    if !signal.1.load(Ordering::SeqCst) {
        return Err("map_async reported an error".into());
    }

    let mapped = canvas_native_webgpu_buffer_get_mapped_range(readback, 0, BUFFER_BYTES as i64);
    if mapped.is_null() {
        return Err("readback buffer mapped but get_mapped_range returned null".into());
    }

    let values = std::slice::from_raw_parts(mapped as *const u32, ELEMENTS as usize);
    let wrong: Vec<(usize, u32, u32)> = values
        .iter()
        .enumerate()
        .filter_map(|(i, got)| {
            let want = (i as u32) * 2 + 1;
            (*got != want).then_some((i, want, *got))
        })
        .collect();

    let first = values[0];
    let last = values[ELEMENTS as usize - 1];
    canvas_native_webgpu_buffer_unmap(readback);

    if let Some((i, want, got)) = wrong.first() {
        return Err(format!(
            "{} of {ELEMENTS} values wrong, first at [{i}]: expected {want}, got {got}",
            wrong.len()
        ));
    }

    Ok(format!(
        "{ELEMENTS} values dispatched and read back, [0]={first} [{}]={last}",
        ELEMENTS - 1
    ))
}

/// Reads the configured surface back through the `toDataURL` path, which runs
/// copy_texture_to_buffer, finish, submit, device poll and a buffer mapping.
unsafe fn data_url(context: *const CanvasGPUCanvasContext) -> Result<String, String> {
    if context.is_null() {
        return Err("no context".into());
    }

    let format = CString::new("image/png").map_err(|_| "format CString".to_string())?;
    let url = canvas_native_webgpu_to_data_url(context, format.as_ptr(), 100);
    if url.is_null() {
        return Err("returned null".into());
    }

    let text = CStr::from_ptr(url).to_string_lossy().into_owned();
    canvas_c::canvas_native_string_destroy(url);

    if !text.starts_with("data:image/png;base64,") {
        let head: String = text.chars().take(32).collect();
        return Err(format!("unexpected prefix: {head:?}"));
    }
    if text.len() < 128 {
        return Err(format!("suspiciously short: {} bytes", text.len()));
    }

    Ok(format!("{} bytes of PNG data URL", text.len()))
}

const OFFSCREEN: u32 = 64;
const TRIANGLE_SHADER: &str = r#"
@vertex
fn main(@builtin(vertex_index) i: u32) -> @builtin(position) vec4f {
    var p = array<vec2f, 3>(vec2f(-1.0, -1.0), vec2f(3.0, -1.0), vec2f(-1.0, 3.0));
    return vec4f(p[i], 0.0, 1.0);
}

@fragment
fn fragment_main() -> @location(0) vec4f {
    return vec4f(0.0, 1.0, 0.0, 1.0);
}
"#;

/// Records a fullscreen triangle into a render bundle, replays it into an
/// offscreen render pass, then reads the target back and checks the pixels are
/// the colour the bundle writes. Covers create_render_bundle_encoder,
/// set_pipeline, draw, finish and execute_bundles.
unsafe fn render_bundle(
    device: *const CanvasGPUDevice,
    queue: *const CanvasGPUQueue,
) -> Result<String, String> {
    let target = make_texture(
        device,
        CanvasGPUTextureUsageRenderAttachment | CanvasGPUTextureUsageCopySrc,
    )?;

    let shader_text = CString::new(TRIANGLE_SHADER).map_err(|_| "shader CString".to_string())?;
    let module =
        canvas_native_webgpu_device_create_shader_module(device, ptr::null(), shader_text.as_ptr());
    if module.is_null() {
        return Err("triangle shader module is null".into());
    }

    let pipeline = make_triangle_pipeline(device, module)?;

    let color_formats = [CanvasGPUTextureFormat::Rgba8Unorm];
    let bundle_desc =
        canvas_c::webgpu::gpu_device::CanvasCreateRenderBundleEncoderDescriptor {
            label: ptr::null(),
            color_formats: color_formats.as_ptr(),
            color_formats_size: color_formats.len(),
            depth_stencil_format: CanvasOptionalGPUTextureFormat::None,
            sample_count: 1,
            depth_read_only: false,
            stencil_read_only: false,
        };

    let bundle_encoder =
        canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_create_render_bundle_encoder(
            device, &bundle_desc,
        );
    if bundle_encoder.is_null() {
        return Err("render bundle encoder is null".into());
    }

    canvas_c::webgpu::gpu_render_bundle_encoder::canvas_native_webgpu_render_bundle_encoder_set_pipeline(
        bundle_encoder, pipeline,
    );
    canvas_c::webgpu::gpu_render_bundle_encoder::canvas_native_webgpu_render_bundle_encoder_draw(
        bundle_encoder, 3, 1, 0, 0,
    );

    let bundle =
        canvas_c::webgpu::gpu_render_bundle_encoder::canvas_native_webgpu_render_bundle_encoder_finish(
            bundle_encoder, ptr::null(),
        );
    if bundle.is_null() {
        return Err("finished render bundle is null".into());
    }

    let view = canvas_c::webgpu::gpu_texture::canvas_native_webgpu_texture_create_texture_view(
        target,
        ptr::null(),
    );
    if view.is_null() {
        return Err("target texture view is null".into());
    }

    let encoder = canvas_native_webgpu_device_create_command_encoder(device, ptr::null());
    let attachments = [canvas_c::webgpu::structs::CanvasRenderPassColorAttachment {
        view,
        resolve_target: ptr::null(),
        channel: canvas_c::webgpu::structs::CanvasPassChannelColor {
            load_op: canvas_c::webgpu::structs::CanvasLoadOp::Clear,
            store_op: canvas_c::webgpu::structs::CanvasStoreOp::Store,
            clear_value: canvas_c::webgpu::structs::CanvasOptionalColor::Some(
                canvas_c::webgpu::structs::CanvasColor {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                },
            ),
            read_only: false,
        },
    }];

    let pass = canvas_c::webgpu::gpu_command_encoder::canvas_native_webgpu_command_encoder_begin_render_pass(
        encoder,
        ptr::null(),
        attachments.as_ptr(),
        attachments.len(),
        ptr::null(),
        ptr::null(),
        ptr::null(),
        -1,
        -1,
    );
    if pass.is_null() {
        return Err("render pass encoder is null".into());
    }

    let bundles = [bundle];
    canvas_c::webgpu::gpu_render_pass_encoder::canvas_native_webgpu_render_pass_encoder_execute_bundles(
        pass,
        bundles.as_ptr(),
        bundles.len(),
    );
    canvas_c::webgpu::gpu_render_pass_encoder::canvas_native_webgpu_render_pass_encoder_end(pass);

    let pixels = readback_texture(device, queue, encoder, target)?;

    // The bundle paints the whole target green over a red clear, so a red
    // pixel means the bundle never replayed.
    let (r, g, b, a) = (pixels[0], pixels[1], pixels[2], pixels[3]);
    if (r, g, b, a) != (0, 255, 0, 255) {
        return Err(format!(
            "top-left pixel is rgba({r},{g},{b},{a}), expected the bundle's green (0,255,0,255)"
        ));
    }

    Ok(format!("{OFFSCREEN}x{OFFSCREEN} target painted by a replayed bundle"))
}

/// Uploads a known RGBA image through copyExternalImageToTexture and reads the
/// texture back to confirm the bytes landed.
unsafe fn external_image(
    device: *const CanvasGPUDevice,
    queue: *const CanvasGPUQueue,
) -> Result<String, String> {
    let texture = make_texture(
        device,
        CanvasGPUTextureUsageCopyDst
            | CanvasGPUTextureUsageCopySrc
            | CanvasGPUTextureUsageTextureBinding,
    )?;

    // Solid magenta so a mismatch is obvious against both black and the
    // red/green used by the bundle test.
    let pixel = [255u8, 0, 255, 255];
    let source: Vec<u8> = pixel
        .iter()
        .cycle()
        .take((OFFSCREEN * OFFSCREEN * 4) as usize)
        .copied()
        .collect();

    let copy_source = canvas_c::webgpu::structs::CanvasImageCopyExternalImage {
        source: source.as_ptr(),
        source_size: source.len(),
        origin: canvas_c::webgpu::structs::CanvasOrigin2d { x: 0, y: 0 },
        flip_y: false,
        width: OFFSCREEN,
        height: OFFSCREEN,
    };

    let destination = canvas_c::webgpu::gpu_command_encoder::CanvasImageCopyTexture {
        texture,
        mip_level: 0,
        origin: canvas_c::webgpu::structs::CanvasOrigin3d { x: 0, y: 0, z: 0 },
        aspect: CanvasTextureAspect::All,
    };

    let size = canvas_c::webgpu::structs::CanvasExtent3d {
        width: OFFSCREEN,
        height: OFFSCREEN,
        depth_or_array_layers: 1,
    };

    canvas_c::webgpu::gpu_queue::canvas_native_webgpu_queue_copy_external_image_to_texture(
        queue,
        &copy_source,
        &destination,
        &size,
    );

    let encoder = canvas_native_webgpu_device_create_command_encoder(device, ptr::null());
    let pixels = readback_texture(device, queue, encoder, texture)?;

    let (r, g, b, a) = (pixels[0], pixels[1], pixels[2], pixels[3]);
    if (r, g, b, a) != (255, 0, 255, 255) {
        return Err(format!(
            "top-left pixel is rgba({r},{g},{b},{a}), expected the uploaded magenta (255,0,255,255)"
        ));
    }

    Ok(format!("{OFFSCREEN}x{OFFSCREEN} upload verified by readback"))
}

/// A small RGBA8 offscreen texture with the given usage flags.
unsafe fn make_texture(
    device: *const CanvasGPUDevice,
    usage: u32,
) -> Result<*const canvas_c::webgpu::gpu_texture::CanvasGPUTexture, String> {
    let desc = canvas_c::webgpu::gpu_device::CanvasCreateTextureDescriptor {
        label: ptr::null(),
        dimension: canvas_c::webgpu::enums::CanvasTextureDimension::D2,
        format: CanvasGPUTextureFormat::Rgba8Unorm,
        mipLevelCount: 1,
        sampleCount: 1,
        width: OFFSCREEN,
        height: OFFSCREEN,
        depthOrArrayLayers: 1,
        usage,
        view_formats: ptr::null(),
        view_formats_size: 0,
    };
    let texture = canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_create_texture(device, &desc);
    if texture.is_null() {
        return Err("texture creation returned null".into());
    }
    Ok(texture)
}

/// Fullscreen-triangle pipeline targeting Rgba8Unorm, used by the bundle test.
unsafe fn make_triangle_pipeline(
    device: *const CanvasGPUDevice,
    module: *const canvas_c::webgpu::gpu_shader_module::CanvasGPUShaderModule,
) -> Result<*const canvas_c::webgpu::gpu_render_pipeline::CanvasGPURenderPipeline, String> {
    let vertex_entry = c"main";
    let fragment_entry = c"fragment_main";

    let targets = [canvas_c::webgpu::structs::CanvasColorTargetState {
        format: CanvasGPUTextureFormat::Rgba8Unorm,
        blend: canvas_c::webgpu::structs::CanvasOptionalBlendState::None,
        write_mask: 0xF,
    }];

    let fragment = canvas_c::webgpu::gpu_device::CanvasFragmentState {
        targets: targets.as_ptr(),
        targets_size: targets.len(),
        module,
        entry_point: fragment_entry.as_ptr(),
        constants: ptr::null(),
    };

    let vertex = canvas_c::webgpu::gpu_device::CanvasVertexState {
        module,
        entry_point: vertex_entry.as_ptr(),
        constants: ptr::null(),
        buffers: ptr::null(),
        buffers_size: 0,
    };

    let primitive = canvas_c::webgpu::gpu_device::CanvasPrimitiveState {
        topology: canvas_c::webgpu::enums::CanvasOptionalPrimitiveTopology::None,
        strip_index_format: canvas_c::webgpu::enums::CanvasOptionalIndexFormat::None,
        front_face: canvas_c::webgpu::enums::CanvasFrontFace::Ccw,
        cull_mode: canvas_c::webgpu::enums::CanvasCullMode::None,
        unclipped_depth: false,
    };

    let desc = canvas_c::webgpu::gpu_device::CanvasCreateRenderPipelineDescriptor {
        label: ptr::null(),
        layout: CanvasGPUPipelineLayoutOrGPUAutoLayoutMode::Auto(CanvasGPUAutoLayoutMode::Auto),
        vertex: &vertex,
        primitive: &primitive,
        depth_stencil: ptr::null(),
        multisample: ptr::null(),
        fragment: &fragment,
    };

    let pipeline =
        canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_create_render_pipeline(device, &desc);
    if pipeline.is_null() {
        return Err("render pipeline is null".into());
    }
    Ok(pipeline)
}

/// Copies a texture into a MAP_READ buffer, submits, waits for the mapping and
/// returns the first row's bytes. `encoder` is consumed by the submit.
unsafe fn readback_texture(
    device: *const CanvasGPUDevice,
    queue: *const CanvasGPUQueue,
    encoder: *const canvas_c::webgpu::gpu_command_encoder::CanvasGPUCommandEncoder,
    texture: *const canvas_c::webgpu::gpu_texture::CanvasGPUTexture,
) -> Result<Vec<u8>, String> {
    // bytes_per_row must be a multiple of 256; 64px * 4 bytes is exactly 256.
    let bytes_per_row = OFFSCREEN * 4;
    let total = (bytes_per_row * OFFSCREEN) as u64;

    // MAP_READ | COPY_DST
    let readback =
        canvas_native_webgpu_device_create_buffer(device, ptr::null(), total, 0x1 | 0x8, false);
    if readback.is_null() {
        return Err("readback buffer is null".into());
    }

    let source = canvas_c::webgpu::gpu_command_encoder::CanvasImageCopyTexture {
        texture,
        mip_level: 0,
        origin: canvas_c::webgpu::structs::CanvasOrigin3d { x: 0, y: 0, z: 0 },
        aspect: CanvasTextureAspect::All,
    };
    let destination = canvas_c::webgpu::gpu_command_encoder::CanvasImageCopyBuffer {
        buffer: readback,
        offset: 0,
        bytes_per_row: bytes_per_row as i32,
        rows_per_image: OFFSCREEN as i32,
    };
    let size = canvas_c::webgpu::structs::CanvasExtent3d {
        width: OFFSCREEN,
        height: OFFSCREEN,
        depth_or_array_layers: 1,
    };

    canvas_c::webgpu::gpu_command_encoder::canvas_native_webgpu_command_encoder_copy_texture_to_buffer(
        encoder,
        &source,
        &destination,
        &size,
    );

    let command_buffer = canvas_native_webgpu_command_encoder_finish(encoder, ptr::null());
    if command_buffer.is_null() {
        return Err("readback command buffer is null".into());
    }
    let buffers = [command_buffer];
    canvas_native_webgpu_queue_submit(queue, buffers.as_ptr(), buffers.len());

    extern "C" fn on_mapped(
        error: canvas_c::webgpu::error::CanvasGPUErrorType,
        _message: *mut std::os::raw::c_char,
        data: *mut std::ffi::c_void,
    ) {
        let done = unsafe { &*(data as *const (AtomicBool, AtomicBool)) };
        done.1.store(
            matches!(error, canvas_c::webgpu::error::CanvasGPUErrorType::None),
            Ordering::SeqCst,
        );
        done.0.store(true, Ordering::SeqCst);
    }

    let signal: (AtomicBool, AtomicBool) = (AtomicBool::new(false), AtomicBool::new(false));
    canvas_native_webgpu_buffer_map_async(
        readback,
        canvas_c::webgpu::gpu_buffer::GPUMapMode::Read,
        0,
        total as i64,
        on_mapped,
        &signal as *const _ as *mut std::ffi::c_void,
    );

    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    while !signal.0.load(Ordering::SeqCst) {
        if std::time::Instant::now() > deadline {
            return Err("texture readback never mapped within 5s".into());
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    if !signal.1.load(Ordering::SeqCst) {
        return Err("texture readback mapping reported an error".into());
    }

    let mapped = canvas_native_webgpu_buffer_get_mapped_range(readback, 0, total as i64);
    if mapped.is_null() {
        return Err("texture readback get_mapped_range returned null".into());
    }
    let row = std::slice::from_raw_parts(mapped as *const u8, bytes_per_row as usize).to_vec();
    canvas_native_webgpu_buffer_unmap(readback);
    Ok(row)
}
