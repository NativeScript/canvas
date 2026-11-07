//! Linux: decoded video frames from @nativescript/canvas-media (`NSCVideoFrame.address`).
//!
//! A GL frame is a texture on GDK's EGL display, which canvases share: WebGL copies it on the
//! GPU through its EGLImage; WebGPU has the producer blit it into an exportable Vulkan image
//! (`webgpu::linux_video_frame`). Other frames are RGBA rows in memory, uploaded once.

use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::c_void;

use crate::webgl::WebGLState;

/// Same layout as `LinuxVideoFrame` in canvas-media's module.
#[repr(C)]
pub struct CanvasLinuxVideoFrame {
    pub size: u32,
    pub kind: u32,
    pub width: u32,
    pub height: u32,
    pub producer: u64,
    pub egl_display: *mut c_void,
    pub egl_image: *mut c_void,
    pub image_id: u64,
    pub data: *const u8,
    pub stride: u32,
    pub reserved: u32,
    pub device_uuid: [u8; 16],
    pub copy_to: Option<
        unsafe extern "C" fn(*mut CanvasLinuxVideoFrame, *const CanvasLinuxFrameTarget) -> bool,
    >,
    /// Set by a GL reader: an EGLSync the producer waits for before reusing the texture.
    pub release_sync: *mut c_void,
    pub user: *mut c_void,
}

pub const KIND_GL: u32 = 1;
pub const KIND_MEMORY: u32 = 2;

/// Same layout as `LinuxFrameTarget` in canvas-media's module: an exportable RGBA8 Vulkan image.
#[repr(C)]
pub struct CanvasLinuxFrameTarget {
    pub size: u32,
    pub memory_fd: i32,
    pub id: u64,
    pub allocation_size: u64,
    pub width: u32,
    pub height: u32,
    pub ready_fd: i32,
    pub release_fd: i32,
    pub wait_release: u32,
    pub reserved: u32,
}

pub(crate) unsafe fn frame<'a>(handle: *mut c_void) -> Option<&'a mut CanvasLinuxVideoFrame> {
    let frame = unsafe { (handle as *mut CanvasLinuxVideoFrame).as_mut() }?;
    (frame.size as usize == std::mem::size_of::<CanvasLinuxVideoFrame>()
        && frame.width > 0
        && frame.height > 0)
        .then_some(frame)
}

const GL_RGBA8: i32 = 0x8058;
const GL_SRGB8_ALPHA8: i32 = 0x8C43;
const EGL_SYNC_FENCE_KHR: u32 = 0x30F9;

type ImageTargetTexture = unsafe extern "C" fn(u32, *mut c_void);
type GetCurrentContext = unsafe extern "C" fn() -> *mut c_void;
type CreateSync = unsafe extern "C" fn(*mut c_void, u32, *const i32) -> *mut c_void;
type DestroySync = unsafe extern "C" fn(*mut c_void, *mut c_void) -> u32;

struct Egl {
    target_texture: ImageTargetTexture,
    current_context: GetCurrentContext,
    create_sync: CreateSync,
    destroy_sync: DestroySync,
}

fn egl() -> Option<&'static Egl> {
    static EGL: std::sync::OnceLock<Option<Egl>> = std::sync::OnceLock::new();
    EGL.get_or_init(|| {
        fn load<T>(name: &str) -> Option<T> {
            let address = canvas_core::gpu::gl::get_proc_address(name);
            (!address.is_null()).then(|| unsafe { std::mem::transmute_copy(&address) })
        }
        Some(Egl {
            target_texture: load("glEGLImageTargetTexture2DOES")?,
            current_context: load("eglGetCurrentContext")?,
            create_sync: load("eglCreateSyncKHR")?,
            destroy_sync: load("eglDestroySyncKHR")?,
        })
    })
    .as_ref()
}

/// A frame image as a texture of one context, and a framebuffer reading it.
struct Imported {
    texture: u32,
    framebuffer: u32,
    used: u64,
}

const MAX_IMPORTED: usize = 8;

thread_local! {
    /// By (EGL context, image id).
    static IMPORTED: RefCell<HashMap<(usize, u64), Imported>> = RefCell::new(HashMap::new());
    /// A draw framebuffer per EGL context.
    static DRAW: RefCell<HashMap<usize, u32>> = RefCell::new(HashMap::new());
    static USES: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// The frame image in the current context, imported once.
unsafe fn imported(egl: &Egl, context: usize, frame: &CanvasLinuxVideoFrame) -> Option<u32> {
    let uses = USES.with(|uses| {
        uses.set(uses.get() + 1);
        uses.get()
    });
    IMPORTED.with(|imported| {
        let mut imported = imported.borrow_mut();
        if let Some(entry) = imported.get_mut(&(context, frame.image_id)) {
            entry.used = uses;
            return Some(entry.framebuffer);
        }
        if imported.keys().filter(|(c, _)| *c == context).count() >= MAX_IMPORTED {
            let oldest = imported
                .iter()
                .filter(|((c, _), _)| *c == context)
                .min_by_key(|(_, entry)| entry.used)
                .map(|(key, _)| *key)?;
            if let Some(entry) = imported.remove(&oldest) {
                unsafe {
                    gl_bindings::DeleteFramebuffers(1, &entry.framebuffer);
                    gl_bindings::DeleteTextures(1, &entry.texture);
                }
            }
        }
        unsafe {
            while gl_bindings::GetError() != gl_bindings::NO_ERROR {}
            let mut bound = 0;
            gl_bindings::GetIntegerv(gl_bindings::TEXTURE_BINDING_2D, &mut bound);
            let mut texture = 0;
            gl_bindings::GenTextures(1, &mut texture);
            gl_bindings::BindTexture(gl_bindings::TEXTURE_2D, texture);
            (egl.target_texture)(gl_bindings::TEXTURE_2D, frame.egl_image);
            gl_bindings::TexParameteri(
                gl_bindings::TEXTURE_2D,
                gl_bindings::TEXTURE_MIN_FILTER,
                gl_bindings::NEAREST as i32,
            );
            gl_bindings::TexParameteri(
                gl_bindings::TEXTURE_2D,
                gl_bindings::TEXTURE_MAG_FILTER,
                gl_bindings::NEAREST as i32,
            );
            gl_bindings::BindTexture(gl_bindings::TEXTURE_2D, bound as u32);
            let mut framebuffer = 0;
            gl_bindings::GenFramebuffers(1, &mut framebuffer);
            if gl_bindings::GetError() != gl_bindings::NO_ERROR {
                gl_bindings::DeleteFramebuffers(1, &framebuffer);
                gl_bindings::DeleteTextures(1, &texture);
                return None;
            }
            imported.insert(
                (context, frame.image_id),
                Imported {
                    texture,
                    framebuffer,
                    used: uses,
                },
            );
            // Attached on first use, with the read binding saved by the caller.
            gl_bindings::BindFramebuffer(gl_bindings::READ_FRAMEBUFFER, framebuffer);
            gl_bindings::FramebufferTexture2D(
                gl_bindings::READ_FRAMEBUFFER,
                gl_bindings::COLOR_ATTACHMENT0,
                gl_bindings::TEXTURE_2D,
                texture,
                0,
            );
            Some(framebuffer)
        }
    })
}

fn draw_framebuffer(context: usize) -> u32 {
    DRAW.with(|draw| {
        *draw.borrow_mut().entry(context).or_insert_with(|| {
            let mut framebuffer = 0;
            unsafe { gl_bindings::GenFramebuffers(1, &mut framebuffer) };
            framebuffer
        })
    })
}

/// The texture binding a `texImage` target's texture is found at, and its parameter target.
fn binding(target: u32) -> Option<(u32, u32)> {
    Some(match target {
        gl_bindings::TEXTURE_2D => (gl_bindings::TEXTURE_BINDING_2D, gl_bindings::TEXTURE_2D),
        gl_bindings::TEXTURE_CUBE_MAP_POSITIVE_X..=gl_bindings::TEXTURE_CUBE_MAP_NEGATIVE_Z => (
            gl_bindings::TEXTURE_BINDING_CUBE_MAP,
            gl_bindings::TEXTURE_CUBE_MAP,
        ),
        gl_bindings::TEXTURE_3D => (gl_bindings::TEXTURE_BINDING_3D, gl_bindings::TEXTURE_3D),
        gl_bindings::TEXTURE_2D_ARRAY => (
            gl_bindings::TEXTURE_BINDING_2D_ARRAY,
            gl_bindings::TEXTURE_2D_ARRAY,
        ),
        _ => return None,
    })
}

/// Unpack state a frame upload resets, put back after.
struct Unpack([i32; 7]);

const UNPACK: [u32; 6] = [
    gl_bindings::UNPACK_ROW_LENGTH,
    gl_bindings::UNPACK_ALIGNMENT,
    gl_bindings::UNPACK_SKIP_ROWS,
    gl_bindings::UNPACK_SKIP_PIXELS,
    gl_bindings::UNPACK_IMAGE_HEIGHT,
    gl_bindings::UNPACK_SKIP_IMAGES,
];

impl Unpack {
    unsafe fn reset(row_length: i32) -> Self {
        let mut saved = [0; 7];
        unsafe {
            for (i, name) in UNPACK.iter().enumerate() {
                gl_bindings::GetIntegerv(*name, &mut saved[i]);
            }
            gl_bindings::GetIntegerv(gl_bindings::PIXEL_UNPACK_BUFFER_BINDING, &mut saved[6]);
            for name in UNPACK {
                gl_bindings::PixelStorei(
                    name,
                    if name == gl_bindings::UNPACK_ALIGNMENT {
                        4
                    } else {
                        0
                    },
                );
            }
            gl_bindings::PixelStorei(gl_bindings::UNPACK_ROW_LENGTH, row_length);
            gl_bindings::BindBuffer(gl_bindings::PIXEL_UNPACK_BUFFER, 0);
        }
        Unpack(saved)
    }

    unsafe fn restore(&self) {
        unsafe {
            for (i, name) in UNPACK.iter().enumerate() {
                gl_bindings::PixelStorei(*name, self.0[i]);
            }
            gl_bindings::BindBuffer(gl_bindings::PIXEL_UNPACK_BUFFER, self.0[6] as u32);
        }
    }
}

/// Where a frame goes: `texImage2D/3D` (a 1-deep volume) defines the level unless the texture is
/// immutable, `texSubImage` writes at the offset.
#[derive(Clone, Copy)]
struct Destination {
    target: u32,
    level: i32,
    internalformat: i32,
    three_d: bool,
    define: bool,
    x: i32,
    y: i32,
    z: i32,
}

unsafe fn define(dst: &Destination, width: i32, height: i32, pixels: *const c_void) {
    unsafe {
        let (format, type_) = (gl_bindings::RGBA, gl_bindings::UNSIGNED_BYTE);
        match (dst.define, dst.three_d) {
            (true, false) => gl_bindings::TexImage2D(
                dst.target,
                dst.level,
                dst.internalformat,
                width,
                height,
                0,
                format,
                type_,
                pixels,
            ),
            (true, true) => gl_bindings::TexImage3D(
                dst.target,
                dst.level,
                dst.internalformat,
                width,
                height,
                1,
                0,
                format,
                type_,
                pixels,
            ),
            (false, false) => gl_bindings::TexSubImage2D(
                dst.target, dst.level, dst.x, dst.y, width, height, format, type_, pixels,
            ),
            (false, true) => gl_bindings::TexSubImage3D(
                dst.target, dst.level, dst.x, dst.y, dst.z, width, height, 1, format, type_, pixels,
            ),
        }
    }
}

/// `texImage` on an immutable texture (texStorage) only writes it.
unsafe fn immutable(target: u32) -> bool {
    let Some((_, parameter_target)) = binding(target) else {
        return false;
    };
    let mut immutable = 0;
    unsafe {
        gl_bindings::GetTexParameteriv(
            parameter_target,
            gl_bindings::TEXTURE_IMMUTABLE_FORMAT,
            &mut immutable,
        )
    };
    immutable != 0
}

/// One upload from memory; flipped rows need a copy.
unsafe fn upload(frame: &CanvasLinuxVideoFrame, dst: &Destination, flip_y: bool) -> bool {
    if frame.data.is_null() || frame.stride < frame.width * 4 || frame.stride % 4 != 0 {
        return false;
    }
    let (width, height) = (frame.width as usize, frame.height as usize);
    let stride = frame.stride as usize;
    let rows = unsafe { std::slice::from_raw_parts(frame.data, stride * (height - 1) + width * 4) };
    let flipped;
    let (pixels, row_length) = if flip_y {
        let row = width * 4;
        let mut out = vec![0u8; row * height];
        for (y, chunk) in out.chunks_exact_mut(row).enumerate() {
            let source = (height - 1 - y) * stride;
            chunk.copy_from_slice(&rows[source..source + row]);
        }
        flipped = out;
        (flipped.as_ptr(), 0)
    } else {
        (rows.as_ptr(), (stride / 4) as i32)
    };
    unsafe {
        let unpack = Unpack::reset(row_length);
        define(dst, width as i32, height as i32, pixels as *const c_void);
        unpack.restore();
        gl_bindings::GetError() == gl_bindings::NO_ERROR
    }
}

/// A GPU copy from the frame's EGLImage; the producer's texture is released once it ran.
unsafe fn copy(frame: &mut CanvasLinuxVideoFrame, dst: &Destination, flip_y: bool) -> bool {
    let Some(egl) = egl() else {
        return false;
    };
    if frame.egl_image.is_null() || frame.egl_display != canvas_core::gpu::gl::egl_display() {
        return false;
    }
    let context = unsafe { (egl.current_context)() } as usize;
    let (Some((binding, _)), true) = (binding(dst.target), context != 0) else {
        return false;
    };
    unsafe {
        while gl_bindings::GetError() != gl_bindings::NO_ERROR {}
        let (mut read, mut draw, mut texture) = (0, 0, 0);
        gl_bindings::GetIntegerv(gl_bindings::READ_FRAMEBUFFER_BINDING, &mut read);
        gl_bindings::GetIntegerv(gl_bindings::DRAW_FRAMEBUFFER_BINDING, &mut draw);
        gl_bindings::GetIntegerv(binding, &mut texture);
        if texture == 0 {
            return false;
        }
        let scissor = gl_bindings::IsEnabled(gl_bindings::SCISSOR_TEST) != 0;
        let restore = || {
            gl_bindings::BindFramebuffer(gl_bindings::READ_FRAMEBUFFER, read as u32);
            gl_bindings::BindFramebuffer(gl_bindings::DRAW_FRAMEBUFFER, draw as u32);
            if scissor {
                gl_bindings::Enable(gl_bindings::SCISSOR_TEST);
            }
        };
        let Some(source) = imported(egl, context, frame) else {
            restore();
            return false;
        };
        let (width, height) = (frame.width as i32, frame.height as i32);
        if dst.define {
            let unpack = Unpack::reset(0);
            define(dst, width, height, std::ptr::null());
            unpack.restore();
        }
        let target = draw_framebuffer(context);
        gl_bindings::BindFramebuffer(gl_bindings::READ_FRAMEBUFFER, source);
        gl_bindings::BindFramebuffer(gl_bindings::DRAW_FRAMEBUFFER, target);
        if dst.three_d {
            gl_bindings::FramebufferTextureLayer(
                gl_bindings::DRAW_FRAMEBUFFER,
                gl_bindings::COLOR_ATTACHMENT0,
                texture as u32,
                dst.level,
                dst.z,
            );
        } else {
            gl_bindings::FramebufferTexture2D(
                gl_bindings::DRAW_FRAMEBUFFER,
                gl_bindings::COLOR_ATTACHMENT0,
                dst.target,
                texture as u32,
                dst.level,
            );
        }
        let complete = gl_bindings::CheckFramebufferStatus(gl_bindings::DRAW_FRAMEBUFFER)
            == gl_bindings::FRAMEBUFFER_COMPLETE;
        if complete {
            gl_bindings::Disable(gl_bindings::SCISSOR_TEST);
            // Row 0 is the frame's top, as texImage wants it unless UNPACK_FLIP_Y_WEBGL.
            let (y0, y1) = if flip_y {
                (dst.y + height, dst.y)
            } else {
                (dst.y, dst.y + height)
            };
            gl_bindings::BlitFramebuffer(
                0,
                0,
                width,
                height,
                dst.x,
                y0,
                dst.x + width,
                y1,
                gl_bindings::COLOR_BUFFER_BIT,
                gl_bindings::NEAREST,
            );
        }
        gl_bindings::FramebufferTexture2D(
            gl_bindings::DRAW_FRAMEBUFFER,
            gl_bindings::COLOR_ATTACHMENT0,
            gl_bindings::TEXTURE_2D,
            0,
            0,
        );
        restore();
        if !complete || gl_bindings::GetError() != gl_bindings::NO_ERROR {
            return false;
        }
        let sync = (egl.create_sync)(frame.egl_display, EGL_SYNC_FENCE_KHR, std::ptr::null());
        gl_bindings::Flush();
        if !frame.release_sync.is_null() {
            (egl.destroy_sync)(frame.egl_display, frame.release_sync);
        }
        frame.release_sync = sync;
        true
    }
}

/// WebGL `texImage2D/3D(video)` from a canvas-media frame: `three_d` writes one slice (`zoffset`),
/// `sub` the offsets. The frame is RGBA8: sized RGBA formats are kept, others become RGBA.
/// `false` leaves the caller on its pixel path.
#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgl_tex_image_video_frame(
    frame: *mut c_void,
    target: u32,
    level: i32,
    internalformat: i32,
    three_d: bool,
    sub: bool,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    state: *mut WebGLState,
) -> bool {
    let (Some(frame), Some(state)) = (unsafe { self::frame(frame) }, unsafe { state.as_ref() })
    else {
        return false;
    };
    let internalformat = match internalformat {
        GL_RGBA8 | GL_SRGB8_ALPHA8 => internalformat,
        _ => gl_bindings::RGBA as i32,
    };
    let mut dst = Destination {
        target,
        level,
        internalformat,
        three_d,
        define: !sub,
        x: if sub { xoffset } else { 0 },
        y: if sub { yoffset } else { 0 },
        z: if sub || three_d { zoffset } else { 0 },
    };
    // The caller keeps the frame until this returns: run now even on a threaded context.
    let frame = frame as *mut CanvasLinuxVideoFrame as usize;
    state.sync(move |state| {
        state.make_current();
        let flip_y = state.get_flip_y();
        dst.define = dst.define && !unsafe { immutable(dst.target) };
        let frame = unsafe { &mut *(frame as *mut CanvasLinuxVideoFrame) };
        let done = match frame.kind {
            KIND_GL => unsafe { copy(frame, &dst, flip_y) },
            KIND_MEMORY => unsafe { upload(frame, &dst, flip_y) },
            _ => false,
        };
        log_path("WebGL", frame.kind, done);
        done
    })
}

/// `CANVAS_LOG=info`: which path video frames take, once per change.
pub(crate) fn log_path(api: &'static str, kind: u32, done: bool) {
    static LAST: std::sync::Mutex<Vec<(&'static str, u32, bool)>> =
        std::sync::Mutex::new(Vec::new());
    let mut last = LAST.lock().unwrap_or_else(|e| e.into_inner());
    if last.contains(&(api, kind, done)) {
        return;
    }
    last.retain(|entry| entry.0 != api);
    last.push((api, kind, done));
    let path = match (kind, done) {
        (KIND_GL, true) => "GPU (shared GL texture)",
        (KIND_MEMORY, true) => "one upload from mapped memory",
        _ => "not native: the caller's fallback",
    };
    log::info!("canvas: {api} video frames: {path}");
}
