//! The paths.

use std::path::PathBuf;

/// The subdirectories to look for the configs at.
const SUBDIRS: &[&str] = &["humanode-launcher"];

/// The various paths to attempt loading the configs from.
pub fn configs() -> impl IntoIterator<Item = PathBuf> {
    let Some(dir) = dirs::config_dir() else {
        return Vec::new();
    };

    SUBDIRS
        .iter()
        .map(move |&subdirs| dir.join(subdirs))
        .collect::<Vec<_>>()
}
