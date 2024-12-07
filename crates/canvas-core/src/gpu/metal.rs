use foreign_types_shared::ForeignTypeRef;
use icrate::objc2;
use icrate::objc2::__framework_prelude::Retained;
use icrate::objc2::ffi::BOOL;
use icrate::objc2::{msg_send, msg_send_id};
use metal::foreign_types::ForeignType;
use metal::{MTLResourceOptions, MetalDrawable, MetalDrawableRef};
use objc2_foundation::{NSAutoreleasePool, NSObject};
use std::os::raw::c_void;

#[derive(Debug)]
pub struct MetalContext {
    queue: metal::CommandQueue,
    device: metal::Device,
    pool: Retained<NSAutoreleasePool>,
    layer: metal::MetalLayer,
    view: Option<Retained<NSObject>>,
    current_drawable: Option<MetalDrawable>,
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
        let device = metal::Device::system_default().expect("no Metal device");
        let queue = device.new_command_queue();
        let view = unsafe { Retained::from_raw(view as _).unwrap() };

        let layer = if cfg!(target_os = "macos") {
            let mut layer: *mut NSObject = unsafe { msg_send![&view, layer] };

            if let Some(clazz) = objc2::runtime::AnyClass::get("CAMetalLayer") {

                let is_metal: BOOL = unsafe { msg_send![layer, isKindOfClass: clazz] };
                if is_metal == objc2::ffi::NO {
                    let _: () = unsafe {
                        msg_send![&view,  setWantsLayer: objc2::ffi::YES]
                    };
                    let mtl_layer = metal::MetalLayer::new();
                    mtl_layer.set_device(&device);
                    mtl_layer.set_pixel_format(metal::MTLPixelFormat::BGRA8Unorm);
                    mtl_layer.set_presents_with_transaction(false);
                    mtl_layer.set_opaque(false);

                    let _: () = unsafe {
                        msg_send![&view, setLayer: mtl_layer.as_ptr() as *mut NSObject ]
                    };

                    mtl_layer
                } else {
                    unsafe { metal::MetalLayer::from_ptr(layer as _) }
                }
            } else {
                unsafe { metal::MetalLayer::from_ptr(layer as _) }
            }
        } else {
            let mut layer: *mut NSObject = unsafe { msg_send![&view, layer] };
            unsafe { metal::MetalLayer::from_ptr(layer as _) }
        };
        let current_drawable = layer.next_drawable().map(|drawable| {
            drawable.to_owned()
        });

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
        let device = metal::Device::system_default().expect("no Metal device");
        let queue = device.new_command_queue();
        let layer = metal::MetalLayer::new();

        let mut size = layer.drawable_size();
        size.width = width as _;
        size.height = height as _;
        layer.set_drawable_size(size);

        layer.set_device(&device);
        layer.set_pixel_format(metal::MTLPixelFormat::BGRA8Unorm);
        layer.set_presents_with_transaction(false);
        layer.set_opaque(false);


        let current_drawable = layer.next_drawable().map(|drawable| {
            drawable.to_owned()
        });

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

    pub unsafe fn new_device_queue(view: *mut c_void, device: *mut c_void, queue: *mut c_void) -> Self {
        let pool = NSAutoreleasePool::new();
        let device = metal::Device::from_ptr(device as _);
        let view = Retained::from_raw(view as _).unwrap();
        let queue = metal::CommandQueue::from_ptr(queue as _);
        let layer: *mut NSObject = unsafe { msg_send![&view, layer] };
        let layer = unsafe { metal::MetalLayer::from_ptr(layer as _) };
        let current_drawable = layer.next_drawable().map(|drawable| {
            drawable.to_owned()
        });

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
        self.queue.as_ptr() as _
    }

    pub fn device(&self) -> *mut c_void {
        self.device.as_ptr() as _
    }

    pub fn sample_count(&self) -> usize {
        match &self.view {
            None => {
                1
            }
            Some(view) => {
                let count: usize = unsafe { msg_send![view, sampleCount] };
                count
            }
        }
    }

    pub fn present(&mut self) {
        if let Some(view) = self.view.as_ref() {
            let _: () = unsafe { msg_send![view, present] };
        } else {}
    }

    pub fn draw(&self) {
        if let Some(view) = self.view.as_ref() {
            let _: () = unsafe { msg_send![view, draw] };
        }
    }

    pub unsafe fn set_view(&mut self, width: f32, height: f32, value: *mut c_void) {
        if value.is_null() {
            let layer = metal::MetalLayer::new();

            let mut size = layer.drawable_size();
            size.width = width as _;
            size.height = height as _;
            layer.set_drawable_size(size);

            layer.set_device(self.device.as_ref());
            layer.set_pixel_format(metal::MTLPixelFormat::BGRA8Unorm);
            layer.set_presents_with_transaction(false);
            layer.set_opaque(false);


            let current_drawable = layer.next_drawable().map(|drawable| {
                drawable.to_owned()
            });

            self.current_drawable = current_drawable;
            self.layer = layer;

            self.view = None;
        } else {
            let view = unsafe { Retained::from_raw(value as _).unwrap() };

            let layer = if cfg!(target_os = "macos") {
                let mut layer: *mut NSObject = unsafe { msg_send![&view, layer] };

                if let Some(clazz) = objc2::runtime::AnyClass::get("CAMetalLayer") {
                    let is_metal: BOOL = unsafe { msg_send![layer, isKindOfClass: clazz] };
                    if  is_metal == objc2::ffi::YES {
                        let _: () = unsafe {
                            msg_send![&view,  setWantsLayer: objc2::ffi::YES]
                        };
                        let mtl_layer = metal::MetalLayer::new();
                        mtl_layer.set_device(self.device.as_ref());
                        mtl_layer.set_pixel_format(metal::MTLPixelFormat::BGRA8Unorm);
                        mtl_layer.set_presents_with_transaction(false);
                        mtl_layer.set_opaque(false);

                        let _: () = unsafe {
                            msg_send![&view, setLayer: mtl_layer.as_ptr() as *mut NSObject ]
                        };

                        mtl_layer
                    } else {
                        unsafe { metal::MetalLayer::from_ptr(layer as _) }
                    }
                } else {
                    unsafe { metal::MetalLayer::from_ptr(layer as _) }
                }
            } else {
                let mut layer: *mut NSObject = unsafe { msg_send![&view, layer] };
                unsafe { metal::MetalLayer::from_ptr(layer as _) }
            };
            let current_drawable = layer.next_drawable().map(|drawable| {
                drawable.to_owned()
            });
            self.current_drawable = current_drawable;
            self.layer = layer;
            self.view = Some(view);
        }
    }

    pub unsafe fn view(&self) -> *mut c_void {
        match &self.view {
            None => std::ptr::null_mut(),
            Some(view) => {
                Retained::into_raw(view.clone()) as _
            }
        }
    }

    pub fn layer(&self) -> *mut c_void {
        self.layer.as_ptr() as _
    }

    pub fn present_drawable(&mut self) {
        let _ = unsafe { NSAutoreleasePool::new() };
        if let Some(drawable) = self.current_drawable.take() {
            let cmd = self.queue.new_command_buffer();
            cmd.present_drawable(&*drawable);
            cmd.commit();
        }
    }

    pub fn current_drawable(&mut self) -> Option<&MetalDrawable> {
        if self.current_drawable.is_none() {
            self.current_drawable = unsafe {
                self.layer.next_drawable().map(|drawable| {
                    drawable.to_owned()
                })
            }
        }
        self.current_drawable.as_ref()
    }

    pub fn next_drawable(&mut self) -> Option<&MetalDrawable> {
        let _ = unsafe { NSAutoreleasePool::new() };
        self.current_drawable = unsafe {
            self.layer.next_drawable().map(|drawable| {
                drawable.to_owned()
            })
        };
        self.current_drawable.as_ref()
    }

    pub fn drawable(&self) -> Option<&MetalDrawableRef> {
        self.layer.next_drawable()
    }

    pub fn drawable_size(&self) -> (f64, f64) {
        let size = self.layer.drawable_size();
        (size.width, size.height)
    }

    pub fn set_drawable_size(&self, width: f64, height: f64) {
        let mut size = self.layer.drawable_size();
        size.width = width;
        size.height = height;
        self.layer.set_drawable_size(size)
    }

    fn blit(device: metal::Device, queue: metal::CommandQueue, drawable: &MetalDrawable) -> Option<Vec<u8>> {
        unsafe {
            if drawable.presented_time() == 0.0 {
                let texture = drawable.texture();
                let length = 4 * texture.width() * texture.height();
                let buffer = device.new_buffer(length, MTLResourceOptions::StorageModeShared);
                let command_buffer = queue.new_command_buffer();
                let blit_encoder = command_buffer.new_blit_command_encoder();
                let region = metal::MTLRegion::new_2d(0, 0, texture.width(), texture.height());
                blit_encoder.copy_from_texture_to_buffer(
                    texture, 0, 0, region.origin, region.size, &buffer,
                    0, 4 * texture.width(), 0, metal::MTLBlitOption::empty(),
                );
                blit_encoder.end_encoding();
                command_buffer.commit();
                command_buffer.wait_until_completed();

                let ret = std::slice::from_raw_parts(buffer.contents().cast(), length as usize);

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
                Some(drawable) => {
                    Self::blit(device, queue, drawable)
                }
                _ => None
            }
            Some(drawable) => {
                Self::blit(device, queue, drawable)
            }
        }
    }
}
