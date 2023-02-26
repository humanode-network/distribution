//! The `manifestUrls.d` files content.

use serde::{Deserialize, Serialize};

/// The structure of a file in the `manifestUrls.d`.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Format {
    /// A list of URLs each serving a manifest.
    pub manifest_urls: Vec<ManifestUrl>,
}

/// A single manifest reference.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManifestUrl {
    /// A URL at which the manifest is served.
    pub url: String,
}
