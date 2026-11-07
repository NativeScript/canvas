//! ANGLE's program cache (`EGL_ANDROID_blob_cache`), in memory and on disk: without it every
//! context recompiles every shader.

use std::collections::HashMap;
use std::ffi::c_void;
use std::path::PathBuf;
use std::sync::OnceLock;

use parking_lot::Mutex;

type SetBlob = unsafe extern "C" fn(*const c_void, isize, *const c_void, isize);
type GetBlob = unsafe extern "C" fn(*const c_void, isize, *mut c_void, isize) -> isize;
type SetBlobCacheFuncs = unsafe extern "system" fn(*mut c_void, SetBlob, GetBlob);

const MAX_MEMORY_BYTES: usize = 32 * 1024 * 1024;
const MAX_DISK_BYTES: u64 = 64 * 1024 * 1024;

#[derive(Default)]
struct Memory {
    entries: HashMap<Vec<u8>, Vec<u8>>,
    bytes: usize,
}

impl Memory {
    fn insert(&mut self, key: Vec<u8>, value: Vec<u8>) {
        if self.bytes + value.len() > MAX_MEMORY_BYTES {
            self.entries.clear();
            self.bytes = 0;
        }
        self.bytes += value.len();
        if let Some(old) = self.entries.insert(key, value) {
            self.bytes -= old.len();
        }
    }
}

struct Cache {
    memory: Mutex<Memory>,
    dir: Option<PathBuf>,
}

static CACHE: OnceLock<Cache> = OnceLock::new();

/// `CANVAS_PROGRAM_CACHE_DIR` (empty: memory only), else `%LOCALAPPDATA%\<exe>\canvas\program-cache`.
fn cache_dir() -> Option<PathBuf> {
    if let Some(dir) = std::env::var_os("CANVAS_PROGRAM_CACHE_DIR") {
        return (!dir.is_empty()).then(|| PathBuf::from(dir));
    }
    let base = std::env::var_os("LOCALAPPDATA")?;
    let exe = std::env::current_exe().ok()?;
    Some(PathBuf::from(base).join(exe.file_stem()?).join("canvas").join("program-cache"))
}

fn prepare(dir: PathBuf) -> Option<PathBuf> {
    let size: u64 = std::fs::read_dir(&dir)
        .map(|entries| entries.flatten().filter_map(|e| e.metadata().ok()).map(|m| m.len()).sum())
        .unwrap_or(0);
    if size > MAX_DISK_BYTES {
        let _ = std::fs::remove_dir_all(&dir);
    }
    std::fs::create_dir_all(&dir).ok()?;
    Some(dir)
}

impl Cache {
    fn path(&self, key: &[u8]) -> Option<PathBuf> {
        if key.len() > 64 {
            return None;
        }
        let name: String = key.iter().map(|b| format!("{b:02x}")).collect();
        Some(self.dir.as_ref()?.join(name))
    }

    fn read(&self, key: &[u8]) -> Option<Vec<u8>> {
        std::fs::read(self.path(key)?).ok()
    }

    fn write(&self, key: &[u8], value: &[u8]) {
        let Some(path) = self.path(key) else { return };
        let temp = path.with_extension("tmp");
        if std::fs::write(&temp, value).is_err() || std::fs::rename(&temp, &path).is_err() {
            let _ = std::fs::remove_file(&temp);
        }
    }
}

unsafe fn bytes<'a>(data: *const c_void, size: isize) -> Option<&'a [u8]> {
    (!data.is_null() && size > 0).then(|| unsafe { std::slice::from_raw_parts(data as *const u8, size as usize) })
}

unsafe extern "C" fn set_blob(key: *const c_void, key_size: isize, value: *const c_void, value_size: isize) {
    let (Some(cache), Some(key), Some(value)) = (CACHE.get(), unsafe { bytes(key, key_size) }, unsafe { bytes(value, value_size) })
    else {
        return;
    };
    cache.memory.lock().insert(key.to_vec(), value.to_vec());
    cache.write(key, value);
}

/// Returns the size; copies only when `value_size` fits.
unsafe extern "C" fn get_blob(key: *const c_void, key_size: isize, value: *mut c_void, value_size: isize) -> isize {
    let (Some(cache), Some(key)) = (CACHE.get(), unsafe { bytes(key, key_size) }) else {
        return 0;
    };
    let mut memory = cache.memory.lock();
    if !memory.entries.contains_key(key) {
        let Some(blob) = cache.read(key) else { return 0 };
        memory.insert(key.to_vec(), blob);
    }
    let Some(blob) = memory.entries.get(key) else { return 0 };
    if !value.is_null() && value_size >= blob.len() as isize {
        unsafe { std::ptr::copy_nonoverlapping(blob.as_ptr(), value as *mut u8, blob.len()) };
    }
    blob.len() as isize
}

/// Before the display's first context.
pub(super) fn install(set_blob_cache_funcs: *const c_void, display: *mut c_void) {
    if set_blob_cache_funcs.is_null() {
        return;
    }
    CACHE.get_or_init(|| Cache {
        memory: Mutex::new(Memory::default()),
        dir: cache_dir().and_then(prepare),
    });
    let set_funcs: SetBlobCacheFuncs = unsafe { std::mem::transmute(set_blob_cache_funcs) };
    unsafe { set_funcs(display, set_blob, get_blob) };
}
