use crate::{display::FattyAcid, prelude::*};
use polars::prelude::*;

/// Bound chunked array
#[derive(Clone)]
#[repr(transparent)]
pub struct BoundChunked(CategoricalChunked);

impl BoundChunked {
    /// Creates a new bound chunked array from a [`Series`].
    ///
    /// # Errors
    ///
    /// Returns an error if the series datatype is not the [`BOUND_DATA_TYPE`].
    pub fn new(series: &Series) -> PolarsResult<&Self> {
        let bounds = series.categorical()?;
        polars_ensure!(
            *bounds.dtype() == *BOUND_DATA_TYPE,
            SchemaMismatch: "invalid bound series datatype: expected `Bound`, got = `{}`",
            bounds.dtype(),
        );
        // [safe](https://doc.rust-lang.org/reference/type-layout.html?highlight=transparent#the-transparent-representation)
        Ok(unsafe { &*(bounds as *const CategoricalChunked as *const Self) })
    }
}

impl BoundChunked {
    /// Returns the number of bounds in the bound chunked array.
    pub fn len(&self) -> u8 {
        self.0.len() as _
    }

    /// Returns the total number of unsaturations in the bound chunked array.
    pub fn unsaturation(&self) -> u8 {
        self.into_iter()
            .filter_map(|bound| bound?.as_unsaturated()?.unsaturation())
            .sum()
    }
}

impl BoundChunked {
    /// Filters the bound chunked array based on a predicate function.
    pub fn filter(&self, predicate: impl Fn(Option<Bound>) -> bool) -> impl Iterator<Item = u8> {
        self.into_iter()
            .enumerate()
            .filter_map(move |(index, bound)| predicate(bound).then_some(index as _))
    }

    /// Returns an iterator over the indices of double bonds in the bound
    /// chunked array.
    pub fn doubles(&self) -> impl Iterator<Item = u8> {
        self.filter(|bound| bound.is_some_and(Bound::is_double))
    }

    /// Returns an iterator over the indices of triple bonds in the bound
    /// chunked array.
    pub fn triples(&self) -> impl Iterator<Item = u8> {
        self.filter(|bound| bound.is_some_and(Bound::is_triple))
    }

    /// Returns an iterator over the indices of unsaturated bonds in the bound
    /// chunked array.
    pub fn unsaturated(&self) -> impl Iterator<Item = u8> {
        self.filter(|bound| bound.is_some_and(Bound::is_unsaturated))
    }
}

impl BoundChunked {
    /// Displays the bound chunked array as a [`FattyAcid`] based on the
    /// provided options.
    pub fn display(&self, options: Options) -> PolarsResult<FattyAcid> {
        use crate::display::{Common, Delta, Kind, System};

        let carbons = self.carbons();
        let unsaturated = self
            .into_iter()
            .enumerate()
            .filter_map(|(index, bound)| match bound {
                Some(Bound::Saturated) => None,
                Some(Bound::Unsaturated(unsaturated)) => Some((index + 1, Some(unsaturated))),
                None => Some((index + 1, None)),
            })
            .collect();
        Ok(match options.kind {
            Kind::Delta => FattyAcid::Common(Common::Delta(Delta {
                carbons,
                unsaturated,
                options,
            })),
            Kind::System => FattyAcid::System(System {
                carbons,
                unsaturated,
            }),
        })
    }
}

impl IntoIterator for &BoundChunked {
    type Item = Option<Bound>;

    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_str().map(|id| Some(Bound::new(id?)))
    }
}

impl<'a> TryFrom<&'a Series> for &'a BoundChunked {
    type Error = PolarsError;

    fn try_from(value: &'a Series) -> Result<Self, Self::Error> {
        BoundChunked::new(value)
    }
}

#[cfg(feature = "atomic")]
mod atomic;
mod chain_length;
mod kind;
#[cfg(feature = "mask")]
mod mask;
#[cfg(feature = "mass")]
mod mass;
