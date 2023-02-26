//! The paths.

use std::path::PathBuf;

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
