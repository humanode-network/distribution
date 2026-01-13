//! The configuration files and utilities.

use serde::{Deserialize, Serialize};

pub mod load;
pub mod paths;
pub mod schemas;

pub mod dirs {
    //! Directories.

    /// The directory that contains files,
    /// each file with a list of URLs, each URL pointing to a repo.
    /// Repo URLs can be directly handled by the resolver.
    pub const REPOS: &str = "repos.d";

    /// The directory that contains files,
    /// each file with a list of URLs, each URL pointing to a manifest.
    /// Manifest URLs can be directly handled by the resolver.
    pub const MANIFEST_URLS: &str = "manifestUrls.d";
}

/// The configured sources.
///
/// This is not intended to be directly persisted, but rather [`load`]ed from
/// the various config files.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Sources {
    /// A list of Manifest URLs.
    pub manifest_urls: Vec<String>,
    /// A list of Repo URLs.
    pub repo_urls: Vec<String>,
}

impl Sources {
    /// Extend the sources from the other instance.
    pub fn extend(&mut self, other: Self) {
        let Self {
            manifest_urls,
            repo_urls,
        } = other;
        self.manifest_urls.extend(manifest_urls);
        self.repo_urls.extend(repo_urls);
    }
}
