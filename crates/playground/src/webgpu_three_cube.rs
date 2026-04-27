use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr;
use raw_window_handle::AppKitWindowHandle;
use canvas_c::webgpu::gpu_canvas_context::{CanvasGPUSurfaceAlphaMode, CanvasGPUSurfaceConfiguration, CanvasGPUPresentMode, canvas_native_webgpu_context_configure, canvas_native_webgpu_context_create_nsview, canvas_native_webgpu_context_get_current_texture, canvas_native_webgpu_context_present_surface};
use canvas_c::webgpu::gpu_command_encoder::{canvas_native_webgpu_command_encoder_begin_render_pass};
use canvas_c::webgpu::gpu_device::{canvas_native_webgpu_device_create_bind_group, canvas_native_webgpu_device_create_buffer, canvas_native_webgpu_device_create_command_encoder, canvas_native_webgpu_device_create_render_pipeline, canvas_native_webgpu_device_create_shader_module, canvas_native_webgpu_device_get_queue, canvas_native_webgpu_device_set_uncaptured_error_callback, CanvasCreateRenderPipelineDescriptor, CanvasFragmentState, CanvasGPUAutoLayoutMode, CanvasGPUPipelineLayoutOrGPUAutoLayoutMode, CanvasPrimitiveState, CanvasVertexState};
use canvas_c::webgpu::gpu_queue::{canvas_native_webgpu_queue_submit, canvas_native_webgpu_queue_write_buffer};
use canvas_c::webgpu::gpu_render_pass_encoder::{canvas_native_webgpu_render_pass_encoder_draw, canvas_native_webgpu_render_pass_encoder_end, canvas_native_webgpu_render_pass_encoder_set_bind_group, canvas_native_webgpu_render_pass_encoder_set_pipeline};
use canvas_c::webgpu::gpu_render_pipeline::canvas_native_webgpu_render_pipeline_get_bind_group_layout;
use canvas_c::webgpu::gpu_texture::canvas_native_webgpu_texture_create_texture_view;
use canvas_c::webgpu::structs::{CanvasColorTargetState, CanvasRenderPassColorAttachment, CanvasPassChannelColor, CanvasLoadOp, CanvasStoreOp, CanvasOptionalBlendState};
use canvas_c::webgpu::enums::{CanvasBindGroupEntry, CanvasBindGroupEntryResource, CanvasBufferBinding, CanvasCullMode, CanvasFrontFace, CanvasGPUTextureFormat, CanvasGPUTextureUsageCopyDst, CanvasGPUTextureUsageCopySrc, CanvasGPUTextureUsageRenderAttachment, CanvasOptionalGPUTextureFormat, CanvasOptionalIndexFormat, CanvasOptionalPrimitiveTopology};
use crate::Data;
use crate::handle_error;

const CUBE_SHADER: &str = r#"
struct Uniforms {
    mvp: mat4x4f,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) color: vec4f,
}

const positions: array<vec4f, 36> = array<vec4f, 36>(
    vec4f(-0.5, -0.5,  0.5, 1.0), vec4f( 0.5, -0.5,  0.5, 1.0), vec4f( 0.5,  0.5,  0.5, 1.0),
    vec4f(-0.5, -0.5,  0.5, 1.0), vec4f( 0.5,  0.5,  0.5, 1.0), vec4f(-0.5,  0.5,  0.5, 1.0),

    vec4f( 0.5, -0.5, -0.5, 1.0), vec4f(-0.5, -0.5, -0.5, 1.0), vec4f(-0.5,  0.5, -0.5, 1.0),
    vec4f( 0.5, -0.5, -0.5, 1.0), vec4f(-0.5,  0.5, -0.5, 1.0), vec4f( 0.5,  0.5, -0.5, 1.0),

    vec4f(-0.5,  0.5,  0.5, 1.0), vec4f( 0.5,  0.5,  0.5, 1.0), vec4f( 0.5,  0.5, -0.5, 1.0),
    vec4f(-0.5,  0.5,  0.5, 1.0), vec4f( 0.5,  0.5, -0.5, 1.0), vec4f(-0.5,  0.5, -0.5, 1.0),

    vec4f(-0.5, -0.5, -0.5, 1.0), vec4f( 0.5, -0.5, -0.5, 1.0), vec4f( 0.5, -0.5,  0.5, 1.0),
    vec4f(-0.5, -0.5, -0.5, 1.0), vec4f( 0.5, -0.5,  0.5, 1.0), vec4f(-0.5, -0.5,  0.5, 1.0),

    vec4f( 0.5, -0.5,  0.5, 1.0), vec4f( 0.5, -0.5, -0.5, 1.0), vec4f( 0.5,  0.5, -0.5, 1.0),
    vec4f( 0.5, -0.5,  0.5, 1.0), vec4f( 0.5,  0.5, -0.5, 1.0), vec4f( 0.5,  0.5,  0.5, 1.0),

    vec4f(-0.5, -0.5, -0.5, 1.0), vec4f(-0.5, -0.5,  0.5, 1.0), vec4f(-0.5,  0.5,  0.5, 1.0),
    vec4f(-0.5, -0.5, -0.5, 1.0), vec4f(-0.5,  0.5,  0.5, 1.0), vec4f(-0.5,  0.5, -0.5, 1.0),
);

@vertex
fn main(@builtin(vertex_index) vertexIndex: u32) -> VertexOutput {
    var output: VertexOutput;
    output.position = uniforms.mvp * positions[vertexIndex];
    output.color = vec4f((positions[vertexIndex].xyz * 0.5) + vec3f(0.5, 0.5, 0.5), 1.0);
    return output;
}

@fragment
fn fragment_main(input: VertexOutput) -> @location(0) vec4f {
    return input.color;
}
"#;

fn multiply_mat4(a: [f32; 16], b: [f32; 16]) -> [f32; 16] {
    let mut result = [0.0_f32; 16];
    for row in 0..4 {
        for col in 0..4 {
            let mut sum = 0.0_f32;
            for i in 0..4 {
                sum += a[row + i * 4] * b[i + col * 4];
            }
            result[row + col * 4] = sum;
        }
    }
    result
}

fn rotation_matrix(yaw: f32, pitch: f32) -> [f32; 16] {
    let (sy, cy) = yaw.sin_cos();
    let (sp, cp) = pitch.sin_cos();
    let rx = [
        1.0, 0.0, 0.0, 0.0,
        0.0, cp, sp, 0.0,
        0.0, -sp, cp, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ];
    let ry = [
        cy, 0.0, -sy, 0.0,
        0.0, 1.0, 0.0, 0.0,
        sy, 0.0, cy, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ];
    multiply_mat4(ry, rx)
}

fn perspective(aspect: f32, fovy_radians: f32, znear: f32, zfar: f32) -> [f32; 16] {
    let f = 1.0 / (fovy_radians * 0.5).tan();
    let nf = 1.0 / (znear - zfar);
    [
        f / aspect, 0.0, 0.0, 0.0,
        0.0, f, 0.0, 0.0,
        0.0, 0.0, (zfar + znear) * nf, -1.0,
        0.0, 0.0, (2.0 * zfar * znear) * nf, 0.0,
    ]
}

fn look_at(eye: [f32; 3], center: [f32; 3], up: [f32; 3]) -> [f32; 16] {
    let f = normalize([center[0] - eye[0], center[1] - eye[1], center[2] - eye[2]]);
    let s = normalize(cross(f, up));
    let u = cross(s, f);

    [
        s[0], u[0], -f[0], 0.0,
        s[1], u[1], -f[1], 0.0,
        s[2], u[2], -f[2], 0.0,
        -dot(s, eye), -dot(u, eye), dot(f, eye), 1.0,
    ]
}

fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn normalize(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len == 0.0 {
        return [0.0, 0.0, 0.0];
    }
    [v[0] / len, v[1] / len, v[2] / len]
}

fn build_model_view_projection(width: u32, height: u32, rotation_angle: f32) -> [f32; 16] {
    let aspect = width as f32 / height as f32;
    let proj = perspective(aspect, 50f32.to_radians(), 0.1, 100.0);
    let view = look_at([0.0, 0.0, 3.0], [0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let model = rotation_matrix(rotation_angle, rotation_angle * 0.5);
    multiply_mat4(proj, multiply_mat4(view, model))
}

static mut ROTATION: f32 = 0.0;

pub unsafe fn render_webgpu_three_cube(data: *mut Data, window: AppKitWindowHandle) {
    let data = &mut *data;
    let device = match data.device {
        Some(device) => device,
        None => {
            println!("webgpu_three_cube: no device available");
            return;
        }
    };

    canvas_native_webgpu_device_set_uncaptured_error_callback(
        device,
        Some(handle_error),
        data as *mut _ as *mut c_void,
    );

    let context = canvas_native_webgpu_context_create_nsview(
        data.instance,
        window.ns_view.as_ptr(),
        data.width,
        data.height,
    );

    if context.is_null() {
        println!("webgpu_three_cube: failed to create WebGPU context");
        return;
    }

    let config = CanvasGPUSurfaceConfiguration {
        alphaMode: CanvasGPUSurfaceAlphaMode::PostMultiplied,
        usage: CanvasGPUTextureUsageRenderAttachment | CanvasGPUTextureUsageCopySrc | CanvasGPUTextureUsageCopyDst,
        presentMode: CanvasGPUPresentMode::Fifo,
        view_formats: ptr::null(),
        view_formats_size: 0,
        size: ptr::null_mut(),
        format: CanvasOptionalGPUTextureFormat::None,
    };

    canvas_native_webgpu_context_configure(context, device, &config);

    let shader_text = CString::new(CUBE_SHADER).expect("CString::new failed");
    let module = canvas_native_webgpu_device_create_shader_module(device, ptr::null(), shader_text.as_ptr());
    if module.is_null() {
        println!("webgpu_three_cube: shader module creation failed");
        return;
    }

    let vertex_entry = c"main";
    let fragment_entry = c"fragment_main";

    let vertex_state = CanvasVertexState {
        module,
        entry_point: vertex_entry.as_ptr(),
        constants: ptr::null(),
        buffers: ptr::null(),
        buffers_size: 0,
    };

    let fragment_targets = [CanvasColorTargetState {
        format: CanvasGPUTextureFormat::Bgra8Unorm,
        blend: CanvasOptionalBlendState::None,
        write_mask: 0xF,
    }];

    let fragment_state = CanvasFragmentState {
        targets: fragment_targets.as_ptr(),
        targets_size: fragment_targets.len(),
        module,
        entry_point: fragment_entry.as_ptr(),
        constants: ptr::null(),
    };

    let primitive = CanvasPrimitiveState {
        topology: CanvasOptionalPrimitiveTopology::None,
        strip_index_format: CanvasOptionalIndexFormat::None,
        front_face: CanvasFrontFace::Ccw,
        cull_mode: CanvasCullMode::None,
        unclipped_depth: false,
    };

    let pipeline_desc = CanvasCreateRenderPipelineDescriptor {
        label: ptr::null(),
        layout: CanvasGPUPipelineLayoutOrGPUAutoLayoutMode::Auto(CanvasGPUAutoLayoutMode::Auto),
        vertex: &vertex_state,
        primitive: &primitive,
        depth_stencil: ptr::null(),
        multisample: ptr::null(),
        fragment: &fragment_state,
    };

    let pipeline = canvas_c::webgpu::gpu_device::canvas_native_webgpu_device_create_render_pipeline(device, &pipeline_desc);
    if pipeline.is_null() {
        println!("webgpu_three_cube: render pipeline creation failed");
        return;
    }

    let bind_group_layout = canvas_native_webgpu_render_pipeline_get_bind_group_layout(pipeline, 0);
    if bind_group_layout.is_null() {
        println!("webgpu_three_cube: bind group layout unavailable");
        return;
    }

    let rotation = {
        ROTATION += 0.05;
        ROTATION
    };
    let mvp = build_model_view_projection(data.width, data.height, rotation);

    // Buffer is used as a uniform binding and updated via write_buffer.
    // COPY_DST is required for queue_write_buffer, and UNIFORM is required for bind group usage.
    let uniform_buffer = canvas_native_webgpu_device_create_buffer(device, ptr::null(), 64, 0x0040 | 0x0008, false);
    if uniform_buffer.is_null() {
        println!("webgpu_three_cube: uniform buffer creation failed");
        return;
    }

    let queue = canvas_native_webgpu_device_get_queue(device);
    canvas_native_webgpu_queue_write_buffer(queue, uniform_buffer, 0, mvp.as_ptr() as *const u8, 64, 0);

    let bind_group_entry = CanvasBindGroupEntry {
        binding: 0,
        resource: CanvasBindGroupEntryResource::Buffer(CanvasBufferBinding {
            buffer: uniform_buffer,
            offset: 0,
            size: 64,
        }),
    };

    let bind_group = canvas_native_webgpu_device_create_bind_group(device, ptr::null(), bind_group_layout, &bind_group_entry, 1);
    if bind_group.is_null() {
        println!("webgpu_three_cube: bind group creation failed");
        return;
    }

    let command_encoder = canvas_native_webgpu_device_create_command_encoder(device, ptr::null());
    if command_encoder.is_null() {
        println!("webgpu_three_cube: command encoder creation failed");
        return;
    }

    let texture = canvas_native_webgpu_context_get_current_texture(context);
    if texture.is_null() {
        println!("webgpu_three_cube: current texture unavailable");
        return;
    }

    let view = canvas_native_webgpu_texture_create_texture_view(texture, ptr::null());
    if view.is_null() {
        println!("webgpu_three_cube: texture view creation failed");
        return;
    }

    let color_attachments = [CanvasRenderPassColorAttachment {
        view,
        resolve_target: ptr::null(),
        channel: CanvasPassChannelColor {
            load_op: CanvasLoadOp::Clear,
            store_op: CanvasStoreOp::Store,
            clear_value: Some(canvas_c::webgpu::structs::CanvasColor { r: 0.1, g: 0.15, b: 0.25, a: 1.0 }).into(),
            read_only: false,
        },
    }];

    let render_pass = canvas_native_webgpu_command_encoder_begin_render_pass(
        command_encoder,
        ptr::null(),
        color_attachments.as_ptr(),
        color_attachments.len(),
        ptr::null(),
        ptr::null(),
        ptr::null(),
        -1,
        -1,
    );

    canvas_native_webgpu_render_pass_encoder_set_pipeline(render_pass, pipeline);
    canvas_native_webgpu_render_pass_encoder_set_bind_group(render_pass, 0, bind_group, ptr::null(), 0, 0, 0);
    canvas_native_webgpu_render_pass_encoder_draw(render_pass, 36, 1, 0, 0);
    canvas_native_webgpu_render_pass_encoder_end(render_pass);

    let command_buffer = canvas_c::webgpu::gpu_command_encoder::canvas_native_webgpu_command_encoder_finish(command_encoder, ptr::null());
    canvas_native_webgpu_queue_submit(queue, &command_buffer as *const _ as *const *const _, 1);
    canvas_native_webgpu_context_present_surface(context, texture);
}
