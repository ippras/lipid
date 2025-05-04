use crate::prelude::*;
use polars::prelude::{enum_::EnumChunkedBuilder, *};
use std::{ops::Deref, sync::LazyLock};

/// The bound data type.
pub const BOUND_DATA_TYPE: LazyLock<DataType> =
    LazyLock::new(|| DataType::Enum(Some(MAP.clone()), Default::default()));

/// The bound chunked array.
#[derive(Clone, Default)]
#[repr(transparent)]
pub struct BoundChunked(CategoricalChunked);

impl BoundChunked {
    pub fn new(categorical: CategoricalChunked) -> Self {
        Self::try_new(categorical).unwrap()
    }

    /// Creates a new bound chunked array from a [`Series`].
    ///
    /// # Errors
    ///
    /// Returns an error if the series datatype is not the [`BOUND_DATA_TYPE`].
    pub fn try_new(categorical: CategoricalChunked) -> PolarsResult<Self> {
        polars_ensure!(
            *categorical.dtype() == *BOUND_DATA_TYPE,
            SchemaMismatch: "invalid bound series datatype: expected `{}`, got = `{}`",
            *BOUND_DATA_TYPE,
            categorical.dtype(),
        );
        Ok(Self(categorical))
    }

    pub fn iter(&self) -> impl Iterator<Item = Option<Bound>> {
        self.0
            .iter_str()
            .map(|identifier| Some(Bound::new(identifier?)))
    }

    pub fn try_iter(&self) -> impl Iterator<Item = PolarsResult<Option<Bound>>> {
        self.0.iter_str().map(|identifier| {
            let Some(identifier) = identifier else {
                return Ok(None);
            };
            match Bound::try_from(identifier) {
                Err(identifier) => Err(polars_err!(
                    not_in_enum,
                    value = identifier,
                    categories = self.0.get_rev_map().get_categories()
                )),
                Ok(bound) => Ok(Some(bound)),
            }
        })
    }
}

impl BoundChunked {
    /// Returns the number of sized bounds in the bound chunked array.
    pub fn sized(&self) -> u8 {
        (self.0.physical().len() - self.0.null_count()) as _
    }

    // /// Returns the number of unsized bounds in the bound chunked array.
    // pub fn r#unsized(&self) -> u8 {
    //     self.0.null_count() as _
    // }

    /// Returns the total number of unsaturations in the bound chunked array.
    pub fn unsaturation(&self) -> u8 {
        self.iter()
            .filter_map(|bound| bound?.as_unsaturated()?.unsaturation())
            .sum()
    }
}

unsafe impl IntoSeries for BoundChunked {
    fn into_series(self) -> Series {
        self.0.into_series()
    }
}

impl Deref for BoundChunked {
    type Target = CategoricalChunked;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoIterator for &BoundChunked {
    type Item = Option<Bound>;

    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> TryFrom<&'a CategoricalChunked> for &'a BoundChunked {
    type Error = PolarsError;

    fn try_from(value: &'a CategoricalChunked) -> Result<Self, Self::Error> {
        polars_ensure!(
            *value.dtype() == *BOUND_DATA_TYPE,
            SchemaMismatch: "invalid bound series datatype: expected `{}`, got = `{}`",
            *BOUND_DATA_TYPE,
            value.dtype(),
        );
        // [safe](https://doc.rust-lang.org/reference/type-layout.html?highlight=transparent#the-transparent-representation)
        Ok(unsafe { &*(value as *const CategoricalChunked as *const Self) })
    }
}

impl<'a> TryFrom<&'a Series> for &'a BoundChunked {
    type Error = PolarsError;

    fn try_from(value: &'a Series) -> Result<Self, Self::Error> {
        value.categorical()?.try_into()
    }
}

impl<const N: usize> TryFrom<&[Option<&str>; N]> for BoundChunked {
    type Error = PolarsError;

    fn try_from(value: &[Option<&str>; N]) -> Result<Self, Self::Error> {
        let mut identifiers = EnumChunkedBuilder::new(
            PlSmallStr::from_static(IDENTIFIER),
            value.len(),
            MAP.clone(),
            Default::default(),
            true,
        );
        for identifier in value {
            match identifier {
                Some(identifier) => identifiers.append_str(identifier)?,
                None => identifiers.append_null(),
            };
        }
        Ok(Self::new(identifiers.finish()))
    }
}

impl<const N: usize> TryFrom<&[&str; N]> for BoundChunked {
    type Error = PolarsError;

    fn try_from(value: &[&str; N]) -> Result<Self, Self::Error> {
        let mut identifiers = EnumChunkedBuilder::new(
            PlSmallStr::from_static(IDENTIFIER),
            value.len(),
            MAP.clone(),
            Default::default(),
            true,
        );
        for identifier in value {
            identifiers.append_str(identifier)?;
        }
        Ok(Self::new(identifiers.finish()))
    }
}

impl BoundChunked {
    pub fn rco(&self) -> Rco<&Self> {
        Rco(self)
    }

    pub fn rcoo(&self) -> Rcoo<&Self> {
        Rcoo(self)
    }

    pub fn rcooh(&self) -> Rcooh<&Self> {
        Rcooh(self)
    }

    pub fn rcooch3(&self) -> Rcooch3<&Self> {
        Rcooch3(self)
    }
}

#[cfg(feature = "atomic")]
impl Atomic for &BoundChunked {
    type Output = u8;

    #[inline]
    fn carbons(self) -> u8 {
        self.sized() + 1
    }

    #[inline]
    fn hydrogens(self) -> u8 {
        2 * self.carbons() - 2 * self.unsaturation()
    }
}

#[cfg(feature = "ecn")]
impl EquivalentCarbonNumber for &BoundChunked {
    type Output = u8;

    #[inline]
    fn equivalent_carbon_number(self) -> u8 {
        self.carbons() - 2 * self.unsaturation()
    }
}

#[cfg(feature = "mass")]
mod mass;
