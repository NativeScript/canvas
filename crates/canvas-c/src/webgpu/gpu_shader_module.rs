use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Arc;

use crate::webgpu::prelude::label_to_ptr;

//use wgpu_core::gfx_select;
use super::gpu::CanvasWebGPUInstance;

pub struct CanvasGPUShaderModule {
    pub(crate) label: Option<Cow<'static, str>>,
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) module: Arc<wgpu_core::pipeline::ShaderModule>,
    pub(crate) compilation_info: CanvasGPUCompilationInfo,
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_shader_module_get_label(
    shader_module: *const CanvasGPUShaderModule,
) -> *mut c_char {
    if shader_module.is_null() {
        return std::ptr::null_mut();
    }

    let shader_module = &*shader_module;

    label_to_ptr(shader_module.label.clone())
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_shader_module_reference(
    shader_module: *const CanvasGPUShaderModule,
) {
    if shader_module.is_null() {
        return;
    }
    Arc::increment_strong_count(shader_module);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_shader_module_release(
    shader_module: *const CanvasGPUShaderModule,
) {
    if shader_module.is_null() {
        return;
    }
    Arc::decrement_strong_count(shader_module);
}

#[derive(Clone)]
pub struct CanvasGPUCompilationInfo {
    messages: Arc<Vec<CanvasGPUCompilationMessage>>,
}

impl CanvasGPUCompilationInfo {
    pub fn new(messages: Vec<CanvasGPUCompilationMessage>) -> Self {
        Self {
            messages: Arc::new(messages),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CanvasGPUCompilationMessageType {
    Error,
    Warning,
    Info,
}

impl CanvasGPUCompilationMessageType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Info => "info",
        }
    }
}

#[derive(Clone)]
pub struct CanvasGPUCompilationMessage {
    message: Arc<CString>,

    r#type: CanvasGPUCompilationMessageType,

    line_num: u64,

    line_pos: u64,

    offset: u64,

    length: u64,
}

impl CanvasGPUCompilationMessage {
    pub unsafe fn msg(&self) -> *const c_char {
        self.message.as_ptr()
    }
    pub fn message(&self) -> &CStr {
        self.message.as_ref()
    }

    pub fn type_(&self) -> CanvasGPUCompilationMessageType {
        self.r#type
    }

    pub fn ty(&self) -> &'static str {
        self.r#type.as_str()
    }

    #[inline]
    pub fn line_num(&self) -> u64 {
        self.line_num
    }

    #[inline]
    pub fn line_pos(&self) -> u64 {
        self.line_pos
    }

    #[inline]
    pub fn offset(&self) -> u64 {
        self.offset
    }

    #[inline]
    pub fn length(&self) -> u64 {
        self.length
    }

    /// wgpu-core now attaches compilation messages to the module itself
    /// (`ShaderModule::compilation_info`) rather than handing back an error, so
    /// this builds our message from one of those. `SourceLocation` counts UTF-8
    /// bytes; GPUCompilationMessage is specified in UTF-16 code units, so the
    /// offsets are converted here exactly as the error-based path did.
    pub fn from_compilation_message(
        message: &wgt::CompilationMessage,
        source: &str,
    ) -> Self {
        let text = message.message.replace('\0', "<NUL>");
        let text = Arc::new(
            CString::new(text).unwrap_or_else(|_| CString::new("shader compilation message").unwrap()),
        );

        let r#type = match message.message_type {
            wgt::CompilationMessageType::Error => CanvasGPUCompilationMessageType::Error,
            wgt::CompilationMessageType::Warning => CanvasGPUCompilationMessageType::Warning,
            wgt::CompilationMessageType::Info => CanvasGPUCompilationMessageType::Info,
        };

        match message.location {
            Some(loc) => {
                let len_utf16 = |s: &str| s.chars().map(|c| c.len_utf16() as u64).sum();
                let start = (loc.offset as usize).min(source.len());
                let end = (start + loc.length as usize).min(source.len());
                let line_start = source[0..start].rfind('\n').map(|pos| pos + 1).unwrap_or(0);

                Self {
                    message: text,
                    r#type,
                    line_num: loc.line_number.into(),
                    line_pos: len_utf16(&source[line_start..start]) + 1,
                    offset: len_utf16(&source[0..start]),
                    length: len_utf16(&source[start..end]),
                }
            }
            None => Self {
                message: text,
                r#type,
                line_num: 0,
                line_pos: 0,
                offset: 0,
                length: 0,
            },
        }
    }

    pub fn new(error: &wgpu_core::pipeline::CreateShaderModuleError, source: &str) -> Self {
        let message = Arc::new(CString::new(error.to_string()).unwrap());

        let loc = match error {
            wgpu_core::pipeline::CreateShaderModuleError::Parsing(e) => e.inner.location(source),
            wgpu_core::pipeline::CreateShaderModuleError::Validation(e) => e.inner.location(source),
            _ => None,
        };

        match loc {
            Some(loc) => {
                let len_utf16 = |s: &str| s.chars().map(|c| c.len_utf16() as u64).sum();

                let start = loc.offset as usize;

                // Naga reports a `line_pos` using UTF-8 bytes, so we cannot use it.
                let line_start = source[0..start].rfind('\n').map(|pos| pos + 1).unwrap_or(0);
                let line_pos = len_utf16(&source[line_start..start]) + 1;

                Self {
                    message,
                    r#type: CanvasGPUCompilationMessageType::Error,
                    line_num: loc.line_number.into(),
                    line_pos,
                    offset: len_utf16(&source[0..start]),
                    length: len_utf16(&source[start..start + loc.length as usize]),
                }
            }
            _ => Self {
                message,
                r#type: CanvasGPUCompilationMessageType::Error,
                line_num: 0,
                line_pos: 0,
                offset: 0,
                length: 0,
            },
        }
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_device_create_shader_module_get_compilation_info(
    shader_module: *const CanvasGPUShaderModule,
) -> *mut CanvasGPUCompilationInfo {
    if shader_module.is_null() {
        return std::ptr::null_mut();
    }

    let shader_module = unsafe { &*shader_module };
    Box::into_raw(Box::new(shader_module.compilation_info.clone()))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_compilation_info_get_messages_count(
    info: *mut CanvasGPUCompilationInfo,
) -> usize {
    if info.is_null() {
        return 0;
    }

    let info = unsafe { &*info };
    info.messages.len()
}


#[no_mangle]
pub extern "C" fn canvas_native_webgpu_compilation_info_get_message_at(
    info: *mut CanvasGPUCompilationInfo,
    index: usize,
) -> *mut CanvasGPUCompilationMessage {
    if info.is_null() {
        return std::ptr::null_mut();
    }

    let info = unsafe { &*info };
    match info.messages.get(index) {
        Some(info) => Box::into_raw(Box::new(info.clone())),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_compilation_info_release(
    info: *mut CanvasGPUCompilationInfo,
) {
    if info.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(info) };
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_compilation_message_release(
    message: *mut CanvasGPUCompilationMessage,
) {
    if message.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(message) };
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_compilation_message_get_message(
    message: *mut CanvasGPUCompilationMessage,
) -> *const c_char {
    if message.is_null() {
        return std::ptr::null_mut();
    }
    let message = unsafe { &*message };
    message.msg()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_compilation_message_get_type(
    message: *mut CanvasGPUCompilationMessage,
) -> CanvasGPUCompilationMessageType {
    if message.is_null() {
        return CanvasGPUCompilationMessageType::Error;
    }
    let message = unsafe { &*message };
    message.type_()
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_compilation_message_get_line_num(
    message: *mut CanvasGPUCompilationMessage,
) -> u64 {
    if message.is_null() {
        return 0;
    }
    let message = unsafe { &*message };
    message.line_num
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_compilation_message_get_line_pos(
    message: *mut CanvasGPUCompilationMessage,
) -> u64 {
    if message.is_null() {
        return 0;
    }
    let message = unsafe { &*message };
    message.line_pos
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_compilation_message_get_offset(
    message: *mut CanvasGPUCompilationMessage,
) -> u64 {
    if message.is_null() {
        return 0;
    }
    let message = unsafe { &*message };
    message.offset
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_compilation_message_get_length(
    message: *mut CanvasGPUCompilationMessage,
) -> u64 {
    if message.is_null() {
        return 0;
    }
    let message = unsafe { &*message };
    message.length
}
