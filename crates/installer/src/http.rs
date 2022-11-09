//! The HTTP utils.

use digest::Digest;
use futures::pin_mut;
use tokio::io::AsyncWriteExt;

/// An error that can happen when the loading a file.
#[derive(Debug, thiserror::Error)]
pub enum FileLoadError {
    /// A failure from `reqwest`.
    #[error("reqwest error: {0}")]
    Reqwest(#[source] reqwest::Error),
    /// The server returned a bad status code.
    #[error("server error: {0}")]
    Server(reqwest::StatusCode),
    /// The reading from the network failed.
    #[error("read error: {0}")]
    Read(#[source] reqwest::Error),
    /// The writing to the file failed.
    #[error("write error: {0}")]
    Write(#[source] tokio::io::Error),
}

/// Load a meta URL and parse it as a YAML document.
pub async fn load_file<T: Digest>(
    client: &reqwest::Client,
    url: &str,
    dest: impl tokio::io::AsyncWrite,
    mut digest: T,
) -> Result<digest::Output<T>, FileLoadError> {
    let req = client.get(url).build().map_err(FileLoadError::Reqwest)?;

    let mut res = client.execute(req).await.map_err(FileLoadError::Reqwest)?;

    let status = res.status();
    if !status.is_success() {
        return Err(FileLoadError::Server(status));
    }

    pin_mut!(dest);

    while let Some(mut buf) = res.chunk().await.map_err(FileLoadError::Read)? {
        use bytes::Buf;
        loop {
            let chunk = buf.chunk();
            if chunk.is_empty() {
                break;
            }

            dest.write_all(chunk).await.map_err(FileLoadError::Write)?;
            tokio::task::block_in_place(|| {
                digest.update(chunk);
            });

            buf.advance(chunk.len());
        }
    }

    let hash = digest.finalize();
    Ok(hash)
}
