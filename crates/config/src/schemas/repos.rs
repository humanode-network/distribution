//! The `repos.d` files content.

use serde::{Deserialize, Serialize};

/// The structure of a file in the `repos.d`.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Format {
    /// A list of URLs each serving a repo.
    pub repo_urls: Vec<RepoUrl>,
}

/// A single repo reference.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoUrl {
    /// The URL of this repo.
    pub url: String,
}
