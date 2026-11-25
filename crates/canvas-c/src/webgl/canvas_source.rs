//! A 2D canvas as a WebGL texture source.

use crate::c2d::CanvasRenderingContext2D;
use crate::webgl::WebGLState;

/// For the readback, which is unpremultiplied, when `UNPACK_PREMULTIPLY_ALPHA_WEBGL` is set.
pub(crate) fn premultiply_in_place(pixels: &mut [u8]) {
    for pixel in pixels.chunks_exact_mut(4) {
        let alpha = pixel[3] as u32;
        if alpha != 255 {
            for channel in &mut pixel[..3] {
                *channel = ((*channel as u32 * alpha + 127) / 255) as u8;
            }
        }
    }
}

/// Where an upload lands: the texture bound to `target`, at (`x`, `y`) of `level`.
#[derive(Clone, Copy)]
#[allow(dead_code)]
struct Upload {
    target: u32,
    level: i32,
    /// texImage2D: allocate `level` at the canvas size with this internal format first.
    allocate: Option<i32>,
    x: i32,
    y: i32,
    format: u32,
    type_: u32,
    webgl2: bool,
}

/// Copies the canvas into the texture bound to `target` with no readback. `false`: use the
/// readback.
#[allow(clippy::too_many_arguments)]
pub(crate) fn copy_on_gpu(
    canvas: &mut CanvasRenderingContext2D,
    state: &WebGLState,
    target: u32,
    level: i32,
    allocate: Option<i32>,
    x: i32,
    y: i32,
    format: u32,
    type_: u32,
) -> bool {
    if !platform::SUPPORTED || format != gl_bindings::RGBA || type_ != gl_bindings::UNSIGNED_BYTE {
        return false;
    }
    let (flip_y, premultiply, webgl2) = state.unpack_flags();
    // Sized formats are WebGL 2 only; anything else is validated by the readback path.
    let allowed = |internal: i32| internal == gl_bindings::RGBA as i32 || (webgl2 && internal == gl_bindings::RGBA8 as i32);
    if allocate.is_some_and(|internal| !allowed(internal)) {
        return false;
    }
    let upload = Upload {
        target,
        level,
        allocate,
        x,
        y,
        format,
        type_,
        webgl2,
    };
    platform::copy(canvas, state, flip_y, premultiply, upload)
}

/// On the WebGL thread: `source`, a `width` x `height` texture of this context whose first row is
/// the canvas's top, copied into the upload's texture. Touches only the framebuffer binding it
/// restores (the read binding on WebGL 2) and, for WebGL 2, the unpack buffer while allocating.
#[allow(dead_code)]
unsafe fn copy_from(source: u32, width: i32, height: i32, upload: Upload) {
    let (bind, binding) = if upload.webgl2 {
        (gl_bindings::READ_FRAMEBUFFER, gl_bindings::READ_FRAMEBUFFER_BINDING)
    } else {
        (gl_bindings::FRAMEBUFFER, gl_bindings::FRAMEBUFFER_BINDING)
    };
    let mut framebuffer = 0;
    let mut previous = 0;
    gl_bindings::GetIntegerv(binding, &mut previous);
    gl_bindings::GenFramebuffers(1, &mut framebuffer);
    gl_bindings::BindFramebuffer(bind, framebuffer);
    gl_bindings::FramebufferTexture2D(bind, gl_bindings::COLOR_ATTACHMENT0, gl_bindings::TEXTURE_2D, source, 0);
    if let Some(internal) = upload.allocate {
        let mut unpack_buffer = 0;
        if upload.webgl2 {
            gl_bindings::GetIntegerv(gl_bindings::PIXEL_UNPACK_BUFFER_BINDING, &mut unpack_buffer);
            gl_bindings::BindBuffer(gl_bindings::PIXEL_UNPACK_BUFFER, 0);
        }
        gl_bindings::TexImage2D(
            upload.target,
            upload.level,
            internal,
            width,
            height,
            0,
            upload.format,
            upload.type_,
            std::ptr::null(),
        );
        if unpack_buffer != 0 {
            gl_bindings::BindBuffer(gl_bindings::PIXEL_UNPACK_BUFFER, unpack_buffer as u32);
        }
    }
    gl_bindings::CopyTexSubImage2D(upload.target, upload.level, upload.x, upload.y, 0, 0, width, height);
    gl_bindings::BindFramebuffer(bind, previous as u32);
    gl_bindings::DeleteFramebuffers(1, &framebuffer);
}

/// Linux/Android: the canvas draws itself into a texture of its GL context, which this context
/// reads through an EGLImage once an EGL fence says it is written.
#[cfg(any(target_os = "linux", target_os = "android"))]
mod platform {
    use super::*;

    pub(super) const SUPPORTED: bool = true;

    pub(super) fn copy(canvas: &mut CanvasRenderingContext2D, state: &WebGLState, flip_y: bool, premultiply: bool, upload: Upload) -> bool {
        // A threaded canvas's stand-in reports CPU; its real context answers below.
        if !canvas.is_threaded() && canvas.engine() != crate::c2d::Engine::GL {
            return false;
        }
        let Some(shared) = canvas
            .with_pixels_source(move |context| context.share_gl_texture(flip_y, premultiply))
            .flatten()
        else {
            return false;
        };
        let (width, height) = canvas.get_context().dimensions();
        let (width, height) = (width as i32, height as i32);

        state.post(move |state| unsafe {
            state.make_current();
            let mut source = 0;
            gl_bindings::GenTextures(1, &mut source);
            if shared.attach(source) {
                copy_from(source, width, height, upload);
            }
            gl_bindings::DeleteTextures(1, &source);
            drop(shared);
        });
        true
    }
}

/// iOS/tvOS: the canvas (Metal) is blitted into an IOSurface-backed pixel buffer on its own
/// queue, which this context (EAGL) reads as a texture once the blit has finished. Mac Catalyst
/// is `target_os = "ios"` but has no OpenGLES, so it takes the fallback.
#[cfg(all(feature = "gl", feature = "metal", any(target_os = "ios", target_os = "tvos"), not(target_abi = "macabi")))]
mod platform {
    use super::*;
    use objc2::rc::Retained;
    use objc2::runtime::AnyObject;
    use objc2_foundation::{NSDictionary, NSNumber, NSString};
    use std::ffi::c_void;
    use std::sync::{Arc, Condvar, Mutex};

    pub(super) const SUPPORTED: bool = true;

    type CFTypeRef = *const c_void;

    const PIXEL_FORMAT_32BGRA: u32 = u32::from_be_bytes(*b"BGRA");
    const MTL_PIXEL_FORMAT_BGRA8_UNORM: usize = 80;
    const GL_BGRA_EXT: u32 = 0x80E1;

    #[link(name = "CoreVideo", kind = "framework")]
    extern "C" {
        static kCVPixelBufferIOSurfacePropertiesKey: CFTypeRef;
        static kCVPixelBufferMetalCompatibilityKey: CFTypeRef;
        static kCVPixelBufferOpenGLESCompatibilityKey: CFTypeRef;
        static kCVPixelBufferOpenGLESTextureCacheCompatibilityKey: CFTypeRef;
        static kCVPixelBufferPixelFormatTypeKey: CFTypeRef;
        static kCVPixelBufferWidthKey: CFTypeRef;
        static kCVPixelBufferHeightKey: CFTypeRef;
        fn CVPixelBufferPoolCreate(
            allocator: CFTypeRef,
            pool_attributes: CFTypeRef,
            pixel_buffer_attributes: CFTypeRef,
            out: *mut *mut c_void,
        ) -> i32;
        fn CVPixelBufferPoolCreatePixelBuffer(allocator: CFTypeRef, pool: *mut c_void, out: *mut *mut c_void) -> i32;
        fn CVMetalTextureCacheCreate(
            allocator: CFTypeRef,
            cache_attributes: CFTypeRef,
            device: *const c_void,
            texture_attributes: CFTypeRef,
            out: *mut *mut c_void,
        ) -> i32;
        fn CVMetalTextureCacheCreateTextureFromImage(
            allocator: CFTypeRef,
            cache: *mut c_void,
            image: *mut c_void,
            attributes: CFTypeRef,
            format: usize,
            width: usize,
            height: usize,
            plane: usize,
            out: *mut *mut c_void,
        ) -> i32;
        fn CVMetalTextureGetTexture(texture: *mut c_void) -> *mut c_void;
        fn CVOpenGLESTextureCacheCreate(
            allocator: CFTypeRef,
            cache_attributes: CFTypeRef,
            context: *const c_void,
            texture_attributes: CFTypeRef,
            out: *mut *mut c_void,
        ) -> i32;
        fn CVOpenGLESTextureCacheCreateTextureFromImage(
            allocator: CFTypeRef,
            cache: *mut c_void,
            image: *mut c_void,
            attributes: CFTypeRef,
            target: u32,
            internal_format: i32,
            width: i32,
            height: i32,
            format: u32,
            type_: u32,
            plane: usize,
            out: *mut *mut c_void,
        ) -> i32;
        fn CVOpenGLESTextureGetName(texture: *mut c_void) -> u32;
        fn CVPixelBufferLockBaseAddress(buffer: *mut c_void, flags: u64) -> i32;
        fn CVPixelBufferUnlockBaseAddress(buffer: *mut c_void, flags: u64) -> i32;
        fn CVPixelBufferGetBaseAddress(buffer: *mut c_void) -> *mut c_void;
        fn CVPixelBufferGetBytesPerRow(buffer: *mut c_void) -> usize;
    }

    #[link(name = "CoreFoundation", kind = "framework")]
    extern "C" {
        fn CFRelease(value: CFTypeRef);
    }

    /// A +1 CoreFoundation object, released on drop; CF objects may cross threads.
    struct Owned(*mut c_void);
    unsafe impl Send for Owned {}
    impl Drop for Owned {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe { CFRelease(self.0) };
            }
        }
    }

    /// Set once the canvas's blit into the pixel buffer has finished.
    #[derive(Clone, Default)]
    struct Done(Arc<(Mutex<bool>, Condvar)>);
    impl Done {
        fn set(&self) {
            let (lock, signal) = &*self.0;
            if let Ok(mut done) = lock.lock() {
                *done = true;
            }
            signal.notify_all();
        }
        fn wait(&self) {
            let (lock, signal) = &*self.0;
            if let Ok(mut done) = lock.lock() {
                while !*done {
                    match signal.wait(done) {
                        Ok(next) => done = next,
                        Err(_) => return,
                    }
                }
            }
        }
    }

    /// One texture cache per device, for the canvases' blits.
    fn metal_cache(device: usize) -> Option<usize> {
        static CACHE: Mutex<Option<(usize, usize)>> = Mutex::new(None);
        let mut slot = CACHE.lock().ok()?;
        if let Some((cached_device, cache)) = *slot {
            if cached_device == device {
                return Some(cache);
            }
        }
        let mut cache = std::ptr::null_mut();
        let status = unsafe {
            CVMetalTextureCacheCreate(std::ptr::null(), std::ptr::null(), device as *const c_void, std::ptr::null(), &mut cache)
        };
        if status != 0 || cache.is_null() {
            return None;
        }
        // Kept for the life of the process, like the device.
        *slot = Some((device, cache as usize));
        Some(cache as usize)
    }

    /// From a pool per size: a buffer comes back once nothing (Metal or GL) still uses it.
    fn pixel_buffer(width: usize, height: usize) -> Option<Owned> {
        static POOLS: Mutex<Vec<((usize, usize), usize)>> = Mutex::new(Vec::new());
        const MAX_POOLS: usize = 16;
        let mut pools = POOLS.lock().ok()?;
        let pool = match pools.iter().position(|(size, _)| *size == (width, height)) {
            Some(index) => {
                let entry = pools.remove(index);
                pools.push(entry);
                entry.1
            }
            None => unsafe {
                let surface: Retained<NSDictionary<NSString, AnyObject>> = NSDictionary::new();
                let yes = NSNumber::new_bool(true);
                let format = NSNumber::new_u32(PIXEL_FORMAT_32BGRA);
                let w = NSNumber::new_usize(width);
                let h = NSNumber::new_usize(height);
                let keys: [&NSString; 7] = [
                    &*(kCVPixelBufferIOSurfacePropertiesKey as *const NSString),
                    &*(kCVPixelBufferMetalCompatibilityKey as *const NSString),
                    &*(kCVPixelBufferOpenGLESCompatibilityKey as *const NSString),
                    &*(kCVPixelBufferOpenGLESTextureCacheCompatibilityKey as *const NSString),
                    &*(kCVPixelBufferPixelFormatTypeKey as *const NSString),
                    &*(kCVPixelBufferWidthKey as *const NSString),
                    &*(kCVPixelBufferHeightKey as *const NSString),
                ];
                let values: [&AnyObject; 7] = [&surface, &yes, &yes, &yes, &format, &w, &h];
                let attributes = NSDictionary::from_slices(&keys, &values);
                let mut pool = std::ptr::null_mut();
                let status = CVPixelBufferPoolCreate(
                    std::ptr::null(),
                    std::ptr::null(),
                    Retained::as_ptr(&attributes) as CFTypeRef,
                    &mut pool,
                );
                if status != 0 || pool.is_null() {
                    return None;
                }
                if pools.len() == MAX_POOLS {
                    let (_, oldest) = pools.remove(0);
                    CFRelease(oldest as CFTypeRef);
                }
                pools.push(((width, height), pool as usize));
                pool as usize
            },
        };
        let mut buffer = std::ptr::null_mut();
        let status = unsafe { CVPixelBufferPoolCreatePixelBuffer(std::ptr::null(), pool as *mut c_void, &mut buffer) };
        (status == 0 && !buffer.is_null()).then(|| Owned(buffer))
    }

    /// When GL cannot use the pixel buffer: upload what the blit wrote there through memory.
    unsafe fn upload_from_memory(buffer: &Owned, width: usize, height: usize, upload: Upload) {
        const READ_ONLY: u64 = 1;
        if CVPixelBufferLockBaseAddress(buffer.0, READ_ONLY) != 0 {
            return;
        }
        let base = CVPixelBufferGetBaseAddress(buffer.0) as *const u8;
        let stride = CVPixelBufferGetBytesPerRow(buffer.0);
        let mut pixels = vec![0u8; width * height * 4];
        if !base.is_null() {
            for row in 0..height {
                let source = std::slice::from_raw_parts(base.add(row * stride), width * 4);
                let target = &mut pixels[row * width * 4..(row + 1) * width * 4];
                for (to, from) in target.chunks_exact_mut(4).zip(source.chunks_exact(4)) {
                    to.copy_from_slice(&[from[2], from[1], from[0], from[3]]);
                }
            }
        }
        CVPixelBufferUnlockBaseAddress(buffer.0, READ_ONLY);
        let mut unpack_buffer = 0;
        if upload.webgl2 {
            gl_bindings::GetIntegerv(gl_bindings::PIXEL_UNPACK_BUFFER_BINDING, &mut unpack_buffer);
            gl_bindings::BindBuffer(gl_bindings::PIXEL_UNPACK_BUFFER, 0);
        }
        let mut alignment = 4;
        gl_bindings::GetIntegerv(gl_bindings::UNPACK_ALIGNMENT, &mut alignment);
        gl_bindings::PixelStorei(gl_bindings::UNPACK_ALIGNMENT, 4);
        let data = pixels.as_ptr() as *const c_void;
        match upload.allocate {
            Some(internal) => gl_bindings::TexImage2D(
                upload.target,
                upload.level,
                internal,
                width as i32,
                height as i32,
                0,
                upload.format,
                upload.type_,
                data,
            ),
            None => gl_bindings::TexSubImage2D(
                upload.target,
                upload.level,
                upload.x,
                upload.y,
                width as i32,
                height as i32,
                upload.format,
                upload.type_,
                data,
            ),
        }
        gl_bindings::PixelStorei(gl_bindings::UNPACK_ALIGNMENT, alignment);
        if unpack_buffer != 0 {
            gl_bindings::BindBuffer(gl_bindings::PIXEL_UNPACK_BUFFER, unpack_buffer as u32);
        }
    }

    pub(super) fn copy(canvas: &mut CanvasRenderingContext2D, state: &WebGLState, flip_y: bool, premultiply: bool, upload: Upload) -> bool {
        // A blit cannot flip or unpremultiply.
        if flip_y || !premultiply {
            return false;
        }
        // Asks the real context: a threaded canvas's own engine is its recorder's.
        let device = canvas.metal_device();
        if device == 0 {
            return false;
        }
        let (width, height) = canvas.get_context().dimensions();
        let (width, height) = (width as usize, height as usize);
        if width == 0 || height == 0 {
            return false;
        }
        let Some(cache) = metal_cache(device) else {
            return false;
        };
        let Some(buffer) = pixel_buffer(width, height) else {
            return false;
        };
        let mut metal_texture = std::ptr::null_mut();
        let status = unsafe {
            CVMetalTextureCacheCreateTextureFromImage(
                std::ptr::null(),
                cache as *mut c_void,
                buffer.0,
                std::ptr::null(),
                MTL_PIXEL_FORMAT_BGRA8_UNORM,
                width,
                height,
                0,
                &mut metal_texture,
            )
        };
        if status != 0 || metal_texture.is_null() {
            return false;
        }
        let metal_texture = Owned(metal_texture);

        let done = Done::default();
        let written = done.clone();
        canvas.post_to_pixels_source(move |context| {
            let metal_texture = metal_texture;
            let target = unsafe { CVMetalTextureGetTexture(metal_texture.0) };
            if !target.is_null() {
                context.copy_to_metal_texture(target, std::ptr::null(), 0, true);
            }
            written.set();
        });

        state.post(move |state| unsafe {
            let buffer = buffer;
            done.wait();
            state.make_current();
            let context: *mut AnyObject = objc2::msg_send![objc2::class!(EAGLContext), currentContext];
            if context.is_null() {
                upload_from_memory(&buffer, width, height, upload);
                return;
            }
            let mut gl_cache = std::ptr::null_mut();
            if CVOpenGLESTextureCacheCreate(std::ptr::null(), std::ptr::null(), context as *const c_void, std::ptr::null(), &mut gl_cache) != 0 {
                upload_from_memory(&buffer, width, height, upload);
                return;
            }
            let gl_cache = Owned(gl_cache);
            let mut bound = 0;
            gl_bindings::GetIntegerv(gl_bindings::TEXTURE_BINDING_2D, &mut bound);
            let mut texture = std::ptr::null_mut();
            let status = CVOpenGLESTextureCacheCreateTextureFromImage(
                std::ptr::null(),
                gl_cache.0,
                buffer.0,
                std::ptr::null(),
                gl_bindings::TEXTURE_2D,
                gl_bindings::RGBA as i32,
                width as i32,
                height as i32,
                GL_BGRA_EXT,
                gl_bindings::UNSIGNED_BYTE,
                0,
                &mut texture,
            );
            gl_bindings::BindTexture(gl_bindings::TEXTURE_2D, bound as u32);
            if status != 0 || texture.is_null() {
                upload_from_memory(&buffer, width, height, upload);
                return;
            }
            let texture = Owned(texture);
            copy_from(CVOpenGLESTextureGetName(texture.0), width as i32, height as i32, upload);
            drop(texture);
            drop(gl_cache);
        });
        true
    }
}

#[cfg(not(any(
    target_os = "linux",
    target_os = "android",
    all(feature = "gl", feature = "metal", any(target_os = "ios", target_os = "tvos"), not(target_abi = "macabi"))
)))]
mod platform {
    use super::*;

    pub(super) const SUPPORTED: bool = false;

    pub(super) fn copy(_canvas: &mut CanvasRenderingContext2D, _state: &WebGLState, _flip_y: bool, _premultiply: bool, _upload: Upload) -> bool {
        false
    }
}
