use std::ffi::{CStr, CString};

use libc::c_char;

pub fn get_sdk_version() -> i32 {
    let mut ret = -1;
    let cmd = CString::new("getprop ro.build.version.sdk").unwrap();
    let mode = CString::new("r").unwrap();
    let file = unsafe { libc::popen(cmd.as_ptr(), mode.as_ptr()) };
    if !file.is_null() {
        let mut output = [0 as c_char; 100];

        if !unsafe { libc::fgets(output.as_mut_ptr(), 100, file).is_null() } {
            let string = unsafe { CStr::from_ptr(output.as_ptr()) };
            if let Ok(value) = string.to_string_lossy().parse::<i32>() {
                ret = value;
            }
        }

        unsafe {
            libc::pclose(file);
        }
    }
    ret
}
