//! The schemas of various config files.
//!
//! These should be no need to use these schemas outside of this crate, since
//! the loading process resolves the files into
//! the effective [`crate::Sources`].

pub mod manifest_urls;
pub mod repos;
