//! Compiled pipelines kept on disk between launches, as browsers do, so a page
//! does not pay the driver's shader compiles again on every start. Vulkan only:
//! wgpu has no pipeline cache for the other backends.

use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};
use std::time::Duration;

use wgpu_core::device::Device;

static DIR: OnceLock<PathBuf> = OnceLock::new();

/// Where cache files go. Set once by the platform layer; later calls are ignored.
pub fn set_dir(dir: PathBuf) {
    let _ = DIR.set(dir);
}

fn dir() -> Option<&'static Path> {
    static DEFAULT: OnceLock<Option<PathBuf>> = OnceLock::new();
    DIR.get()
        .map(PathBuf::as_path)
        .or_else(|| DEFAULT.get_or_init(default_dir).as_deref())
}

#[cfg(target_os = "linux")]
fn default_dir() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_CACHE_HOME")
        .filter(|dir| !dir.is_empty())
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".cache")))?;
    let exe = std::env::current_exe().ok()?;
    Some(base.join(exe.file_stem()?).join("webgpu"))
}

#[cfg(not(target_os = "linux"))]
fn default_dir() -> Option<PathBuf> {
    None
}

/// Saves wait this long after a pipeline is created, so a burst of pipelines
/// is written once.
const SAVE_DELAY: Duration = Duration::from_secs(1);

/// The cache only grows: past this it starts over rather than keep every
/// pipeline the app ever made.
const MAX_BYTES: u64 = 32 * 1024 * 1024;

pub(crate) struct PipelineCache {
    cache: Arc<wgpu_core::pipeline::PipelineCache>,
    path: Option<PathBuf>,
    save_scheduled: Arc<AtomicBool>,
}

impl std::fmt::Debug for PipelineCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PipelineCache")
            .field("path", &self.path)
            .finish()
    }
}

impl PipelineCache {
    /// Loads the cache for `adapter`, or `None` if the device cannot cache
    /// pipelines. Without a directory the cache only lives as long as the device.
    pub(crate) fn create(device: &Arc<Device>, adapter: &wgt::AdapterInfo) -> Option<Self> {
        if !device.features().contains(wgt::Features::PIPELINE_CACHE) {
            return None;
        }
        let path = dir().map(|dir| {
            dir.join(format!(
                "pipelines-{}-{:x}-{:x}.bin",
                adapter.backend.to_str(),
                adapter.vendor,
                adapter.device
            ))
        });
        let data = path
            .as_deref()
            .filter(|path| std::fs::metadata(path).is_ok_and(|meta| meta.len() <= MAX_BYTES))
            .and_then(|path| std::fs::read(path).ok());
        let create = |data: Option<Vec<u8>>| {
            // SAFETY: the data was written by `get_data`. wgpu checks its header
            // against this adapter and driver.
            unsafe {
                device.create_pipeline_cache(&wgpu_core::pipeline::PipelineCacheDescriptor {
                    label: None,
                    data: data.map(Cow::Owned),
                    fallback: true,
                })
            }
        };
        let had_data = data.is_some();
        let (mut cache, mut error) = create(data);
        // wgpu rejects data from another device even with `fallback`: start
        // empty, and the next save replaces the file.
        if error.is_some() && had_data {
            (cache, error) = create(None);
        }
        if let Some(error) = error {
            log::warn!("webgpu pipeline cache unavailable: {error}");
            return None;
        }
        Some(PipelineCache {
            cache,
            path,
            save_scheduled: Arc::default(),
        })
    }

    pub(crate) fn get(&self) -> Arc<wgpu_core::pipeline::PipelineCache> {
        Arc::clone(&self.cache)
    }

    /// Writes the cache out, off the calling thread, shortly after the last
    /// pipeline was added to it.
    pub(crate) fn pipeline_created(&self) {
        let Some(path) = self.path.clone() else {
            return;
        };
        if self.save_scheduled.swap(true, Ordering::AcqRel) {
            return;
        }
        let cache = Arc::clone(&self.cache);
        let scheduled = Arc::clone(&self.save_scheduled);
        let spawned = std::thread::Builder::new()
            .name("webgpu-pipeline-cache".into())
            .spawn(move || {
                std::thread::sleep(SAVE_DELAY);
                scheduled.store(false, Ordering::Release);
                if let Some(data) = cache.get_data() {
                    if let Err(error) = write_atomic(&path, &data) {
                        log::warn!("webgpu pipeline cache not saved: {error}");
                    }
                }
            });
        if spawned.is_err() {
            self.save_scheduled.store(false, Ordering::Release);
        }
    }
}

/// A reader never sees a partly written file: wgpu checks the header and size
/// but not the contents.
fn write_atomic(path: &Path, data: &[u8]) -> std::io::Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let temp = path.with_extension(format!("tmp{}", std::process::id()));
    std::fs::write(&temp, data)?;
    std::fs::rename(&temp, path).inspect_err(|_| {
        let _ = std::fs::remove_file(&temp);
    })
}
