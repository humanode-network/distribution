//! The detection machinery.

#[cfg(not(windows))]
#[path = "implementation/default.rs"]
mod implementation;

#[cfg(windows)]
#[path = "implementation/windows.rs"]
mod implementation;

/// Info about the system.
#[derive(Debug)]
pub struct Info {
    /// The name of the system architecture.
    pub arch: String,
    /// The name of the platform.
    pub platform: String,
}

/// Obtain the system information.
pub fn detect() -> std::io::Result<Info> {
    self::implementation::detect()
}
