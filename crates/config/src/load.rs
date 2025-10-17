//! The configs loading logic.

use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::Sources;

/// The result of loading the sources.
#[derive(Debug)]
pub struct SourcesLoadingResult {
    /// The loaded sources.
    pub sources: Sources,
    /// The errors that have occurred while loading the sources.
    pub errors: SourcesLoadingErrors,
}

/// The errors that can occur while loading the sources.
#[derive(Debug)]
pub struct SourcesLoadingErrors {
    /// The Manifest URLs loading errors.
    pub manifest_urls: Vec<LoadingError>,
    /// The Repo URLs loading errors.
    pub repo_urls: Vec<LoadingError>,
}

impl SourcesLoadingErrors {
    /// Returns `true` if no errors have occurred.
    pub fn is_empty(&self) -> bool {
        self.manifest_urls.is_empty() && self.repo_urls.is_empty()
    }

    /// Iterate over all of the errors.
    pub fn all(self) -> impl Iterator<Item = LoadingError> {
        self.manifest_urls.into_iter().chain(self.repo_urls)
    }
}

impl std::fmt::Display for SourcesLoadingErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            f.write_str("no errors occurred while loading the source configs")
        } else {
            let errors_num = self.manifest_urls.len() + self.repo_urls.len();
            write!(
                f,
                "{errors_num} errors have occurred while loading the source configs"
            )
        }
    }
}

/// A loading error.
#[derive(Debug, thiserror::Error)]
pub enum LoadingError {
    /// Error while reading the directory.
    #[error("reading directory {1}: {0}")]
    DirReading(std::io::Error, PathBuf),
    /// Error while reading the next dir entry.
    #[error("reading directory entry {1}: {0}")]
    DirNextEntry(std::io::Error, PathBuf),
    /// Error while reading the file.
    #[error("reading file {1}: {0}")]
    FileReading(std::io::Error, PathBuf),
    /// Error while parsing the file.
    #[error("parsing file {1}: {0}")]
    Parsing(serde_yaml_bw::Error, PathBuf),
}

/// Load the sources from the given path.
///
/// The path is the directory that contains `repos.d` and `manifestUrls.d`.
pub async fn sources(path: impl AsRef<Path>) -> SourcesLoadingResult {
    let path = path.as_ref();
    let (repo_urls_result, manifest_urls_result) = tokio::join!(
        repo_urls(path.join(crate::dirs::REPOS)),
        manifest_urls(path.join(crate::dirs::MANIFEST_URLS)),
    );

    let (repo_urls, repo_urls_errors) = repo_urls_result;
    let (manifest_urls, manifest_urls_errors) = manifest_urls_result;

    let sources = Sources {
        manifest_urls,
        repo_urls,
    };

    let errors = SourcesLoadingErrors {
        manifest_urls: manifest_urls_errors,
        repo_urls: repo_urls_errors,
    };

    SourcesLoadingResult { sources, errors }
}

/// Load the Repo URLs.
async fn repo_urls(path: impl AsRef<Path>) -> (Vec<String>, Vec<LoadingError>) {
    load(path, |file: crate::schemas::repos::Format| {
        file.repo_urls.into_iter().map(|item| item.url)
    })
    .await
}

/// Load the Manifest URLs.
async fn manifest_urls(path: impl AsRef<Path>) -> (Vec<String>, Vec<LoadingError>) {
    load(path, |file: crate::schemas::manifest_urls::Format| {
        file.manifest_urls.into_iter().map(|item| item.url)
    })
    .await
}

/// Load the data from the given path, extracting the values with the provided
/// processor.
async fn load<T, I>(
    path: impl AsRef<Path>,
    process_parsed: impl Fn(T) -> I,
) -> (Vec<String>, Vec<LoadingError>)
where
    T: for<'de> Deserialize<'de>,
    I: IntoIterator<Item = String>,
{
    let path = path.as_ref();
    let mut read_dir = match tokio::fs::read_dir(path).await {
        Ok(val) => val,
        Err(err) => {
            return (
                Vec::new(),
                vec![LoadingError::DirReading(err, path.to_path_buf())],
            )
        }
    };

    let mut values = Vec::new();
    let mut errors = Vec::new();

    loop {
        let entry = match read_dir.next_entry().await {
            Ok(Some(val)) => val,
            Ok(None) => break,
            Err(err) => {
                errors.push(LoadingError::DirNextEntry(err, path.to_path_buf()));
                break;
            }
        };

        let path = entry.path();
        let data = match tokio::fs::read(&path).await {
            Ok(data) => data,
            Err(err) => {
                errors.push(LoadingError::FileReading(err, path));
                continue;
            }
        };

        let parsed = match serde_yaml_bw::from_slice(&data) {
            Ok(v) => v,
            Err(err) => {
                errors.push(LoadingError::Parsing(err, path));
                continue;
            }
        };

        let parsed_values = process_parsed(parsed);

        values.extend(parsed_values);
    }

    (values, errors)
}
