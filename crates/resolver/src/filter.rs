//! The filtering logic.

use humanode_distribution_schema::manifest::Binary;

use crate::resolve::Contextualized;

/// Filter params.
pub struct Params {
    /// Platform.
    pub platform: String,
    /// Architecture.
    pub arch: String,
}

impl Params {
    /// Check if the binariy matches the filter.
    pub fn matches(&self, item: &Contextualized<Binary>) -> bool {
        item.value.platform.0 == self.platform && item.value.arch.0 == self.arch
    }

    /// Filter the input binaries with the filtering params.
    pub fn filter<'a>(
        &'a self,
        binaries: impl IntoIterator<Item = Contextualized<Binary>> + 'a,
    ) -> impl Iterator<Item = Contextualized<Binary>> + 'a {
        binaries
            .into_iter()
            .filter(|binaries| self.matches(binaries))
    }
}
