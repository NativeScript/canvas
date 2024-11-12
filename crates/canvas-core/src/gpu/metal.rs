use icrate::objc2::__framework_prelude::Retained;
use icrate::objc2::msg_send;
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
    view: Retained<NSObject>,
    current_drawable: Option<MetalDrawable>,
}

impl MetalContext {
    pub fn new_release_pool() -> Retained<NSAutoreleasePool> {
        unsafe { NSAutoreleasePool::new() }
    }
    pub fn new(view: *mut c_void) -> Self {
        let pool = unsafe { NSAutoreleasePool::new() };
        let device = metal::Device::system_default().expect("no Metal device");
        let queue = device.new_command_queue();
        let view = unsafe { Retained::from_raw(view as _).unwrap() };
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
            view,
            current_drawable,
        }
    }

    pub unsafe fn new_device_queue(view: *mut c_void, device: *mut c_void, queue: *mut c_void) -> Self {
        let pool = NSAutoreleasePool::new();
        let device = metal::Device::from_ptr(device as _);
        let view = icrate::objc2::rc::Retained::from_raw(view as _).unwrap();
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
            view,
            current_drawable,
        }
    }

    pub fn queue(&self) -> *mut c_void {
        self.queue.as_ptr() as _
    }

    pub fn device(&self) -> *mut c_void {
        self.device.as_ptr() as _
    }

    pub fn sample_count(&self) -> usize {
        let count: usize = unsafe { msg_send![&self.view, sampleCount] };
        count
    }

    pub fn present(&self) {
        let _: () = unsafe { msg_send![&self.view, present] };
    }

    pub fn draw(&self) {
        let _: () = unsafe { msg_send![&self.view, draw] };
    }

    pub unsafe fn view(&self) -> *mut c_void {
        Retained::into_raw(self.view.clone()) as _
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
