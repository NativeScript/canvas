//! Linux: the canvas drawn into a Vulkan image that GL imports (`GL_EXT_memory_object_fd`),
//! ordered against the reader with the image's semaphores (`GL_EXT_semaphore_fd`).

use std::ffi::{c_void, CStr};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::OnceLock;

use crate::context::Context;

const GL_TEXTURE_TILING_EXT: u32 = 0x9580;
const GL_DEDICATED_MEMORY_OBJECT_EXT: u32 = 0x9581;
const GL_OPTIMAL_TILING_EXT: i32 = 0x9584;
const GL_HANDLE_TYPE_OPAQUE_FD_EXT: u32 = 0x9586;
const GL_LAYOUT_SHADER_READ_ONLY_EXT: u32 = 0x9591;
const GL_DEVICE_UUID_EXT: u32 = 0x9597;

/// A Vulkan image's memory and semaphores. The fds are only set the first time an image is
/// passed, and are owned by the callee from then on.
pub struct VulkanImageTarget {
    pub id: u64,
    pub memory_fd: i32,
    pub allocation_size: u64,
    pub width: i32,
    pub height: i32,
    pub ready_fd: i32,
    pub release_fd: i32,
    /// The reader's last use signalled `release`.
    pub wait_release: bool,
}

struct Fns {
    create_memory_objects: unsafe extern "C" fn(i32, *mut u32),
    delete_memory_objects: unsafe extern "C" fn(i32, *const u32),
    memory_object_parameteriv: unsafe extern "C" fn(u32, u32, *const i32),
    import_memory_fd: unsafe extern "C" fn(u32, u64, u32, i32),
    tex_storage_mem_2d: unsafe extern "C" fn(u32, i32, u32, i32, i32, u32, u64),
    get_unsigned_bytev: unsafe extern "C" fn(u32, *mut u8),
    gen_semaphores: unsafe extern "C" fn(i32, *mut u32),
    delete_semaphores: unsafe extern "C" fn(i32, *const u32),
    import_semaphore_fd: unsafe extern "C" fn(u32, u32, i32),
    wait_semaphore: unsafe extern "C" fn(u32, u32, *const u32, u32, *const u32, *const u32),
    signal_semaphore: unsafe extern "C" fn(u32, u32, *const u32, u32, *const u32, *const u32),
}

fn fns() -> Option<&'static Fns> {
    static FNS: OnceLock<Option<Fns>> = OnceLock::new();
    FNS.get_or_init(|| unsafe {
        fn load(name: &str) -> Option<*const c_void> {
            let f = canvas_core::gpu::gl::get_proc_address(name);
            (!f.is_null()).then_some(f)
        }
        Some(Fns {
            create_memory_objects: std::mem::transmute(load("glCreateMemoryObjectsEXT")?),
            delete_memory_objects: std::mem::transmute(load("glDeleteMemoryObjectsEXT")?),
            memory_object_parameteriv: std::mem::transmute(load("glMemoryObjectParameterivEXT")?),
            import_memory_fd: std::mem::transmute(load("glImportMemoryFdEXT")?),
            tex_storage_mem_2d: std::mem::transmute(load("glTexStorageMem2DEXT")?),
            get_unsigned_bytev: std::mem::transmute(load("glGetUnsignedBytevEXT")?),
            gen_semaphores: std::mem::transmute(load("glGenSemaphoresEXT")?),
            delete_semaphores: std::mem::transmute(load("glDeleteSemaphoresEXT")?),
            import_semaphore_fd: std::mem::transmute(load("glImportSemaphoreFdEXT")?),
            wait_semaphore: std::mem::transmute(load("glWaitSemaphoreEXT")?),
            signal_semaphore: std::mem::transmute(load("glSignalSemaphoreEXT")?),
        })
    })
    .as_ref()
}

fn close(fd: i32) {
    extern "C" {
        fn close(fd: i32) -> i32;
    }
    if fd >= 0 {
        unsafe { close(fd) };
    }
}

/// One image imported into the canvas's GL context.
struct Import {
    id: u64,
    memory: u32,
    texture: u32,
    ready: u32,
    release: u32,
}

impl Import {
    /// The context current; takes the target's fds.
    unsafe fn new(fns: &Fns, target: &VulkanImageTarget) -> Option<Self> {
        if target.memory_fd < 0 {
            for fd in [target.ready_fd, target.release_fd] {
                close(fd);
            }
            return None;
        }
        while gl_bindings::GetError() != gl_bindings::NO_ERROR {}
        let mut memory = 0;
        (fns.create_memory_objects)(1, &mut memory);
        (fns.memory_object_parameteriv)(memory, GL_DEDICATED_MEMORY_OBJECT_EXT, &1);
        (fns.import_memory_fd)(memory, target.allocation_size, GL_HANDLE_TYPE_OPAQUE_FD_EXT, target.memory_fd);
        let mut bound = 0;
        gl_bindings::GetIntegerv(gl_bindings::TEXTURE_BINDING_2D, &mut bound);
        let mut texture = 0;
        gl_bindings::GenTextures(1, &mut texture);
        gl_bindings::BindTexture(gl_bindings::TEXTURE_2D, texture);
        gl_bindings::TexParameteri(gl_bindings::TEXTURE_2D, GL_TEXTURE_TILING_EXT, GL_OPTIMAL_TILING_EXT);
        (fns.tex_storage_mem_2d)(
            gl_bindings::TEXTURE_2D,
            1,
            gl_bindings::RGBA8,
            target.width,
            target.height,
            memory,
            0,
        );
        gl_bindings::BindTexture(gl_bindings::TEXTURE_2D, bound as u32);
        let (mut ready, mut release) = (0, 0);
        if target.ready_fd >= 0 && target.release_fd >= 0 {
            (fns.gen_semaphores)(1, &mut ready);
            (fns.import_semaphore_fd)(ready, GL_HANDLE_TYPE_OPAQUE_FD_EXT, target.ready_fd);
            (fns.gen_semaphores)(1, &mut release);
            (fns.import_semaphore_fd)(release, GL_HANDLE_TYPE_OPAQUE_FD_EXT, target.release_fd);
        } else {
            close(target.ready_fd);
            close(target.release_fd);
        }
        let import = Import {
            id: target.id,
            memory,
            texture,
            ready,
            release,
        };
        if gl_bindings::GetError() != gl_bindings::NO_ERROR {
            import.delete(fns);
            return None;
        }
        Some(import)
    }

    /// The context current.
    unsafe fn delete(&self, fns: &Fns) {
        gl_bindings::DeleteTextures(1, &self.texture);
        (fns.delete_memory_objects)(1, &self.memory);
        for semaphore in [self.ready, self.release] {
            if semaphore != 0 {
                (fns.delete_semaphores)(1, &semaphore);
            }
        }
    }
}

/// Per canvas: the image it last drew into, and the id the reader caches images under.
pub(crate) struct VulkanInterop {
    producer: u64,
    import: Option<Import>,
}

impl Default for VulkanInterop {
    fn default() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        Self {
            producer: NEXT.fetch_add(1, Ordering::Relaxed),
            import: None,
        }
    }
}

impl Context {
    /// Identifies this canvas to a reader that caches the images it hands out.
    pub fn vulkan_producer(&self) -> u64 {
        self.vulkan_interop.producer
    }

    /// The UUID of the GL device, when GL can import Vulkan memory and semaphores.
    pub fn gl_device_uuid(&mut self) -> Option<[u8; 16]> {
        static UUID: OnceLock<Option<[u8; 16]>> = OnceLock::new();
        if let Some(uuid) = UUID.get() {
            return *uuid;
        }
        let fns = fns()?;
        let gl = self.gl_context.as_ref()?;
        if !gl.make_current() {
            return None;
        }
        let uuid = unsafe {
            let extensions = gl_bindings::GetString(gl_bindings::EXTENSIONS);
            let extensions = if extensions.is_null() {
                String::new()
            } else {
                CStr::from_ptr(extensions as *const _).to_string_lossy().into_owned()
            };
            let has = |name: &str| extensions.split(' ').any(|e| e == name);
            if has("GL_EXT_memory_object_fd") && has("GL_EXT_semaphore_fd") {
                while gl_bindings::GetError() != gl_bindings::NO_ERROR {}
                let mut uuid = [0u8; 16];
                (fns.get_unsigned_bytev)(GL_DEVICE_UUID_EXT, uuid.as_mut_ptr());
                (gl_bindings::GetError() == gl_bindings::NO_ERROR).then_some(uuid)
            } else {
                None
            }
        };
        *UUID.get_or_init(|| uuid)
    }

    /// Draws the canvas, premultiplied and top row first, into `target`: after the reader's
    /// `release`, then signals `ready`. Without semaphores it finishes before returning.
    pub fn draw_into_vulkan_image(&mut self, target: &VulkanImageTarget) -> bool {
        let Some(fns) = fns() else {
            return false;
        };
        let current = self.gl_context.as_ref().is_some_and(|gl| gl.make_current());
        if !current {
            return false;
        }
        if self.vulkan_interop.import.as_ref().map(|import| import.id) != Some(target.id) {
            if let Some(old) = self.vulkan_interop.import.take() {
                unsafe { old.delete(fns) };
            }
            self.vulkan_interop.import = unsafe { Import::new(fns, target) };
            if let Some(ctx) = self.direct_context.as_mut() {
                ctx.reset(None);
            }
        }
        let Some((texture, ready, release)) = self
            .vulkan_interop
            .import
            .as_ref()
            .map(|import| (import.texture, import.ready, import.release))
        else {
            return false;
        };
        self.flush();
        let layout = GL_LAYOUT_SHADER_READ_ONLY_EXT;
        if release != 0 && target.wait_release {
            unsafe { (fns.wait_semaphore)(release, 0, std::ptr::null(), 1, &texture, &layout) };
        }
        // The reader waits on `ready` from here on, so it is signalled even if drawing failed.
        if !self.draw_into_gl_texture(texture, target.width, target.height, false, None) {
            log::warn!("canvas: could not draw a 2D canvas into a WebGPU image");
        }
        unsafe {
            if ready != 0 {
                (fns.signal_semaphore)(ready, 0, std::ptr::null(), 1, &texture, &layout);
                gl_bindings::Flush();
            } else {
                gl_bindings::Finish();
            }
        }
        true
    }
}
