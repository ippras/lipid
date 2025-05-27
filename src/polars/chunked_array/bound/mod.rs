use super::Identifier;
use crate::prelude::{
    physical::{S, TRANS},
    *,
};
use polars::prelude::{enum_::EnumChunkedBuilder, *};
use std::{
    fmt::{self, Debug, Display, Formatter},
    mem::take,
    sync::LazyLock,
};

/// The bound data type.
pub const BOUND_DATA_TYPE: LazyLock<DataType> =
    LazyLock::new(|| DataType::Enum(Some(MAP.clone()), Default::default()));

/// The bound chunked array.
#[derive(Clone, Default)]
#[repr(transparent)]
pub struct BoundChunked(pub(crate) CategoricalChunked);

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
        check_data_type(&categorical)?;
        Ok(Self(categorical))
    }

    #[inline]
    pub fn as_categorical(&self) -> &CategoricalChunked {
        &self.0
    }

    #[inline]
    pub fn into_categorical(self) -> CategoricalChunked {
        self.0
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = Bound> + ExactSizeIterator {
        self.0
            .iter_str()
            .map(|identifier| Bound(identifier.map(Explicit::new)))
    }

    pub fn try_iter(&self) -> impl Iterator<Item = PolarsResult<Option<Explicit>>> {
        self.0.iter_str().map(|identifier| {
            let Some(identifier) = identifier else {
                return Ok(None);
            };
            match Explicit::try_from(identifier) {
                Err(identifier) => Err(polars_err!(
                    not_in_enum,
                    value = identifier,
                    categories = self.0.get_rev_map().get_categories()
                )),
                Ok(bound) => Ok(Some(bound)),
            }
        })
    }

    pub fn filter(&self, filter: &BooleanChunked) -> PolarsResult<Self> {
        let physical = self.0.physical().filter(filter)?;
        // SAFETY: we only filter the physical indexes so we are still in bounds.
        Ok(Self(unsafe {
            CategoricalChunked::from_cats_and_rev_map_unchecked(
                physical,
                self.0.get_rev_map().clone(),
                self.0.is_enum(),
                self.0.get_ordering(),
            )
        }))
    }

    pub fn pop(&mut self) {
        let physical = self.0.physical();
        let categorical = unsafe {
            CategoricalChunked::from_cats_and_dtype_unchecked(
                physical.slice(0, physical.len().saturating_sub(1)),
                BOUND_DATA_TYPE.clone(),
            )
        };
        *self = Self(categorical);
    }

    pub fn push(&mut self) -> PolarsResult<()> {
        let mut physical = take(self).0.into_physical();
        physical.extend(&UInt32Chunked::full(PlSmallStr::EMPTY, physical::S, 1))?;
        // SAFETY: we extend the physical by one `S` so we are still in bounds.
        let categorical = unsafe {
            CategoricalChunked::from_cats_and_dtype_unchecked(physical, BOUND_DATA_TYPE.clone())
        };
        *self = Self(categorical);
        Ok(())
    }

    pub fn with_name(self, name: PlSmallStr) -> Self {
        let physical = self.0.into_physical().with_name(name);
        // SAFETY: we only rename the physical so we are still in bounds.
        unsafe {
            Self(CategoricalChunked::from_cats_and_dtype_unchecked(
                physical,
                BOUND_DATA_TYPE.clone(),
            ))
        }
    }
}

impl BoundChunked {
    /// Returns the number of sized bounds in the fatty acid.
    pub fn bounds(&self) -> u8 {
        (self.0.len() - self.0.null_count()) as _
    }

    /// Returns the number of saturated bounds in the fatty acid.
    #[inline]
    pub fn saturated(&self) -> BooleanChunked {
        self.equal(S)
    }

    /// Returns the number of unsaturated bounds in the fatty acid.
    #[inline]
    pub fn unsaturated(&self) -> BooleanChunked {
        self.not_equal(S)
    }

    /// Returns the total number of unsaturations in the bound chunked array.
    pub fn unsaturation(&self) -> u8 {
        self.iter()
            .filter_map(|bound| bound.0?.as_unsaturated()?.unsaturation())
            .sum()
    }
}

impl BoundChunked {
    /// # Returns
    ///
    /// The output is unknown (`None`) if the array contains any null values and
    /// no `false` values.
    pub fn is_saturated(&self) -> Option<bool> {
        self.saturated().all_kleene()
    }

    /// Checks if the bound chunked array contains any unsaturated bonds.
    ///
    /// # Returns
    ///
    /// The output is unknown (`None`) if the array contains any null values and
    /// no `true` values.
    pub fn is_unsaturated(&self) -> Option<bool> {
        self.unsaturated().any_kleene()
    }

    /// Checks if the bound chunked array contains exactly one unsaturated bond.
    ///
    /// # Returns
    ///
    /// [`true`] if there is exactly one unsaturated bond, [`false`] otherwise.
    pub fn is_monounsaturated(&self) -> Option<bool> {
        let is_unsaturated = self.unsaturated();
        if is_unsaturated.has_nulls() {
            None
        } else {
            Some(is_unsaturated.num_trues() == 1)
        }
    }

    /// Checks if the bound chunked array contains more than one unsaturated bond.
    ///
    /// # Returns
    ///
    /// [`true`] if there are more than one unsaturated bonds, [`false`] otherwise.
    pub fn is_polyunsaturated(&self) -> Option<bool> {
        let is_unsaturated = self.unsaturated();
        let trues = is_unsaturated.num_trues();
        if trues > 1 {
            Some(true)
        } else if trues + is_unsaturated.null_count() > 1 {
            None
        } else {
            Some(false)
        }
    }

    pub fn is_cis(&self) -> Option<bool> {
        let is_unsaturated = self.unsaturated().any_kleene()?;
        let is_not_trans = (self.0.physical() % 3).not_equal(TRANS).all_kleene()?;
        Some(is_unsaturated & is_not_trans)
    }

    pub fn is_trans(&self) -> Option<bool> {
        let is_unsaturated = self.unsaturated();
        let is_trans = (self.0.physical() % 3).equal(TRANS);
        Some((is_unsaturated & is_trans).any_kleene()?)
    }

    /// Checks if the bound chunked array contains unsaturated cis-only bonds.
    ///
    /// # Returns
    ///
    /// [`true`] if all unsaturated bounds are cis, [`false`] otherwise.
    pub fn is_cis_old(&self) -> PolarsResult<Option<bool>> {
        let mut is_cis = Some(false);
        for bound in self.try_iter() {
            let bound = bound?;
            is_cis = match bound {
                Some(Explicit::Saturated) => is_cis,
                Some(Explicit::Unsaturated(Unsaturated {
                    isomerism: Some(Isomerism::Cis),
                    ..
                })) => Some(true),
                Some(Explicit::Unsaturated(Unsaturated {
                    isomerism: Some(Isomerism::Trans),
                    ..
                })) => return Ok(Some(false)),
                _ => None,
            };
        }
        Ok(is_cis)
    }

    /// Checks if the bound chunked array contains any trans bonds.
    ///
    /// # Returns
    ///
    /// [`true`] if any bound is trans, [`false`] otherwise.
    pub fn is_trans_old(&self) -> PolarsResult<Option<bool>> {
        let mut is_trans = Some(false);
        for bound in self.try_iter() {
            let bound = bound?;
            is_trans = match bound {
                Some(Explicit::Saturated) => is_trans,
                Some(Explicit::Unsaturated(Unsaturated {
                    isomerism: Some(Isomerism::Cis),
                    ..
                })) => is_trans,
                Some(Explicit::Unsaturated(Unsaturated {
                    isomerism: Some(Isomerism::Trans),
                    ..
                })) => return Ok(Some(true)),
                _ => None,
            };
        }
        Ok(is_trans)
    }
}

impl ChunkCompareEq<&str> for BoundChunked {
    type Item = PolarsResult<BooleanChunked>;

    fn equal(&self, rhs: &str) -> Self::Item {
        self.0.equal(rhs)
    }

    fn equal_missing(&self, rhs: &str) -> Self::Item {
        self.0.equal_missing(rhs)
    }

    fn not_equal(&self, rhs: &str) -> Self::Item {
        self.0.not_equal(rhs)
    }

    fn not_equal_missing(&self, rhs: &str) -> Self::Item {
        self.0.not_equal_missing(rhs)
    }
}

impl ChunkCompareEq<u32> for BoundChunked {
    type Item = BooleanChunked;

    fn equal(&self, rhs: u32) -> Self::Item {
        self.0.physical().equal(rhs)
    }

    fn equal_missing(&self, rhs: u32) -> Self::Item {
        self.0.physical().equal_missing(rhs)
    }

    fn not_equal(&self, rhs: u32) -> Self::Item {
        self.0.physical().not_equal(rhs)
    }

    fn not_equal_missing(&self, rhs: u32) -> Self::Item {
        self.0.physical().not_equal_missing(rhs)
    }
}

impl ChunkCompareIneq<&str> for BoundChunked {
    type Item = PolarsResult<BooleanChunked>;

    fn gt(&self, rhs: &str) -> Self::Item {
        self.0.gt(rhs)
    }

    fn gt_eq(&self, rhs: &str) -> Self::Item {
        self.0.gt_eq(rhs)
    }

    fn lt(&self, rhs: &str) -> Self::Item {
        self.0.lt(rhs)
    }

    fn lt_eq(&self, rhs: &str) -> Self::Item {
        self.0.lt_eq(rhs)
    }
}

impl ChunkCompareIneq<u32> for BoundChunked {
    type Item = BooleanChunked;

    fn gt(&self, rhs: u32) -> Self::Item {
        self.0.physical().gt(rhs)
    }

    fn gt_eq(&self, rhs: u32) -> Self::Item {
        self.0.physical().gt_eq(rhs)
    }

    fn lt(&self, rhs: u32) -> Self::Item {
        self.0.physical().lt(rhs)
    }

    fn lt_eq(&self, rhs: u32) -> Self::Item {
        self.0.physical().lt_eq(rhs)
    }
}

unsafe impl IntoSeries for BoundChunked {
    fn into_series(self) -> Series {
        self.0.into_series()
    }
}

impl Debug for BoundChunked {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("BoundChunked")
            .field(&self.0.physical())
            .finish()
    }
}

impl Display for BoundChunked {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut iter = self.iter();
        if let Some(bound) = iter.next() {
            Display::fmt(&bound, f)?;
            for bound in iter {
                Display::fmt(&bound, f)?;
            }
        }
        Ok(())
    }
}

impl IntoIterator for &BoundChunked {
    type Item = Bound;

    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> TryFrom<&'a CategoricalChunked> for &'a BoundChunked {
    type Error = PolarsError;

    fn try_from(value: &'a CategoricalChunked) -> Result<Self, Self::Error> {
        check_data_type(value)?;
        // [safe](https://doc.rust-lang.org/reference/type-layout.html?highlight=transparent#the-transparent-representation)
        Ok(unsafe { &*(value as *const CategoricalChunked as *const BoundChunked) })
    }
}

impl<'a> TryFrom<&'a Series> for &'a BoundChunked {
    type Error = PolarsError;

    fn try_from(value: &'a Series) -> Result<Self, Self::Error> {
        value.categorical()?.try_into()
    }
}

impl<const N: usize, T: Identifier> TryFrom<[T; N]> for BoundChunked {
    type Error = PolarsError;

    fn try_from(value: [T; N]) -> Result<Self, Self::Error> {
        let mut identifiers = EnumChunkedBuilder::new(
            PlSmallStr::from_static(BOUND),
            value.len(),
            MAP.clone(),
            Default::default(),
            true,
        );
        for identifier in value {
            identifier.append_to(&mut identifiers)?;
        }
        Ok(Self::new(identifiers.finish()))
    }
}

impl<T: Copy + Identifier> TryFrom<&[T]> for BoundChunked {
    type Error = PolarsError;

    fn try_from(value: &[T]) -> Result<Self, Self::Error> {
        let mut identifiers = EnumChunkedBuilder::new(
            PlSmallStr::from_static(BOUND),
            value.len(),
            MAP.clone(),
            Default::default(),
            true,
        );
        for identifier in value {
            identifier.append_to(&mut identifiers)?;
        }
        Ok(Self::new(identifiers.finish()))
    }
}

impl FromIterator<Bound> for BoundChunked {
    fn from_iter<T: IntoIterator<Item = Bound>>(iter: T) -> Self {
        BoundChunked::new(unsafe {
            CategoricalChunked::from_cats_and_rev_map_unchecked(
                UInt32Chunked::from_iter_options(
                    BOUND.into(),
                    iter.into_iter().map(|bound| bound.categorical()),
                ),
                MAP.clone(),
                true,
                CategoricalOrdering::Physical,
            )
        })
    }
}

#[cfg(feature = "atomic")]
impl Atomic for &BoundChunked {
    type Output = u8;

    #[inline]
    fn carbons(self) -> u8 {
        self.bounds() + 1
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

fn check_data_type(categorical: &CategoricalChunked) -> PolarsResult<()> {
    polars_ensure!(
        *categorical.dtype() == *BOUND_DATA_TYPE,
        SchemaMismatch: "invalid bound data type: expected `BOUND_DATA_TYPE`, got = `{}`",
        categorical.dtype(),
    );
    Ok(())
}

#[cfg(feature = "mass")]
mod mass;
