use crate::prelude::*;
use polars::prelude::*;

/// Fatty acid chunked array
///
/// This struct represents a chunked array of fatty acids, which is a wrapper
/// around a [`ListChunked`]. It provides methods to interact with and
/// manipulate the fatty acid data.
#[derive(Clone, Debug)]
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

    /// Applies a function to each fatty acid in the chunked array and returns
    /// an iterator over the results.
    ///
    /// # Arguments
    ///
    /// * `f` - A function to apply to each fatty acid.
    ///
    /// # Returns
    ///
    /// An iterator over the results of applying the function to each fatty
    /// acid.
    #[inline]
    pub fn map<T>(
        &self,
        f: impl Fn(&BoundChunked) -> T,
    ) -> impl Iterator<Item = PolarsResult<Option<T>>> {
        self.0.into_iter().map(move |bounds| {
            let Some(bounds) = bounds else {
                return Ok(None);
            };
            Ok(Some(f(bounds.bound()?)))
        })
    }

    /// Returns the number of bounds for each fatty acid in the chunked array.
    ///
    /// # Returns
    ///
    /// A [`PolarsResult`] containing a [`UInt8Chunked`] with the number of
    /// bounds for each fatty acid.
    pub fn bounds(&self) -> PolarsResult<UInt8Chunked> {
        self.map(|bounds| bounds.len()).collect()
    }

    /// Returns the unsaturation levels of each fatty acid in the chunked array.
    ///
    /// # Returns
    ///
    /// A [`PolarsResult`] containing a [`UInt8Chunked`] with the unsaturation
    /// levels.
    pub fn unsaturation(&self) -> PolarsResult<UInt8Chunked> {
        self.map(|bounds| bounds.unsaturation()).collect()
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

mod atomic;
mod chain_length;
mod mask;
mod select;
