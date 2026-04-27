use objc2::ffi::BOOL;
use objc2::msg_send;
use objc2::rc::Retained;
use objc2::runtime::{AnyObject, ProtocolObject};
use objc2_foundation::{NSAutoreleasePool, NSObject};
use objc2_metal::{
    MTLBlitCommandEncoder, MTLBlitOption, MTLBuffer, MTLCommandBuffer, MTLCommandEncoder,
    MTLCommandQueue, MTLCreateSystemDefaultDevice, MTLDevice, MTLDrawable, MTLOrigin,
    MTLPixelFormat, MTLRegion, MTLResourceOptions, MTLSize, MTLTexture,
};
use objc2_quartz_core::{CAMetalDrawable, CAMetalLayer};
use std::os::raw::c_void;

#[derive(Debug)]
pub struct MetalTexture {
    pool: Retained<NSAutoreleasePool>,
    texture: Retained<ProtocolObject<dyn MTLTexture>>,
}

impl MetalTexture {
    pub fn width(&self) -> u64 {
        self.texture.width() as u64
    }

    pub fn height(&self) -> u64 {
        self.texture.height() as u64
    }

    pub unsafe fn from_texture(texture: *mut c_void) -> Option<Self> {
        if texture.is_null() {
            return None;
        }

        let obj = texture as *mut AnyObject;

        match Retained::retain(obj.cast()) {
            Some(texture) => Some(Self {
                texture,
                pool: NSAutoreleasePool::new(),
            }),
            None => None,
        }
    }

    pub fn texture(&self) -> *mut c_void {
        Retained::as_ptr(&self.texture) as *mut c_void
    }
}

#[derive(Debug)]
pub struct MetalContext {
    queue: Retained<ProtocolObject<dyn MTLCommandQueue>>,
    device: Retained<ProtocolObject<dyn MTLDevice>>,
    pool: Retained<NSAutoreleasePool>,
    layer: Retained<CAMetalLayer>,
    view: Option<Retained<NSObject>>,
    current_drawable: Option<Retained<ProtocolObject<dyn CAMetalDrawable>>>,
    is_offscreen: bool,
}

impl MetalContext {
    pub fn new_release_pool() -> Retained<NSAutoreleasePool> {
        unsafe { NSAutoreleasePool::new() }
    }

    pub fn is_offscreen(&self) -> bool {
        self.is_offscreen
    }
    pub fn new(view: *mut c_void) -> Self {
        let pool = unsafe { NSAutoreleasePool::new() };
        let device = MTLCreateSystemDefaultDevice().expect("no Metal device");
        let queue = device.newCommandQueue().unwrap();
        let view = unsafe { Retained::from_raw(view as _).unwrap() };

        let layer = if cfg!(target_os = "macos") {
            let layer: Retained<CAMetalLayer> = unsafe { msg_send![&view, layer] };

            if let Some(clazz) = objc2::runtime::AnyClass::get(c"CAMetalLayer") {
                let is_metal: BOOL = unsafe { msg_send![&layer, isKindOfClass: clazz] };
                if is_metal == objc2::ffi::NO {
                    let _: () = unsafe { msg_send![&view,  setWantsLayer: objc2::ffi::YES] };
                    let mtl_layer = CAMetalLayer::new();
                    mtl_layer.setDevice(Some(&device));
                    mtl_layer.setPixelFormat(MTLPixelFormat::BGRA8Unorm);
                    mtl_layer.setPresentsWithTransaction(false);
                    mtl_layer.setFramebufferOnly(false);
                    mtl_layer.setOpaque(false);

                    let _: () = unsafe { msg_send![&view, setLayer: &*mtl_layer ] };

                    mtl_layer
                } else {
                    layer
                }
            } else {
                layer
            }
        } else {
            let layer: Retained<CAMetalLayer> = unsafe { msg_send![&view, layer] };
            layer
        };

        let current_drawable = layer.nextDrawable().map(|drawable| drawable.to_owned());

        Self {
            queue,
            device,
            pool,
            layer,
            view: Some(view),
            current_drawable,
            is_offscreen: false,
        }
    }

    pub fn new_offscreen(width: f32, height: f32) -> Self {
        let pool = unsafe { NSAutoreleasePool::new() };
        let device = MTLCreateSystemDefaultDevice().expect("no Metal device");
        let queue = device.newCommandQueue().unwrap();
        let layer = CAMetalLayer::new();

        let mut size = layer.drawableSize();
        size.width = width as _;
        size.height = height as _;
        layer.setDrawableSize(size);

        layer.setDevice(Some(&device));
        layer.setPixelFormat(MTLPixelFormat::BGRA8Unorm);
        layer.setPresentsWithTransaction(false);
        layer.setOpaque(false);
        layer.setFramebufferOnly(false);

        let current_drawable = layer.nextDrawable().map(|drawable| drawable.to_owned());

        Self {
            queue,
            device,
            pool,
            layer,
            view: None,
            current_drawable,
            is_offscreen: true,
        }
    }

    pub unsafe fn new_device_queue(
        view: *mut c_void,
        device: *mut c_void,
        queue: *mut c_void,
    ) -> Self {
        let pool = NSAutoreleasePool::new();
        let device: Retained<ProtocolObject<dyn MTLDevice>> =
            Retained::retain((device as *mut AnyObject).cast()).unwrap();
        // view is non null
        let view: Retained<NSObject> = Retained::retain((view as *mut AnyObject).cast()).unwrap();
        let queue: Retained<ProtocolObject<dyn MTLCommandQueue>> =
            Retained::retain((queue as *mut AnyObject).cast()).unwrap();
        let layer: Retained<CAMetalLayer> = unsafe { msg_send![&*view, layer] };
        let current_drawable = layer.nextDrawable().map(|drawable| drawable.to_owned());

        Self {
            queue,
            device,
            pool,
            layer,
            view: Some(view),
            current_drawable,
            is_offscreen: false,
        }
    }

    pub fn queue(&self) -> *mut c_void {
        Retained::as_ptr(&self.queue) as _
    }

    pub fn device(&self) -> *mut c_void {
        Retained::as_ptr(&self.device) as _
    }

    pub fn sample_count(&self) -> usize {
        match &self.view {
            None => 1,
            Some(view) => {
                let count: usize = unsafe { msg_send![view, sampleCount] };
                count
            }
        }
    }

    pub fn present(&mut self) {
        if let Some(view) = self.view.as_ref() {
            let _: () = unsafe { msg_send![view, present] };
        }
    }

    pub fn draw(&self) {
        if let Some(view) = self.view.as_ref() {
            let _: () = unsafe { msg_send![view, draw] };
        }
    }

    pub unsafe fn set_view(&mut self, width: f32, height: f32, value: *mut c_void) {
        if value.is_null() {
            let layer = CAMetalLayer::new();

            let mut size = layer.drawableSize();
            size.width = width as _;
            size.height = height as _;
            layer.setDrawableSize(size);

            layer.setDevice(Some(self.device.as_ref()));
            layer.setPixelFormat(MTLPixelFormat::BGRA8Unorm);
            layer.setPresentsWithTransaction(false);
            layer.setOpaque(false);

            let current_drawable = layer.nextDrawable().map(|drawable| drawable.to_owned());

            self.current_drawable = current_drawable;
            self.layer = layer;

            self.view = None;
        } else {
            let view = unsafe { Retained::from_raw(value as _).unwrap() };

            let layer = if cfg!(target_os = "macos") {
                let layer: Retained<CAMetalLayer> = unsafe { msg_send![&view, layer] };

                if let Some(clazz) = objc2::runtime::AnyClass::get(c"CAMetalLayer") {
                    let is_metal: BOOL = unsafe { msg_send![&layer, isKindOfClass: clazz] };
                    if is_metal == objc2::ffi::YES {
                        let _: () = unsafe { msg_send![&view,  setWantsLayer: objc2::ffi::YES] };
                        let mtl_layer = CAMetalLayer::new();
                        mtl_layer.setDevice(Some(self.device.as_ref()));
                        mtl_layer.setPixelFormat(MTLPixelFormat::BGRA8Unorm);
                        mtl_layer.setPresentsWithTransaction(false);
                        mtl_layer.setOpaque(false);

                        let _: () = unsafe { msg_send![&view, setLayer: &*mtl_layer ] };

                        mtl_layer
                    } else {
                        layer
                    }
                } else {
                    layer
                }
            } else {
                let layer: Retained<CAMetalLayer> = unsafe { msg_send![&view, layer] };
                layer
            };
            let current_drawable = layer.nextDrawable().map(|drawable| drawable.to_owned());
            self.current_drawable = current_drawable;
            self.layer = layer;
            self.view = Some(view);
        }
    }

    pub unsafe fn view(&self) -> *mut c_void {
        match &self.view {
            None => std::ptr::null_mut(),
            Some(view) => Retained::into_raw(view.clone()) as _,
        }
    }

    pub fn layer(&self) -> *mut c_void {
        Retained::as_ptr(&self.layer) as _
    }

    pub fn present_drawable(&mut self) {
        let _ = unsafe { NSAutoreleasePool::new() };
        if let Some(drawable) = self.current_drawable.take() {
            if let Some(cmd) = self.queue.commandBuffer() {
                cmd.presentDrawable(drawable.as_ref());
                cmd.commit();
            }
        }
    }

    pub fn current_drawable(&mut self) -> Option<&Retained<ProtocolObject<dyn CAMetalDrawable>>> {
        if self.current_drawable.is_none() {
            self.current_drawable = unsafe {
                self.layer
                    .nextDrawable()
                    .map(|drawable| drawable.to_owned())
            }
        }
        self.current_drawable.as_ref()
    }

    pub fn next_drawable(&mut self) -> Option<&Retained<ProtocolObject<dyn CAMetalDrawable>>> {
        let _ = unsafe { NSAutoreleasePool::new() };
        self.current_drawable = unsafe {
            self.layer
                .nextDrawable()
                .map(|drawable| drawable.to_owned())
        };
        self.current_drawable.as_ref()
    }

    pub fn drawable(&self) -> Option<Retained<ProtocolObject<dyn CAMetalDrawable>>> {
        self.layer.nextDrawable()
    }

    pub fn drawable_size(&self) -> (f64, f64) {
        let size = self.layer.drawableSize();
        (size.width, size.height)
    }

    pub fn set_drawable_size(&self, width: f64, height: f64) {
        let mut size = self.layer.drawableSize();
        size.width = width;
        size.height = height;
        self.layer.setDrawableSize(size)
    }

    fn blit(
        device: Retained<ProtocolObject<dyn MTLDevice>>,
        queue: Retained<ProtocolObject<dyn MTLCommandQueue>>,
        drawable: &Retained<ProtocolObject<dyn CAMetalDrawable>>,
    ) -> Option<Vec<u8>> {
        unsafe {
            if drawable.presentedTime() == 0.0 {
                let texture = drawable.texture();
                let texture_width = texture.width();
                let length = 4 * texture_width * texture.height();
                let buffer = device
                    .newBufferWithLength_options(length, MTLResourceOptions::StorageModeShared)?;
                let command_buffer = queue.commandBuffer()?;
                let blit_encoder = command_buffer.blitCommandEncoder()?;
                let region = MTLRegion {
                    origin: MTLOrigin { x: 0, y: 0, z: 0 },
                    size: MTLSize {
                        width: texture_width,
                        height: texture.height(),
                        depth: 1,
                    },
                };

                blit_encoder.copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toBuffer_destinationOffset_destinationBytesPerRow_destinationBytesPerImage_options(
                    &texture,
                    0,
                    0,
                    region.origin,
                    region.size,
                    &buffer,
                    0,
                    4 * texture_width,
                    0,
                    MTLBlitOption::empty(),
                );
                blit_encoder.endEncoding();
                command_buffer.commit();
                command_buffer.waitUntilCompleted();

                let ret = std::slice::from_raw_parts(buffer.contents().as_ptr() as *mut u8, length as usize);

                return Some(ret.to_vec());
            }
        }
        None
    }

    pub fn snapshot(&mut self) -> Option<Vec<u8>> {
        let device = self.device.clone();
        let queue = self.queue.clone();
        match self.current_drawable.as_ref() {
            None => match self.next_drawable() {
                Some(drawable) => Self::blit(device, queue, drawable),
                _ => None,
            },
            Some(drawable) => Self::blit(device, queue, drawable),
        }
    }
}
