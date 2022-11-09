//! The detection machinery.

use std::ffi::{c_char, CStr};

/// Info about the system.
#[derive(Debug)]
pub struct Info {
    /// The name of the system architecture.
    pub arch: String,
    /// The name of the platform.
    pub platform: String,
}

/// Obtain the system information.
#[allow(unsafe_code)]
pub fn detect() -> std::io::Result<Info> {
    let mut value = unsafe { std::mem::zeroed() };
    if unsafe { libc::uname(&mut value) } != 0 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(Info {
        platform: to_string(&value.sysname[..]),
        arch: to_string(&value.machine[..]),
    })
}

/// Assume a valid C string and copy it into a new Rust string with a lossy
/// UTF-8 conversion.
#[allow(unsafe_code)]
fn to_string(buf: &[c_char]) -> String {
    let c_str = unsafe { CStr::from_ptr(buf.as_ptr()) };
    c_str.to_string_lossy().into_owned()
}
