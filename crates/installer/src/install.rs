//! Installation logic.

use std::path::PathBuf;

use digest::Digest;
use humanode_distribution_schema::manifest::Package;
use url::Url;

use crate::http::{self, FileLoadError};

/// An error that can happen during the installation.
#[derive(Debug, thiserror::Error)]
pub enum InstallationError {
    /// Base URL is invalid.
    #[error("unable to parse hash {hash:?} for a given file {url}: {error}")]
    ParseHash {
        /// The hash that we attempted to parse.
        hash: String,
        /// The path of the file we attempted to parse the hash for.
        path: String,
        /// The URL of the file we attempted to parse the hash for.
        url: String,
        /// The unferlying error.
        #[source]
        error: hex::FromHexError,
    },
    /// Base URL is invalid.
    #[error("invalid base URL {url}: {error}")]
    ParseBaseUrl {
        /// The URL we attempted to parse.
        url: String,
        /// The unferlying error.
        #[source]
        error: url::ParseError,
    },
    /// Failed to parse the file URL relative to the base URL.
    #[error("invalid file URL {url}: {error}")]
    ParseFileUrl {
        /// The URL we atempted to parse.
        url: String,
        /// The base URL we used.
        base_url: Url,
        /// The underlying error.
        #[source]
        error: <Url as std::str::FromStr>::Err,
    },
    /// Failed to create a dir where to put the files.
    #[error("unable to create dir {path}: {error}")]
    CreateDir {
        /// The directory path.
        path: PathBuf,
        /// The unferlying error.
        #[source]
        error: std::io::Error,
    },
    /// Failed to create a file.
    #[error("unable to create file {path}: {error}")]
    CreateFile {
        /// File path.
        path: PathBuf,
        /// The underlying error.
        #[source]
        error: std::io::Error,
    },
    /// Downloading file failed.
    #[error("unable to load the file {path} from {url}: {error}")]
    LoadFile {
        /// The local path to the file we were downloading.
        path: PathBuf,
        /// The URL of the file we were downloading.
        url: String,
        /// The underlying error.
        #[source]
        error: FileLoadError,
    },
    /// The file hash did not match the expectation.
    #[error("loaded file {path} hash mismatch: expected {expected:?} but got {actual:?}")]
    FileHashMismatch {
        /// The file path.
        path: PathBuf,
        /// The expected hash.
        expected: Vec<u8>,
        /// The actual hash.
        actual: Vec<u8>,
    },
    /// Failed to set the file permissions.
    #[error("unable to set permissions for {path}: {error}")]
    SetFilePermissions {
        /// The file path.
        path: PathBuf,
        /// The underlying error.
        #[source]
        error: std::io::Error,
    },
}

/// The installation routine parameters.
pub struct Params {
    /// HTTP client.
    pub client: reqwest::Client,
    /// The path to the target directory where to install the package.
    pub dir: String,
    /// The base URL to use for resolving the URLs.
    pub base_url: String,
    /// The package to install.
    pub package: Package,
}

/// Prepare the directories, then download the files and set proper file
/// permissions.
pub async fn install(params: Params) -> Result<(), InstallationError> {
    let Params {
        client,
        dir,
        base_url,
        package,
    } = params;

    let base_path = PathBuf::from(dir);
    let base_url = Url::parse(&base_url).map_err(|error| InstallationError::ParseBaseUrl {
        url: base_url,
        error,
    })?;

    // Download the files.
    for file in package.files {
        let path = base_path.join(&file.destination_sub_path.0);

        let expected_hash =
            hex::decode(&file.sha256.0).map_err(|error| InstallationError::ParseHash {
                hash: file.sha256.0.clone(),
                path: file.destination_sub_path.0.clone(),
                url: file.sub_url.0.clone(),
                error,
            })?;

        let url = Url::options()
            .base_url(Some(&base_url))
            .parse(&file.sub_url.0)
            .map_err(|error| InstallationError::ParseFileUrl {
                url: file.sub_url.0.clone(),
                base_url: base_url.clone(),
                error,
            })?;

        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(&parent).await.map_err(|error| {
                InstallationError::CreateDir {
                    path: parent.to_path_buf(),
                    error,
                }
            })?;
        }

        let fileio = tokio::fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&path)
            .await
            .map_err(|error| InstallationError::CreateFile {
                path: path.clone(),
                error,
            })?;
        let digest = sha2::Sha256::new();
        let hash = http::load_file(&client, url.as_str(), fileio, digest)
            .await
            .map_err(|error| InstallationError::LoadFile {
                path: path.clone(),
                url: url.to_string(),
                error,
            })?;

        if hash.as_slice() != expected_hash.as_slice() {
            return Err(InstallationError::FileHashMismatch {
                path: path.clone(),
                expected: expected_hash,
                actual: hash.to_vec(),
            });
        }
    }

    // Set executable permissions, only on unix systems.
    #[cfg(unix)]
    {
        let executables = [
            package.executable_path,
            package.ngrok_path,
            package.humanode_websocket_tunnel_client_path,
        ];
        for executable in executables {
            let path = base_path.join(executable.0);

            use std::{fs::Permissions, os::unix::prelude::PermissionsExt};
            tokio::fs::set_permissions(&path, Permissions::from_mode(0o755))
                .await
                .map_err(|error| InstallationError::SetFilePermissions { path, error })?;
        }
    }

    Ok(())
}
