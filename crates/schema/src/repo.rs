//! The repo.

use serde::{Deserialize, Serialize};

/// A single repo.
#[derive(Debug, Serialize, Deserialize)]
pub struct Repo {
    /// A list of URLs each serving a manifest.
    pub manifest_urls: Vec<ManifestUrl>,
}

/// A single manifest reference.
#[derive(Debug, Serialize, Deserialize)]
pub struct ManifestUrl {
    /// A URL at which the manifest is served.
    pub url: String,
}
