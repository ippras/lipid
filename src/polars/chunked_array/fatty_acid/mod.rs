// pub use self::iter::{IdentifierIteratorExt, IndexIteratorExt};

use crate::prelude::*;
use polars::prelude::{enum_::EnumChunkedBuilder, *};
use std::{
    borrow::Cow,
    fmt::{self, Debug, Display, Formatter, Write, from_fn},
    num::NonZeroI8,
    sync::LazyLock,
};

// /// The fatty acid data type.
// pub const FATTY_ACID_DATA_TYPE: LazyLock<DataType> = LazyLock::new(|| {
//     DataType::Struct(vec![
//         Field::new(PlSmallStr::from_static(INDEX), DataType::Int8),
//         Field::new(
//             PlSmallStr::from_static(BOUND),
//             TRIPLE_BOUND_DATA_TYPE.clone(),
//         ),
//     ])
// });

/// The fatty acid data type.
pub const FATTY_ACID_DATA_TYPE: LazyLock<DataType> = LazyLock::new(|| {
    DataType::Struct(vec![
        Field::new(
            PlSmallStr::from_static(FORMULA),
            DataType::Struct(vec![
                Field::new(PlSmallStr::from_static("C"), DataType::UInt8),
                Field::new(PlSmallStr::from_static("H"), DataType::UInt8),
                Field::new(PlSmallStr::from_static("O"), DataType::UInt8),
            ]),
        ),
        // Field::new(PlSmallStr::from_static(FORMULA), DataType::UInt8),
        // Field::new(
        //     PlSmallStr::from_static(DOUBLE_BOUNDS),
        //     DataType::List(Box::new(DataType::Struct(vec![
        //         Field::new(PlSmallStr::from_static(INDEX), DataType::Int8),
        //         Field::new(PlSmallStr::from_static(PARITY), DataType::Boolean),
        //     ]))),
        // ),
        Field::new(
            PlSmallStr::from_static(TRIPLE_BOUNDS),
            DataType::List(Box::new(DataType::Int8)),
        ),
    ])
});

/// Fatty acid chunked array.
#[derive(Clone, Debug)]
pub struct FattyAcidChunked {
    formula: Formula,
    triple_bound_list: TripleBoundListChunked,
}

impl FattyAcidChunked {
    // pub fn iter(
    //     &self,
    // ) -> impl DoubleEndedIterator<Item = (Option<Option<NonZeroI8>>, Bound)> + ExactSizeIterator
    // {
    //     self.formula
    //         .iter()
    //         .map(|index| index.map(NonZeroI8::new))
    //         .zip(self.triple_bound.iter())
    // }

    pub fn into_struct(self, name: PlSmallStr) -> PolarsResult<StructChunked> {
        StructChunked::from_series(
            name,
            self.formula.0.len(),
            [
                self.formula.0.into_series(),
                self.triple_bound_list.into_series(),
            ]
            .iter(),
        )
    }

    pub fn sort(self) -> FattyAcidChunked {
        Self {
            formula: self.formula,
            triple_bound_list: self.triple_bound_list.sort(false),
        }
    }

    pub fn triple_bounds(
        &self,
    ) -> impl Iterator<Item = PolarsResult<Option<Int8Chunked>>> + ExactSizeIterator {
        self.triple_bound_list.0.amortized_iter().map(|series| {
            let Some(series) = series else {
                return Ok(None);
            };
            let index = series.as_ref().i8()?.clone();
            Ok(Some(index))
        })
    }
}

// Map.
impl FattyAcidChunked {
    /// Returns the total number of unsaturations in the fatty acid.
    pub fn unsaturation(&self) -> PolarsResult<UInt8Chunked> {
        self.triple_bound_list
            .map(|index| index.len() as u8 * 4)
            .collect()
    }
}

// Mask.
impl FattyAcidChunked {
    #[inline]
    pub fn is_saturated(&self) -> PolarsResult<BooleanChunked> {
        self.triple_bound_list.mask(|index| index.is_empty())
    }

    #[inline]
    pub fn is_unsaturated(&self, offset: Option<NonZeroI8>) -> PolarsResult<BooleanChunked> {
        self.triple_bound_list.mask(|index| !index.is_empty())
    }

    #[inline]
    pub fn is_monounsaturated(&self) -> PolarsResult<BooleanChunked> {
        self.triple_bound_list.mask(|index| index.len() == 1)
    }

    #[inline]
    pub fn is_polyunsaturated(&self) -> PolarsResult<BooleanChunked> {
        self.triple_bound_list.mask(|index| index.len() > 1)
    }

    #[inline]
    pub fn is_cis(&self) -> bool {
        true
        // self.triple_bound_list.is_cis()
    }

    #[inline]
    pub fn is_trans(&self) -> bool {
        false
        // self.triple_bound_list.is_trans()
    }
}

impl FattyAcidChunked {
    /// Displays the bound chunked array as a [`FattyAcid`] based on the
    /// provided options.
    pub fn display(&self, options: Options) -> impl Display {
        from_fn(move |f| {
            // Display::fmt(&self.carbons(), f)?;
            // f.write_char(':')?;
            // Display::fmt(&self.unsaturated_count(), f)?;
            // if f.alternate() {
            //     let item = |index: Option<Option<NonZeroI8>>, unsaturated: Unsaturated| {
            //         from_fn(move |f| {
            //             match index {
            //                 None => f.write_char('*')?,
            //                 Some(None) => f.write_char('?')?,
            //                 Some(Some(index)) => Display::fmt(&index, f)?,
            //             }
            //             match unsaturated.unsaturation {
            //                 None => f.write_char('≊')?, // ≅
            //                 Some(Unsaturation::Double) => {
            //                     if options.isomerism == Elision::Explicit {
            //                         f.write_char('=')?;
            //                     }
            //                 }
            //                 Some(Unsaturation::Triple) => f.write_char('≡')?,
            //             }
            //             match unsaturated.isomerism {
            //                 None => {}
            //                 Some(Isomerism::Cis) => {
            //                     if options.isomerism == Elision::Explicit {
            //                         f.write_char('c')?;
            //                     }
            //                 }
            //                 Some(Isomerism::Trans) => f.write_char('t')?,
            //             }
            //             Ok(())
            //         })
            //     };
            //     let mut iter = self.iter().unsaturated();
            //     if let Some((index, unsaturated)) = iter.next() {
            //         match index {
            //             Some(Some(index)) if index.is_negative() => f.write_char('ω')?,
            //             _ => f.write_char('Δ')?,
            //         }
            //         Display::fmt(&item(index, unsaturated), f)?;
            //         for (index, unsaturated) in iter {
            //             f.write_char(',')?;
            //             Display::fmt(&item(index, unsaturated), f)?;
            //         }
            //     }
            // }
            Ok(())
        })
    }
}

// impl Default for FattyAcidChunked {
//     fn default() -> Self {
//         Self {
//             formula: Int8Chunked::default().with_name(INDEX.into()),
//             triple_bound_list: TripleBoundListChunked::default().with_name(BOUND.into()),
//         }
//     }
// }

// impl Display for FattyAcidChunked {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         let item = |index: Option<Option<NonZeroI8>>, bound: Bound| {
//             from_fn(move |f| {
//                 Display::fmt(&bound, f)?;
//                 match index {
//                     None if f.alternate() => f.write_char('*')?,
//                     Some(None) if f.alternate() => f.write_char('?')?,
//                     Some(Some(index)) if f.alternate() => Display::fmt(&index, f)?,
//                     None => f.write_char('m')?,
//                     Some(None) => f.write_char('o')?,
//                     Some(Some(index)) => Display::fmt(&index, f)?,
//                 }
//                 Ok(())
//             })
//         };
//         let mut iter = self.iter();
//         if let Some((index, bound)) = iter.next() {
//             Display::fmt(&item(index, bound), f)?;
//             for (index, bound) in iter {
//                 Display::fmt(&item(index, bound), f)?;
//             }
//         }
//         Ok(())
//     }
// }

// impl IntoIterator for &FattyAcidChunked {
//     type Item = (Option<Option<NonZeroI8>>, Bound);

//     type IntoIter = impl Iterator<Item = Self::Item>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.iter()
//     }
// }

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
        let formula = Formula(value.field_by_name(FORMULA)?.struct_()?.clone());
        let triple_bound_list =
            TripleBoundListChunked::try_new(value.field_by_name(TRIPLE_BOUNDS)?.list()?.clone())?;
        Ok(Self {
            formula,
            triple_bound_list,
        })
    }
}

// impl<const N: usize, U: Identifier> TryFrom<[(Option<Option<NonZeroI8>>, U); N]>
//     for FattyAcidChunked
// {
//     type Error = PolarsError;

//     fn try_from(value: [(Option<Option<NonZeroI8>>, U); N]) -> Result<Self, Self::Error> {
//         let capacity = value.len();
//         let mut indices = PrimitiveChunkedBuilder::<Int8Type>::new(INDEX.into(), capacity);
//         let mut bounds = EnumChunkedBuilder::new(
//             BOUND.into(),
//             capacity,
//             MAP.clone(),
//             Default::default(),
//             true,
//         );
//         for (index, identifier) in value {
//             let index = index.map(|index| index.map_or(0, |index| index.into()));
//             indices.append_option(index);
//             identifier.append_to(&mut bounds)?;
//         }
//         Ok(Self {
//             formula: indices.finish(),
//             triple_bound_list: TripleBoundListChunked::new(bounds.finish()),
//         })
//     }
// }

// impl<const N: usize, T: Identifier> TryFrom<[T; N]> for FattyAcidChunked {
//     type Error = PolarsError;

//     fn try_from(value: [T; N]) -> Result<Self, Self::Error> {
//         Ok(Self {
//             formula: Int8Chunked::new_vec(INDEX.into(), (1i8..=value.len() as _).collect()),
//             triple_bound_list: value.try_into()?,
//         })
//     }
// }

// impl<T: Copy + Identifier> TryFrom<&[T]> for FattyAcidChunked {
//     type Error = PolarsError;

//     fn try_from(value: &[T]) -> Result<Self, Self::Error> {
//         Ok(Self {
//             formula: Int8Chunked::new_vec(INDEX.into(), (1i8..=value.len() as _).collect()),
//             triple_bound_list: value.try_into()?,
//         })
//     }
// }

// #[cfg(feature = "atomic")]
// impl Atomic for &FattyAcidChunked {
//     type Output = u8;

//     #[inline]
//     fn carbons(self) -> u8 {
//         self.triple_bound_list.carbons()
//     }

//     #[inline]
//     fn hydrogens(self) -> u8 {
//         self.triple_bound_list.hydrogens()
//     }
// }

// #[cfg(feature = "ecn")]
// impl EquivalentCarbonNumber for &FattyAcidChunked {
//     type Output = u8;

//     #[inline]
//     fn equivalent_carbon_number(self) -> u8 {
//         self.triple_bound_list.equivalent_carbon_number()
//     }
// }

// mod iter;
// #[cfg(feature = "mass")]
// mod mass;

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
