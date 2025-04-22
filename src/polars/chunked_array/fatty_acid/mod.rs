use crate::prelude::*;
use polars::prelude::*;
use std::ops::Deref;

/// Fatty acid chunked array
///
/// This struct represents a chunked array of fatty acids, which is a wrapper
/// around a [`ListChunked`]. It provides methods to interact with and
/// manipulate the fatty acid data.
#[derive(Clone, Debug, Default)]
#[repr(transparent)]
pub struct FattyAcidChunked(ListChunked);

impl FattyAcidChunked {
    /// Creates a new [`FattyAcidChunked`] from a [`Series`].
    ///
    /// Ensures that the inner data type of the series is [`BOUND_DATA_TYPE`].
    ///
    /// # Arguments
    ///
    /// * `series` - A reference to a [`Series`] containing the fatty acid data.
    ///
    /// # Returns
    ///
    /// A [`PolarsResult`] containing a reference to the new
    /// [`FattyAcidChunked`].
    ///
    /// # Errors
    ///
    /// Returns a [`PolarsError`] if the inner data type of the series is not
    /// [`BOUND_DATA_TYPE`].
    pub fn new(series: &Series) -> PolarsResult<&Self> {
        let fatty_acids = series.list()?;
        polars_ensure!(
            *fatty_acids.inner_dtype() == *BOUND_DATA_TYPE,
            SchemaMismatch: "invalid fatty_acid series inner datatype: expected `Bound`, got = `{}`",
            fatty_acids.inner_dtype(),
        );
        // [safe](https://doc.rust-lang.org/reference/type-layout.html?highlight=transparent#the-transparent-representation)
        Ok(unsafe { &*(fatty_acids as *const ListChunked as *const Self) })
    }

    /// Length of the chunked array.
    ///
    /// # Returns
    ///
    /// The length of the chunked array.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Retrieves the bound series at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the fatty acid in the chunked array to
    ///   retrieve.
    ///
    /// # Returns
    ///
    /// An [`Option`] containing the [`Series`] at the specified index, or
    /// [`None`] if the series does not contain the specified index
    pub fn get(&self, index: usize) -> Option<Series> {
        Some(self.0.get_as_series(index)?.cast(&BOUND_DATA_TYPE).unwrap())
    }
}

impl Deref for FattyAcidChunked {
    type Target = ListChunked;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromIterator<Option<Series>> for FattyAcidChunked {
    fn from_iter<T: IntoIterator<Item = Option<Series>>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for &FattyAcidChunked {
    type Item = Option<Series>;

    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> TryFrom<&'a Series> for &'a FattyAcidChunked {
    type Error = PolarsError;

    fn try_from(value: &'a Series) -> Result<Self, Self::Error> {
        FattyAcidChunked::new(value)
    }
}

unsafe impl IntoSeries for FattyAcidChunked {
    fn into_series(self) -> Series {
        self.0.into_series()
    }
}

#[cfg(feature = "atomic")]
mod atomic;
mod chain_length;
#[cfg(feature = "map")]
mod map;
#[cfg(feature = "mask")]
mod mask;
#[cfg(feature = "select")]
mod select;
