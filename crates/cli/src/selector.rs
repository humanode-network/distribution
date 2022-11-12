//! Package selector.

use humanode_distribution_schema::manifest::Package;

/// Package selector that's optimizied for the CLI experience.
pub struct Selector {
    /// The package display name, optional.
    pub package_display_name: Option<String>,
}

/// An error that can occur during the selection.
#[derive(Debug, thiserror::Error)]
pub enum SelectionError {
    /// When we don't the packages to filter from.
    #[error("no packages are available")]
    NoPackages,
    /// When a more specific selector is required to narrow the choice.
    #[error("more than one package is available, use a selector to specify which one you want")]
    NotSpecificEnough,
    /// When the selector filtered out all the possible packages without landing on one.
    #[error("unable to find the requested package")]
    NotFound,
}

impl Selector {
    /// Select a package from the list.
    pub fn select<T: AsRef<Package>>(&self, mut packages: Vec<T>) -> Result<T, SelectionError> {
        let last = packages.pop().ok_or(SelectionError::NoPackages)?;

        let package_display_name = match self.package_display_name {
            Some(ref package_display_name) => package_display_name,
            None if !packages.is_empty() => return Err(SelectionError::NotSpecificEnough),
            None => return Ok(last),
        };

        let selected = packages
            .into_iter()
            .chain(std::iter::once(last))
            .find(|package| &package.as_ref().display_name == package_display_name)
            .ok_or(SelectionError::NotFound)?;

        Ok(selected)
    }
}
