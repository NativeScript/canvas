use std::borrow::Cow;
use std::os::raw::c_char;
use std::sync::Arc;

use crate::webgpu::prelude::label_to_ptr;

//use wgpu_core::gfx_select;
use super::enums::CanvasQueryType;
use super::gpu::CanvasWebGPUInstance;

pub struct CanvasGPUQuerySet {
    pub(super) label: Option<Cow<'static, str>>,
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) query: wgpu_core::id::QuerySetId,
    pub(crate) type_: CanvasQueryType,
    pub(super) count: u32,
}

impl Drop for CanvasGPUQuerySet {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            let global = self.instance.global();
            gfx_select!(self.query => global.query_set_drop(self.query));
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_query_set_get_label(
    query_set: *const CanvasGPUQuerySet,
) -> *mut c_char {
    if query_set.is_null() {
        return std::ptr::null_mut();
    }

    let query_set = &*query_set;
    label_to_ptr(query_set.label.clone())
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_query_set_get_count(
    query_set: *const CanvasGPUQuerySet,
) -> u32 {
    if query_set.is_null() {
        return 0;
    }

    let query_set = &*query_set;
    query_set.count
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_query_set_get_type(
    query_set: *const CanvasGPUQuerySet,
) -> CanvasQueryType {
    let query_set = &*query_set;
    query_set.type_
}

#[no_mangle]
#[allow(unused)]
pub unsafe extern "C" fn canvas_native_webgpu_query_set_destroy(
    query_set: *const CanvasGPUQuerySet,
) {
    // todo
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_query_set_reference(
    query_set: *const CanvasGPUQuerySet,
) {
    if query_set.is_null() {
        return;
    }

    Arc::increment_strong_count(query_set);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_query_set_release(
    query_set: *const CanvasGPUQuerySet,
) {
    if query_set.is_null() {
        return;
    }

    Arc::decrement_strong_count(query_set);
}
