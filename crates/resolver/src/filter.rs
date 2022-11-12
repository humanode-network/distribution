//! The filtering logic.

use humanode_distribution_schema::manifest::Package;

/// Filter params.
pub struct Params {
    /// Platform.
    pub platform: String,
    /// Architecture.
    pub arch: String,
}

impl Params {
    /// Check if the package matches the filter.
    pub fn matches(&self, item: impl AsRef<Package>) -> bool {
        let package = item.as_ref();
        package.platform.0 == self.platform && package.arch.0 == self.arch
    }

    /// Filter the input packages with the filtering params.
    pub fn filter<'a, T: AsRef<Package>>(
        &'a self,
        items: impl IntoIterator<Item = T> + 'a,
    ) -> impl Iterator<Item = T> + 'a {
        items.into_iter().filter(|item| self.matches(item))
    }
}
