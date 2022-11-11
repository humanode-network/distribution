//! Resolver.

use std::collections::HashSet;

use futures::{pin_mut, Sink, SinkExt};
use humanode_distribution_schema::{
    manifest::{Binary, Manifest},
    repo::Repo,
};
use serde::{Deserialize, Serialize};

use crate::http::load_meta;

/// An issue that occured during resolution.
#[derive(Debug)]
pub struct ResolutionError {
    /// The URL that was attempted.
    pub url: String,
    /// The error description.
    pub error: String,
}

impl ResolutionError {
    /// Construct a new resolution issue.
    pub fn from_display(url: String, err: impl std::fmt::Display) -> Self {
        Self {
            url,
            error: err.to_string(),
        }
    }
}

/// The resolver params.
#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
    /// The manifest URLs.
    pub manifest_urls: Vec<String>,
    /// The repo URLs.
    pub repo_urls: Vec<String>,
}

/// The context-enhanced value.
#[derive(Debug, Serialize, Deserialize)]
pub struct Contextualized<T> {
    /// The manifest URL this binary came from.
    pub manifest_url: String,
    /// The value that is contextualized.
    pub value: T,
}

/// Resolve the binaries.
pub async fn resolve(
    client: reqwest::Client,
    params: Params,
    issues: impl Sink<ResolutionError>,
    filter: impl Fn(&Contextualized<Binary>) -> bool,
) -> Vec<Contextualized<Binary>> {
    let Params {
        manifest_urls,
        repo_urls,
    } = params;

    pin_mut!(issues);

    let mut manifest_urls: HashSet<String> = HashSet::from_iter(manifest_urls);

    for url in repo_urls {
        let repo: Repo = match load_meta(&client, &url).await {
            Ok(val) => val,
            Err(err) => {
                let _ = issues
                    .send(ResolutionError {
                        url,
                        error: err.to_string(),
                    })
                    .await;
                continue;
            }
        };
        manifest_urls.extend(repo.manifest_urls.into_iter().map(|item| item.url));
    }

    let mut binaries = Vec::new();

    for url in manifest_urls {
        let manifest: Manifest = match load_meta(&client, &url).await {
            Ok(val) => val,
            Err(err) => {
                let _ = issues
                    .send(ResolutionError {
                        url,
                        error: err.to_string(),
                    })
                    .await;
                continue;
            }
        };

        binaries.extend(
            manifest
                .binaries
                .into_iter()
                .map(|binary| Contextualized {
                    manifest_url: url.clone(),
                    value: binary,
                })
                .filter(&filter),
        );
    }

    binaries
}
