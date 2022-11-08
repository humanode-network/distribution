//! The manifest.

use serde::{Deserialize, Serialize};

/// A single manifest.
#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    /// Binaries provided by this manifest.
    pub binaries: Vec<Binary>,
}

/// A single binary package.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Binary {
    /// The platform this binary is intended for.
    pub platform: Platform,
    /// The architecture this binary is intended for.
    pub arch: Arch,

    /// The name to use when displaying the package.
    pub display_name: String,
    /// The description of the package.
    pub description: String,

    /// The path to the icon to use.
    /// Relative to the location of the manifest file.
    pub icon: String,

    /// The path to the executable.
    pub executable_path: LocalPath,
    /// The path to the chain spec.
    pub chainspec_path: LocalPath,
    /// The path to the ngrok.
    pub ngrok_path: LocalPath,
    /// The path to the Humanode Websocket Tunnel Client.
    pub humanode_websocket_tunnel_client_path: LocalPath,

    /// Files included in this package.
    pub files: Vec<File>,
}

/// The platform code.
///
/// Values returned by `uname -s`.
///
/// Sample values are:
/// - Drawin
/// - Linux
/// - Windows
#[derive(Debug, Serialize, Deserialize)]
pub struct Platform(pub String);

/// The architecture code.
///
/// Values returned by `uname -m`.
///
/// Sample values are:
/// - x86_64
/// - arm64
#[derive(Debug, Serialize, Deserialize)]
pub struct Arch(pub String);

/// Relative path in the context of the file system of the distribution.
///
/// Must be evaludated relative to the directory of the distribution root, or
/// in the context where the ditribution root is the process' current directory.
#[derive(Debug, Serialize, Deserialize)]
pub struct LocalPath(pub String);

/// The URL.
///
/// Have to evaluted against the Manifest URL using the Base URL algorithm,
/// see <https://developer.mozilla.org/en-US/docs/Web/API/URL/URL>.
#[derive(Debug, Serialize, Deserialize)]
pub struct Url(pub String);

/// The hexadecimal representation of a SHA-256 sum.
#[derive(Debug, Serialize, Deserialize)]
pub struct Sha256(pub String);

/// A single file description.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    /// The sub URL where to look for the asset.
    pub sub_url: Url,
    /// The destination sub path to place the file at.
    pub destination_sub_path: LocalPath,
    /// The SHA-256 sum of the file.
    pub sha256: Sha256,
}

#[cfg(test)]
mod tests {
    use crate::test_utils::read_test_asset;

    use super::*;

    #[test]
    fn e2e() {
        let raw = read_test_asset("manifest.json");
        let raw_value: serde_json::Value = serde_json::from_slice(&raw).unwrap();
        let manifest: Manifest = serde_json::from_slice(&raw).unwrap();
        let manifest_value = serde_json::to_value(manifest).unwrap();
        assert_eq!(raw_value, manifest_value);
    }
}
