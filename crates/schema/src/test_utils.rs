//! Test utils.

#![cfg(test)]

use std::path::{Path, PathBuf};

/// Read a test asset.
pub fn read_test_asset<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let dir = std::env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let path = {
        let mut val = PathBuf::from(dir);
        val.push("testdata");
        val.push(path);
        val
    };
    std::fs::read(path).unwrap()
}
