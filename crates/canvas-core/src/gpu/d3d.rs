//! Direct3D 12 device for Skia's Ganesh D3D backend on Windows.
//!
//! Every canvas created on a thread shares one device and direct queue, the way the Metal backend
//! shares one `MTLDevice`: GPU resources (images, patterns, the glyph cache) can then move between
//! canvases without a readback, and a page with many canvases does not create many devices.

use std::cell::RefCell;
use std::rc::Rc;

use windows::core::{Interface, Result};
use windows::Win32::Graphics::Direct3D::D3D_FEATURE_LEVEL_11_0;
use windows::Win32::Graphics::Direct3D12::{
    D3D12CreateDevice, D3D12GetDebugInterface, ID3D12CommandQueue, ID3D12Debug, ID3D12Device,
    ID3D12Device5, D3D12_COMMAND_LIST_TYPE_DIRECT, D3D12_COMMAND_QUEUE_DESC,
};
use windows::Win32::Graphics::Dxgi::{
    CreateDXGIFactory2, IDXGIAdapter1, IDXGIFactory4, IDXGIFactory6, DXGI_ADAPTER_FLAG,
    DXGI_ADAPTER_FLAG_SOFTWARE, DXGI_CREATE_FACTORY_DEBUG, DXGI_CREATE_FACTORY_FLAGS,
    DXGI_GPU_PREFERENCE, DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE, DXGI_GPU_PREFERENCE_MINIMUM_POWER,
    DXGI_GPU_PREFERENCE_UNSPECIFIED,
};

/// Mirrors the `powerPreference` context attribute.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PowerPreference {
    #[default]
    Default,
    LowPower,
    HighPerformance,
}

impl PowerPreference {
    fn dxgi(self) -> DXGI_GPU_PREFERENCE {
        match self {
            PowerPreference::Default => DXGI_GPU_PREFERENCE_UNSPECIFIED,
            PowerPreference::LowPower => DXGI_GPU_PREFERENCE_MINIMUM_POWER,
            PowerPreference::HighPerformance => DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE,
        }
    }
}

pub struct D3D12Context {
    factory: IDXGIFactory4,
    adapter: IDXGIAdapter1,
    device: ID3D12Device,
    queue: ID3D12CommandQueue,
    is_warp: bool,
    on12: std::cell::OnceCell<Option<D3D11On12>>,
}

/// A D3D11 device layered on this D3D12 device and queue (D3D11On12): what XAML surfaces take.
/// Its work goes to the same queue, after the D3D12 work submitted before it.
pub struct D3D11On12 {
    pub device: windows::Win32::Graphics::Direct3D11::ID3D11Device,
    pub context: windows::Win32::Graphics::Direct3D11::ID3D11DeviceContext,
    pub on12: windows::Win32::Graphics::Direct3D11on12::ID3D11On12Device,
}

thread_local! {
    static SHARED: RefCell<Option<Rc<D3D12Context>>> = const { RefCell::new(None) };
}

fn env_flag(name: &str) -> bool {
    std::env::var_os(name).is_some_and(|v| !v.is_empty() && v != "0")
}

impl D3D12Context {
    /// The device shared by every canvas on this thread, created on first use (and again after
    /// the device is removed). `preference` only applies to a creation -- it is a hint, like the
    /// web attribute it mirrors.
    pub fn shared(preference: PowerPreference) -> Option<Rc<D3D12Context>> {
        SHARED.with(|shared| {
            let mut shared = shared.borrow_mut();
            if let Some(context) = shared.as_ref() {
                if !context.is_removed() {
                    return Some(context.clone());
                }
            }
            match D3D12Context::new(preference) {
                Ok(context) => {
                    let context = Rc::new(context);
                    *shared = Some(context.clone());
                    Some(context)
                }
                Err(error) => {
                    log::error!("canvas: failed to create a Direct3D 12 device: {error}");
                    None
                }
            }
        })
    }

    /// Creates a standalone device. `CANVAS_FORCE_WARP=1` selects the WARP software rasterizer
    /// (CI machines without a GPU) and `CANVAS_D3D_DEBUG=1` enables the D3D12 debug layer.
    pub fn new(preference: PowerPreference) -> Result<D3D12Context> {
        let debug = env_flag("CANVAS_D3D_DEBUG");
        if debug {
            let mut layer: Option<ID3D12Debug> = None;
            if unsafe { D3D12GetDebugInterface(&mut layer) }.is_ok() {
                if let Some(layer) = layer {
                    unsafe { layer.EnableDebugLayer() };
                }
            }
        }

        let flags = if debug {
            DXGI_CREATE_FACTORY_DEBUG
        } else {
            DXGI_CREATE_FACTORY_FLAGS(0)
        };
        let factory: IDXGIFactory4 = unsafe { CreateDXGIFactory2(flags) }?;

        if !env_flag("CANVAS_FORCE_WARP") {
            if let Some((adapter, device)) = Self::hardware_device(&factory, preference) {
                let queue = Self::create_queue(&device)?;
                return Ok(D3D12Context {
                    factory,
                    adapter,
                    device,
                    queue,
                    is_warp: false,
                    on12: Default::default(),
                });
            }
        }

        let adapter: IDXGIAdapter1 = unsafe { factory.EnumWarpAdapter() }?;
        let device = Self::create_device(&adapter)?;
        let queue = Self::create_queue(&device)?;
        Ok(D3D12Context {
            factory,
            adapter,
            device,
            queue,
            is_warp: true,
            on12: Default::default(),
        })
    }

    fn hardware_device(
        factory: &IDXGIFactory4,
        preference: PowerPreference,
    ) -> Option<(IDXGIAdapter1, ID3D12Device)> {
        // IDXGIFactory6 (Windows 10 1803+) orders adapters by the requested preference; older
        // systems fall back to plain enumeration order, which lists the primary adapter first.
        let by_preference = factory.cast::<IDXGIFactory6>().ok();
        for index in 0.. {
            let adapter: Result<IDXGIAdapter1> = match &by_preference {
                Some(factory) => unsafe { factory.EnumAdapterByGpuPreference(index, preference.dxgi()) },
                None => unsafe { factory.EnumAdapters1(index) },
            };
            let Ok(adapter) = adapter else { break };

            let Ok(desc) = (unsafe { adapter.GetDesc1() }) else { continue };
            if DXGI_ADAPTER_FLAG(desc.Flags as i32).contains(DXGI_ADAPTER_FLAG_SOFTWARE) {
                continue;
            }
            if let Ok(device) = Self::create_device(&adapter) {
                return Some((adapter, device));
            }
        }
        None
    }

    fn create_device(adapter: &IDXGIAdapter1) -> Result<ID3D12Device> {
        let mut device: Option<ID3D12Device> = None;
        unsafe { D3D12CreateDevice(adapter, D3D_FEATURE_LEVEL_11_0, &mut device) }?;
        device.ok_or_else(windows::core::Error::empty)
    }

    fn create_queue(device: &ID3D12Device) -> Result<ID3D12CommandQueue> {
        let desc = D3D12_COMMAND_QUEUE_DESC {
            Type: D3D12_COMMAND_LIST_TYPE_DIRECT,
            ..Default::default()
        };
        unsafe { device.CreateCommandQueue(&desc) }
    }

    pub fn factory(&self) -> &IDXGIFactory4 {
        &self.factory
    }

    pub fn adapter(&self) -> &IDXGIAdapter1 {
        &self.adapter
    }

    pub fn device(&self) -> &ID3D12Device {
        &self.device
    }

    pub fn queue(&self) -> &ID3D12CommandQueue {
        &self.queue
    }

    pub fn is_warp(&self) -> bool {
        self.is_warp
    }

    /// The D3D11On12 device on this device and queue, made on first use.
    pub fn d3d11_on_12(&self) -> Option<&D3D11On12> {
        self.on12
            .get_or_init(|| {
                use windows::Win32::Graphics::Direct3D11::{ID3D11Device, ID3D11DeviceContext, D3D11_CREATE_DEVICE_BGRA_SUPPORT};
                let queue: windows::core::IUnknown = self.queue.cast().ok()?;
                let (mut device, mut context): (Option<ID3D11Device>, Option<ID3D11DeviceContext>) = (None, None);
                let created = unsafe {
                    windows::Win32::Graphics::Direct3D11on12::D3D11On12CreateDevice(
                        &self.device,
                        D3D11_CREATE_DEVICE_BGRA_SUPPORT.0 as u32,
                        None,
                        Some(&[Some(queue)]),
                        0,
                        Some(&mut device),
                        Some(&mut context),
                        None,
                    )
                };
                if let Err(error) = created {
                    log::error!("canvas: D3D11On12CreateDevice failed: {error}");
                    return None;
                }
                let device = device?;
                let on12 = device.cast().ok()?;
                Some(D3D11On12 { device, context: context?, on12 })
            })
            .as_ref()
    }

    /// The device was removed (a driver update or reset, the GPU gone, `simulate_removal`):
    /// everything made on it is lost and a new device has to be created.
    pub fn is_removed(&self) -> bool {
        unsafe { self.device.GetDeviceRemovedReason() }.is_err()
    }

    /// Removes the device, as a driver reset would (tests). `false` before Windows 10 1809.
    pub fn simulate_removal(&self) -> bool {
        match self.device.cast::<ID3D12Device5>() {
            Ok(device) => {
                unsafe { device.RemoveDevice() };
                true
            }
            Err(_) => false,
        }
    }

    /// `simulate_removal` on this thread's shared device, if there is one.
    pub fn simulate_shared_removal() -> bool {
        SHARED.with(|shared| shared.borrow().as_ref().is_some_and(|context| context.simulate_removal()))
    }

    pub fn backend_context(&self) -> skia_safe::gpu::d3d::BackendContext {
        skia_safe::gpu::d3d::BackendContext {
            adapter: self.adapter.clone(),
            device: self.device.clone(),
            queue: self.queue.clone(),
            memory_allocator: None,
            protected_context: skia_safe::gpu::Protected::No,
        }
    }

    /// A Skia context on this device. The device must outlive it; callers keep the owning
    /// `Rc<D3D12Context>` next to the `DirectContext` and drop the `DirectContext` first.
    pub fn make_direct_context(&self) -> Option<skia_safe::gpu::DirectContext> {
        unsafe { skia_safe::gpu::direct_contexts::make_d3d(&self.backend_context(), None) }
    }
}
