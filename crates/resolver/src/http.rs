//! The HTTP utils.

/// An error that can happen when the loading stuff.
#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    /// A failure from `reqwest`.
    #[error("reqwest error: {0}")]
    Reqwest(#[source] reqwest::Error),
    /// The server returned a bad status code.
    #[error("server error: {0}")]
    Server(reqwest::StatusCode),
    /// Deserialization error.
    #[error("serde error: {0}")]
    Serde(#[source] serde_yaml_bw::Error),
}

/// Load a meta URL and parse it as a YAML document.
pub async fn load_meta<T>(client: &reqwest::Client, url: &str) -> Result<T, LoadError>
where
    T: for<'de> serde::Deserialize<'de>,
{
    let req = client
        .get(url)
        .header(
            reqwest::header::ACCEPT,
            "application/json,application/x-yaml,text/yaml",
        )
        .build()
        .map_err(LoadError::Reqwest)?;

    let res = client.execute(req).await.map_err(LoadError::Reqwest)?;

    let status = res.status();
    if !status.is_success() {
        return Err(LoadError::Server(status));
    }

    let bytes = res.bytes().await.map_err(LoadError::Reqwest)?;

    let repo: T = serde_yaml_bw::from_slice(&bytes).map_err(LoadError::Serde)?;

    Ok(repo)
}
