#[cfg(target_os = "ios")]
mod ios;
#[cfg(target_os = "ios")]
pub use ios::*;

#[cfg(target_os = "macos")]
mod mac;

#[cfg(target_os = "macos")]
pub use mac::*;

#[cfg(target_os = "android")]
mod android;

#[cfg(target_os = "android")]
pub use android::*;


pub fn get_shader_info_log(shader: u32) -> String {
    let mut length = 0i32;
    unsafe { gl_bindings::GetShaderiv(shader, gl_bindings::INFO_LOG_LENGTH, &mut length) }

    if length == 0 {
        return String::new();
    }

    let mut log = vec![0; length as usize];
    let mut len = 0;
    unsafe {
        gl_bindings::GetShaderInfoLog(
            shader,
            length,
            &mut len,
            log.as_mut_ptr() as *mut std::ffi::c_char,
        )
    }

    log.shrink_to(len.try_into().unwrap());
    let c_str = unsafe { std::ffi::CStr::from_ptr(log.as_ptr()) };
    c_str.to_string_lossy().to_string()
}


#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) fn get_proc_address(addr: &str) -> *const std::os::raw::c_void {
    use core_foundation::bundle::{CFBundleGetBundleWithIdentifier, CFBundleGetFunctionPointerForName};
    use core_foundation::string::CFString;
    use core_foundation::base::TCFType;
    let symbol_name = CFString::new(addr);
    let framework_name = CFString::new("com.apple.opengles");
    unsafe {
        let framework = CFBundleGetBundleWithIdentifier(framework_name.as_concrete_TypeRef());
        CFBundleGetFunctionPointerForName(framework, symbol_name.as_concrete_TypeRef()).cast()
    }
}