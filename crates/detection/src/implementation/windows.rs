//! Windows specific implementation.

use crate::Info;

/// Obtain the system information.
pub fn detect() -> std::io::Result<Info> {
    #[cfg(target_arch = "x86_64")]
    let arch = "x86_64";
    #[cfg(target_arch = "aarch64")]
    let arch = "aarch64";
    Ok(Info {
        platform: "Windows".into(),
        arch: arch.into(),
    })
}
