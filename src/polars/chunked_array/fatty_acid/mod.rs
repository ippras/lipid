pub use self::iter::{IdentifierIteratorExt, IndexIteratorExt};

use super::Identifier;
use crate::prelude::{physical::S, *};
use polars::prelude::{enum_::EnumChunkedBuilder, *};
use std::{
    borrow::Cow,
    fmt::{self, Debug, Display, Formatter, Write, from_fn},
    num::NonZeroI8,
    sync::LazyLock,
};

/// The fatty acid data type.
pub const FATTY_ACID_DATA_TYPE: LazyLock<DataType> = LazyLock::new(|| {
    DataType::Struct(vec![
        Field::new(PlSmallStr::from_static(INDICES), DataType::Int8),
        Field::new(PlSmallStr::from_static(BOUNDS), BOUND_DATA_TYPE.clone()),
    ])
});

/// Fatty acid chunked array.
#[derive(Clone, Debug, Default)]
pub struct FattyAcidChunked {
    indices: Int8Chunked,
    bounds: BoundChunked,
}

impl FattyAcidChunked {
    pub const INDICES: &str = INDICES;
    pub const BOUNDS: &str = BOUNDS;

    pub fn iter(
        &self,
    ) -> impl DoubleEndedIterator<Item = (Option<Option<NonZeroI8>>, Option<Bound>)> + ExactSizeIterator
    {
        self.indices
            .iter()
            .map(|index| index.map(NonZeroI8::new))
            .zip(self.bounds.iter())
    }

    pub fn into_struct(self, name: PlSmallStr) -> PolarsResult<StructChunked> {
        StructChunked::from_series(
            name,
            self.bounds.as_categorical().len(),
            [self.indices.into_series(), self.bounds.into_series()].iter(),
        )
    }

    pub fn index(&self) -> &Int8Chunked {
        &self.indices
    }

    pub fn bound(&self) -> &BoundChunked {
        &self.bounds
    }

    pub fn sort(&self) -> FattyAcidChunked {
        // SAFETY: we only reordered the indexes so we are still in bounds.
        let options = Default::default();
        let bounds_indices = self.bounds.as_categorical().arg_sort(options);
        let mut indices = unsafe { self.indices.take_unchecked(&bounds_indices) };
        let mut identifiers = unsafe {
            self.bounds
                .as_categorical()
                .physical()
                .take_unchecked(&bounds_indices)
        };
        let index_indices = indices.arg_sort(options);
        indices = unsafe { indices.take_unchecked(&index_indices) };
        identifiers = unsafe { identifiers.take_unchecked(&index_indices) };
        let bounds = BoundChunked::new(unsafe {
            CategoricalChunked::from_cats_and_rev_map_unchecked(
                identifiers,
                self.bounds.as_categorical().get_rev_map().clone(),
                self.bounds.as_categorical().is_enum(),
                self.bounds.as_categorical().get_ordering(),
            )
        });
        Self { indices, bounds }
    }
}

impl FattyAcidChunked {
    pub fn unsized_sized(&self) -> (Option<Option<Bound>>, Cow<Self>) {
        if self.is_sized() {
            return (None, Cow::Borrowed(&self));
        }
        let null_count = self.indices.null_count();
        assert_eq!(null_count, 1);
        let indices = self.indices.slice(1, self.indices.len() - 1);
        let physical = self.bounds.as_categorical().physical();
        let r#unsized = physical
            .first()
            .map(|physical| Bound::new(self.bounds.as_categorical().get_rev_map().get(physical)));
        let sized = physical.slice(1, physical.len() - 1);
        // SAFETY: we created the physical from the enum categories.
        let sized = unsafe {
            CategoricalChunked::from_cats_and_dtype_unchecked(sized, BOUND_DATA_TYPE.clone())
        };
        (
            Some(r#unsized),
            Cow::Owned(Self {
                indices,
                bounds: BoundChunked::new(sized),
            }),
        )
    }

    pub fn implicit_explicit(&self) -> (BoundChunked, Self) {
        assert!(!self.indices.has_nulls());
        let offset = self.implicit_count() as _;
        let (implicit, explicit) = self.bounds.as_categorical().physical().split_at(offset);
        // SAFETY: we created the physical from the enum categories.
        let implicit = unsafe {
            CategoricalChunked::from_cats_and_dtype_unchecked(implicit, BOUND_DATA_TYPE.clone())
        };
        let index = self
            .indices
            .slice(offset, self.indices.len() - offset as usize);
        let explicit = unsafe {
            CategoricalChunked::from_cats_and_dtype_unchecked(explicit, BOUND_DATA_TYPE.clone())
        };
        (
            BoundChunked(implicit),
            Self {
                indices: index,
                bounds: BoundChunked(explicit),
            },
        )
    }
}

// Mask.
impl FattyAcidChunked {
    #[inline]
    pub fn is_sized(&self) -> bool {
        !self.indices.has_nulls()
    }

    #[inline]
    pub fn is_unsized(&self) -> bool {
        self.indices.has_nulls()
    }

    #[inline]
    pub fn is_explicit(&self) -> Option<bool> {
        self.indices.not_equal(0).all_kleene()
    }

    #[inline]
    pub fn is_implicit(&self) -> Option<bool> {
        self.indices.equal(0).any_kleene()
    }
}

impl FattyAcidChunked {
    #[inline]
    pub fn is_saturated(&self) -> Option<bool> {
        self.bounds.is_saturated()
    }

    pub fn is_unsaturated(&self, offset: Option<NonZeroI8>) -> Option<bool> {
        let Some(offset) = offset else {
            return self.bounds.is_unsaturated();
        };
        let mut is_unsaturated;
        if offset.is_positive() {
            let delta = offset.get();
            let carbons = self.carbons();
            if carbons < delta as _ {
                return Some(false);
            }
            let unsaturated = self.indices.equal(delta) & self.bounds.gt(S);
            is_unsaturated = unsaturated.num_trues() == 1;
            let saturated = self.indices.lt(delta) & self.bounds.equal(S);
            is_unsaturated &= saturated.num_trues() == delta as usize - 1;
        } else {
            let offset = offset.get();
            let carbons = self.carbons();
            let Some(delta) = carbons.checked_add_signed(offset) else {
                return Some(false);
            };
            let unsaturated = self.indices.equal(delta) & self.bounds.gt(S);
            is_unsaturated = unsaturated.num_trues() == 1;
            let saturated = self.indices.gt(delta) & self.bounds.equal(S);
            is_unsaturated &= saturated.num_trues() == offset.abs() as usize - 1;
        }
        Some(is_unsaturated)
    }

    #[inline]
    pub fn is_monounsaturated(&self) -> Option<bool> {
        self.bounds.is_monounsaturated()
    }

    #[inline]
    pub fn is_polyunsaturated(&self) -> Option<bool> {
        self.bounds.is_polyunsaturated()
    }

    #[inline]
    pub fn is_cis(&self) -> Option<bool> {
        self.bounds.is_cis()
    }

    #[inline]
    pub fn is_trans(&self) -> Option<bool> {
        self.bounds.is_trans()
    }
}

// Count.
impl FattyAcidChunked {
    /// Returns the number of sized (Some) indices in the fatty acid.
    pub fn sized_count(&self) -> u8 {
        (self.indices.len() - self.indices.null_count()) as _
    }

    /// Returns the number of unsized (None) indices in the fatty acid.
    pub fn unsized_count(&self) -> u8 {
        self.indices.null_count() as _
    }

    // Returns the number of explicit (NonZeroI8) indices in the fatty acid.
    pub fn explicit_count(&self) -> u8 {
        self.indices.not_equal(0).num_trues() as _
    }

    // Returns the number of implicit (Some(0)) indices in the fatty acid.
    pub fn implicit_count(&self) -> u8 {
        self.indices.equal(0).num_trues() as _
    }
}

impl FattyAcidChunked {
    pub fn saturated_count(&self) -> u8 {
        (self.indices.is_not_null() & self.bounds.equal(S)).num_trues() as _
    }

    pub fn unsaturated_count(&self) -> u8 {
        (self.indices.is_not_null() & self.bounds.not_equal(S)).num_trues() as _
    }

    /// Returns the total number of unsaturations in the fatty acid.
    pub fn unsaturation(&self) -> u8 {
        self.iter()
            .unsaturated()
            .filter_map(|(_, unsaturated)| unsaturated.unsaturation())
            .sum()
    }
}

impl FattyAcidChunked {
    /// Displays the bound chunked array as a [`FattyAcid`] based on the
    /// provided options.
    pub fn display(&self, options: Options) -> impl Display {
        from_fn(move |f| {
            Display::fmt(&self.carbons(), f)?;
            f.write_char(':')?;
            Display::fmt(&self.unsaturated_count(), f)?;
            if f.alternate() {
                let item = |index: Option<Option<NonZeroI8>>, unsaturated: Unsaturated| {
                    from_fn(move |f| {
                        match index {
                            None => f.write_char('*')?,
                            Some(None) => f.write_char('?')?,
                            Some(Some(index)) => Display::fmt(&index, f)?,
                        }
                        match unsaturated.unsaturation {
                            None => f.write_char('≅')?,
                            Some(Unsaturation::Double) => {
                                if options.isomerism == Elision::Explicit {
                                    f.write_char('=')?;
                                }
                            }
                            Some(Unsaturation::Triple) => f.write_char('≡')?,
                        }
                        match unsaturated.isomerism {
                            None => {}
                            Some(Isomerism::Cis) => {
                                if options.isomerism == Elision::Explicit {
                                    f.write_char('c')?;
                                }
                            }
                            Some(Isomerism::Trans) => f.write_char('t')?,
                        }
                        Ok(())
                    })
                };
                let mut iter = self.iter().unsaturated();
                if let Some((index, unsaturated)) = iter.next() {
                    match index {
                        Some(Some(index)) if index.is_negative() => f.write_char('ω')?,
                        _ => f.write_char('Δ')?,
                    }
                    Display::fmt(&item(index, unsaturated), f)?;
                    for (index, unsaturated) in iter {
                        f.write_char(',')?;
                        Display::fmt(&item(index, unsaturated), f)?;
                    }
                }
            }
            Ok(())
        })
    }
}

impl Display for FattyAcidChunked {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let item = |index: Option<Option<NonZeroI8>>, bound: Option<Bound>| {
            from_fn(move |f| {
                match bound {
                    None => f.write_str("b")?,
                    Some(Bound::Saturated) => f.write_str("s")?,
                    Some(Bound::Unsaturated(unsaturated)) => {
                        match unsaturated.unsaturation {
                            None => f.write_str("u")?,
                            Some(Unsaturation::Double) => f.write_str("d")?,
                            Some(Unsaturation::Triple) => f.write_str("t")?,
                        }
                        match unsaturated.isomerism {
                            None => {}
                            Some(Isomerism::Cis) => f.write_str("c")?,
                            Some(Isomerism::Trans) => f.write_str("t")?,
                        }
                    }
                }
                match index {
                    None if f.alternate() => f.write_char('*')?,
                    Some(None) if f.alternate() => f.write_char('?')?,
                    Some(Some(index)) if f.alternate() => Display::fmt(&index, f)?,
                    None => f.write_char('m')?,
                    Some(None) => f.write_char('o')?,
                    Some(Some(index)) => Display::fmt(&index, f)?,
                }
                Ok(())
            })
        };
        let mut iter = self.iter();
        if let Some((index, bound)) = iter.next() {
            Display::fmt(&item(index, bound), f)?;
            for (index, bound) in iter {
                Display::fmt(&item(index, bound), f)?;
            }
        }
        Ok(())
    }
}

impl IntoIterator for &FattyAcidChunked {
    type Item = (Option<Option<NonZeroI8>>, Option<Bound>);

    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl TryFrom<&Series> for FattyAcidChunked {
    type Error = PolarsError;

    fn try_from(value: &Series) -> Result<Self, Self::Error> {
        value.struct_()?.try_into()
    }
}

impl TryFrom<&StructChunked> for FattyAcidChunked {
    type Error = PolarsError;

    fn try_from(value: &StructChunked) -> Result<Self, Self::Error> {
        polars_ensure!(
            *value.dtype() == *FATTY_ACID_DATA_TYPE,
            SchemaMismatch: "invalid fatty acid data type: expected `FATTY_ACID_DATA_TYPE`, got = `{}`",
            value.dtype(),
        );
        let indices = value.field_by_name(Self::INDICES)?.i8()?.clone();
        let bounds =
            BoundChunked::try_new(value.field_by_name(Self::BOUNDS)?.categorical()?.clone())?;
        Ok(Self { indices, bounds })
    }
}

impl<const N: usize, U: Identifier> TryFrom<[(Option<Option<NonZeroI8>>, U); N]>
    for FattyAcidChunked
{
    type Error = PolarsError;

    fn try_from(value: [(Option<Option<NonZeroI8>>, U); N]) -> Result<Self, Self::Error> {
        let capacity = value.len();
        let mut indices = PrimitiveChunkedBuilder::<Int8Type>::new(INDICES.into(), capacity);
        let mut identifiers = EnumChunkedBuilder::new(
            BOUNDS.into(),
            capacity,
            MAP.clone(),
            Default::default(),
            true,
        );
        for (index, identifier) in value {
            let index = index.map(|index| index.map_or(0, |index| index.into()));
            indices.append_option(index);
            identifier.append_to(&mut identifiers)?;
        }
        Ok(Self {
            indices: indices.finish(),
            bounds: BoundChunked::new(identifiers.finish()),
        })
    }
}

impl<const N: usize, T: Identifier> TryFrom<[T; N]> for FattyAcidChunked {
    type Error = PolarsError;

    fn try_from(value: [T; N]) -> Result<Self, Self::Error> {
        Ok(Self {
            indices: Int8Chunked::new_vec(INDICES.into(), (1i8..=value.len() as _).collect()),
            bounds: value.try_into()?,
        })
    }
}

#[cfg(feature = "atomic")]
impl Atomic for &FattyAcidChunked {
    type Output = u8;

    #[inline]
    fn carbons(self) -> u8 {
        self.bounds.carbons()
    }

    #[inline]
    fn hydrogens(self) -> u8 {
        self.bounds.hydrogens()
    }
}

#[cfg(feature = "ecn")]
impl EquivalentCarbonNumber for &FattyAcidChunked {
    type Output = u8;

    #[inline]
    fn equivalent_carbon_number(self) -> u8 {
        self.bounds.equivalent_carbon_number()
    }
}

mod iter;
#[cfg(feature = "mass")]
mod mass;

// impl FattyAcidChunked {
//     pub fn try_is_unsaturated(&self, offset: Option<NonZeroI8>) -> PolarsResult<Option<bool>> {
//         let Some(offset) = offset else {
//             return Ok(self.bounds.is_unsaturated());
//         };
//         let mut is_unsaturated;
//         if offset.is_positive() {
//             let delta = offset.get();
//             let carbons = self.carbons();
//             if carbons < delta as _ {
//                 return Ok(Some(false));
//             }
//             let unsaturated = self.indices.equal(delta) & self.bounds.gt(S);
//             is_unsaturated = unsaturated.num_trues() == 1;
//             let saturated = self.indices.lt(delta) & self.bounds.equal(S);
//             is_unsaturated &= saturated.num_trues() == delta as usize - 1;
//         } else {
//             let offset = offset.get();
//             let carbons = self.carbons();
//             let Some(delta) = carbons.checked_add_signed(offset) else {
//                 return Ok(Some(false));
//             };
//             let unsaturated = self.indices.equal(delta) & self.bounds.gt(S);
//             is_unsaturated = unsaturated.num_trues() == 1;
//             let saturated = self.indices.gt(delta) & self.bounds.equal(S);
//             is_unsaturated &= saturated.num_trues() == offset.abs() as usize - 1;
//         }
//         Ok(Some(is_unsaturated))
//     }

//     pub fn is_delta_unsaturated(&self, offset: NonZeroU8) -> PolarsResult<Option<bool>> {
//         let delta = offset.get();
//         let carbons = self.carbons();
//         if carbons < delta {
//             return Ok(Some(false));
//         }
//         let unsaturated = self.indices.equal(delta) & self.bounds.gt(S);
//         let mut is_unsaturated = unsaturated.num_trues() == 1;
//         let saturated = self.indices.lt(delta) & self.bounds.equal(S);
//         is_unsaturated &= saturated.num_trues() == delta as usize - 1;
//         Ok(Some(is_unsaturated))
//     }

//     pub fn is_omega_unsaturated(&self, offset: NonZeroU8) -> PolarsResult<Option<bool>> {
//         let offset = offset.get();
//         let carbons = self.carbons();
//         if carbons < offset {
//             return Ok(Some(false));
//         }
//         let delta = carbons - offset;
//         let unsaturated = self.indices.equal(delta) & self.bounds.gt(S);
//         let mut is_omega_unsaturated = unsaturated.num_trues() == 1;
//         let saturated = self.indices.gt(delta) & self.bounds.equal(S);
//         is_omega_unsaturated &= saturated.num_trues() == offset as usize - 1;
//         Ok(Some(is_omega_unsaturated))
//     }

//     pub fn is_delta_unsaturated_old(&self, index: NonZeroU8) -> PolarsResult<Option<bool>> {
//         // println!("is_delta: {index}");
//         let index = index.get();
//         let length = self.carbons();
//         if index >= length {
//             return Ok(Some(false));
//         }
//         let mut sized = Some(0);
//         let mut r#unsized = Some(Unsized::default());
//         let unsaturated = index;
//         let saturated = 1..index;
//         for (index, bound) in self.try_iter() {
//             let bound = bound?;
//             match index {
//                 Some(Some(index)) => {
//                     // Convert to delta
//                     let index = if index.is_positive() {
//                         index.get() as _
//                     } else {
//                         length.saturating_add_signed(index.get())
//                     };
//                     if index == unsaturated {
//                         match bound {
//                             Some(Bound::Saturated) => return Ok(Some(false)),
//                             Some(Bound::Unsaturated(_)) => {
//                                 if let Some(sized) = &mut sized {
//                                     *sized += 1
//                                 }
//                             }
//                             None => {
//                                 sized = None;
//                                 if let Some(r#unsized) = &mut r#unsized {
//                                     r#unsized.any += 1;
//                                 }
//                             }
//                         }
//                     } else if saturated.contains(&index) {
//                         match bound {
//                             Some(Bound::Saturated) => {
//                                 if let Some(sized) = &mut sized {
//                                     *sized += 1
//                                 }
//                             }
//                             Some(Bound::Unsaturated(_)) => return Ok(Some(false)),
//                             None => {
//                                 sized = None;
//                                 if let Some(r#unsized) = &mut r#unsized {
//                                     r#unsized.any += 1;
//                                 }
//                             }
//                         }
//                     }
//                 }
//                 Some(None) => {
//                     if let Some(r#unsized) = &mut r#unsized {
//                         match bound {
//                             Some(Bound::Saturated) => r#unsized.saturated += 1,
//                             Some(Bound::Unsaturated(_)) => r#unsized.unsaturated += 1,
//                             None => r#unsized.any += 1,
//                         }
//                     }
//                 }
//                 None => r#unsized = None,
//             }
//         }
//         let Some(sized) = sized else {
//             return Ok(None);
//         };
//         Ok(Some(sized == unsaturated))
//     }

//     pub fn is_omega_unsaturated_old(&self, offset: NonZeroU8) -> Option<bool> {
//         let mut is_omega_unsaturated = Some(true);
//         let length = offset.get() as usize;
//         let mut iter = self.iter().rev();
//         if iter.len() < length {
//             return Some(false);
//         }
//         for _ in 0..length - 1 {
//             let Some((index, bound)) = iter.next() else {
//                 return Some(false);
//             };
//             if let None | Some(None) = index {
//                 is_omega_unsaturated = None;
//             }
//             let Some(Bound::Saturated) = bound else {
//                 return Some(false);
//             };
//         }
//         let Some((index, bound)) = iter.next() else {
//             return Some(false);
//         };
//         if let None | Some(None) = index {
//             is_omega_unsaturated = None;
//         }
//         let Some(Bound::Unsaturated(_)) = bound else {
//             return Some(false);
//         };
//         is_omega_unsaturated
//     }
// }

// #[derive(Clone, Copy, Debug, Default)]
// struct Unsized {
//     saturated: usize,
//     unsaturated: usize,
//     any: usize,
// }
