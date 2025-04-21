use crate::prelude::*;
use polars::prelude::{enum_::EnumChunkedBuilder, *};
use std::{
    borrow::Cow,
    fmt::{self, Display, Formatter, Write, from_fn},
    num::NonZeroI8,
    sync::LazyLock,
};

// Index
//
// None - любое количество связей с неизвестными индексами
// Some(Some(0) (Some(None)) - одна связь с неизвестным индексом
// Some(Some(NonZeroI8)) - одна связь с известным индексом

// Identifier
//
// None: любая связь связь
// S: насыщенная связь
// D: ненасыщенная двойная связь
// DC: ненасыщенная двойная цис связь
// DT: ненасыщенная двойная транс связь
// T: ненасыщенная тройная связь
// TC: ненасыщенная тройная цис связь
// TT: ненасыщенная тройная транс связь
// U: ненасыщенная связь
// UC: ненасыщенная цис связь
// UT: ненасыщенная транс связь

/// The fatty acid data type.
pub const FATTY_ACID_DATA_TYPE: LazyLock<DataType> = LazyLock::new(|| {
    DataType::Struct(vec![
        Field::new(PlSmallStr::from_static(INDEX), DataType::Int8),
        Field::new(PlSmallStr::from_static(IDENTIFIER), BOUND_DATA_TYPE.clone()),
    ])
});

/// Fatty acid chunked array.
#[derive(Clone, Default)]
pub struct FattyAcidChunked {
    index: Int8Chunked,
    identifier: BoundChunked,
}

impl FattyAcidChunked {
    pub const INDEX: &str = INDEX;
    pub const IDENTIFIER: &str = IDENTIFIER;

    pub fn iter(&self) -> impl Iterator<Item = (Option<Option<NonZeroI8>>, Option<Bound>)> {
        self.index
            .iter()
            .map(|index| index.map(NonZeroI8::new))
            .zip(self.identifier.iter())
    }

    pub fn try_iter(
        &self,
    ) -> impl Iterator<Item = (Option<Option<NonZeroI8>>, PolarsResult<Option<Bound>>)> {
        self.index
            .iter()
            .map(|index| index.map(NonZeroI8::new))
            .zip(self.identifier.try_iter())
    }

    pub fn into_struct(self, name: PlSmallStr) -> PolarsResult<StructChunked> {
        StructChunked::from_series(
            name,
            self.identifier.len(),
            [self.index.into_series(), self.identifier.into_series()].iter(),
        )
    }

    pub fn index(&self) -> &Int8Chunked {
        &self.index
    }

    pub fn bound(&self) -> &BoundChunked {
        &self.identifier
    }

    pub fn sort(&self) -> FattyAcidChunked {
        let options = Default::default();
        // SAFETY: we only reordered the indexes so we are still in bounds.
        let indices = self.identifier.arg_sort(options);
        let mut index = unsafe { self.index.take_unchecked(&indices) };
        let mut identifier = unsafe { self.identifier.physical().take_unchecked(&indices) };
        let indices = index.arg_sort(options);
        index = unsafe { index.take_unchecked(&indices) };
        identifier = unsafe { identifier.take_unchecked(&indices) };
        let identifier = BoundChunked::new(unsafe {
            CategoricalChunked::from_cats_and_rev_map_unchecked(
                identifier,
                self.identifier.get_rev_map().clone(),
                self.identifier.is_enum(),
                CategoricalOrdering::Lexical, // TODO
            )
        });
        Self { index, identifier }
    }
}

// .split_at(offset)
impl FattyAcidChunked {
    pub fn sized(&self) -> (Option<&str>, Cow<Self>) {
        if !self.index.has_nulls() {
            return (None, Cow::Borrowed(&self));
        }
        let null_count = self.index.null_count();
        let index = self
            .index
            .slice(null_count as _, self.index.len() - null_count);
        let physical = self.identifier.physical();
        let r#unsized = physical
            .first()
            .map(|physical| self.identifier.get_rev_map().get(physical));
        let sized = physical.slice(null_count as _, physical.len() - null_count);
        // SAFETY: we created the physical from the enum categories.
        let sized = unsafe {
            CategoricalChunked::from_cats_and_dtype_unchecked(sized, BOUND_DATA_TYPE.clone())
        };
        (
            r#unsized,
            Cow::Owned(Self {
                index,
                identifier: BoundChunked::new(sized),
            }),
        )
    }

    pub fn explicit(&self) -> (BoundChunked, Self) {
        assert!(!self.index.has_nulls());
        let offset = self.implicit_count() as _;
        let (implicit, explicit) = self.identifier.physical().split_at(offset);
        // SAFETY: we created the physical from the enum categories.
        let implicit = unsafe {
            CategoricalChunked::from_cats_and_dtype_unchecked(implicit, BOUND_DATA_TYPE.clone())
        };
        let index = self.index.slice(offset, self.index.len() - offset as usize);
        let explicit = unsafe {
            CategoricalChunked::from_cats_and_dtype_unchecked(explicit, BOUND_DATA_TYPE.clone())
        };
        (
            BoundChunked(implicit),
            Self {
                index,
                identifier: BoundChunked(explicit),
            },
        )
    }
}

// Count.
impl FattyAcidChunked {
    /// Returns the number of sized (Some) indices in the fatty acid.
    pub fn sized_count(&self) -> u8 {
        (self.index.len() - self.index.null_count()) as _
    }

    /// Returns the number of unsized (None) indices in the fatty acid.
    pub fn unsized_count(&self) -> u8 {
        self.index.null_count() as _
    }

    // Returns the number of explicit (NonZeroI8) indices in the fatty acid.
    pub fn explicit_count(&self) -> u8 {
        self.index.not_equal(0).num_trues() as _
    }

    // Returns the number of implicit (Some(0)) indices in the fatty acid.
    pub fn implicit_count(&self) -> u8 {
        self.index.equal(0).num_trues() as _
    }

    pub fn saturated_count(&self) -> u8 {
        (self.index.is_not_null()
            & self
                .identifier
                .equal(S)
                .expect("S is valid bound identifier"))
        .num_trues() as _
    }

    pub fn unsaturated_count(&self) -> u8 {
        (self.index.is_not_null()
            & self
                .identifier
                .not_equal(S)
                .expect("S is valid bound identifier"))
        .num_trues() as _
    }

    /// Returns the total number of unsaturations in the fatty acid.
    pub fn unsaturation_count(&self) -> u8 {
        self.iter()
            .filter_map(|(index, bound)| unsaturated((index, bound))?.1.unsaturation())
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

        // let unsaturated = self
        //     .into_iter()
        //     .filter_map(|(index, bound)| match bound {
        //         Some(Bound::Saturated) => None,
        //         Some(Bound::Unsaturated(unsaturated)) => Some((index, Some(unsaturated))),
        //         None => Some((index, None)),
        //     })
        //     .collect();

        // Ok(format!("carbons:?"))
        // let unsaturated = self
        //     .into_iter()
        //     .filter_map(|(index, bound)| match bound {
        //         Some(Bound::Saturated) => None,
        //         Some(Bound::Unsaturated(unsaturated)) => Some((index, Some(unsaturated))),
        //         None => Some((index, None)),
        //     })
        //     .collect();
        // Ok(match options.kind {
        //     Kind::Delta => FattyAcid::Common(Common::Delta(Delta {
        //         carbons,
        //         unsaturated,
        //         options,
        //     })),
        //     Kind::System => FattyAcid::System(System {
        //         carbons,
        //         unsaturated,
        //     }),
        // })
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
        // polars_ensure!(
        //     *value.dtype() == *FATTY_ACID_DATA_TYPE,
        //     SchemaMismatch: "invalid series data type: expected `{}`, got = `{}`",
        //     *FATTY_ACID_DATA_TYPE,
        //     value.dtype(),
        // );
        // let index = r#struct.field_by_name(Self::INDEX)?.i8()?.clone();
        // let bopund = BoundChunked::try_new(
        //     r#struct
        //         .field_by_name(Self::IDENTIFIER)?
        //         .categorical()?
        //         .clone(),
        // )?;
        // Ok(Self {
        //     index,
        //     bound: bopund,
        // })
    }
}

impl TryFrom<&StructChunked> for FattyAcidChunked {
    type Error = PolarsError;

    fn try_from(value: &StructChunked) -> Result<Self, Self::Error> {
        polars_ensure!(
            *value.dtype() == *FATTY_ACID_DATA_TYPE,
            SchemaMismatch: "invalid fatty acid data type: expected `{}`, got = `{}`",
            *FATTY_ACID_DATA_TYPE,
            value.dtype(),
        );
        let index = value.field_by_name(Self::INDEX)?.i8()?.clone();
        let bound = BoundChunked::try_new(
            value
                .field_by_name(Self::IDENTIFIER)?
                .categorical()?
                .clone(),
        )?;
        Ok(Self {
            index,
            identifier: bound,
        })
    }
}

impl<const N: usize> TryFrom<&[(Option<Option<NonZeroI8>>, Option<&str>); N]> for FattyAcidChunked {
    type Error = PolarsError;

    fn try_from(
        value: &[(Option<Option<NonZeroI8>>, Option<&str>); N],
    ) -> Result<Self, Self::Error> {
        let capacity = value.len();
        let mut indices = PrimitiveChunkedBuilder::<Int8Type>::new(INDEX.into(), capacity);
        let mut identifiers = EnumChunkedBuilder::new(
            IDENTIFIER.into(),
            capacity,
            MAP.clone(),
            Default::default(),
            true,
        );
        for (index, identifier) in value {
            let index = index.map(|index| index.map_or(0, |index| index.into()));
            indices.append_option(index);
            match identifier {
                Some(identifier) => identifiers.append_str(identifier)?,
                None => identifiers.append_null(),
            };
        }
        Ok(Self {
            index: indices.finish(),
            identifier: BoundChunked::new(identifiers.finish()),
        })
    }
}

/// Sized
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Sized;

/// Extension methods for [`Iterator`]
pub trait IndexIteratorExt<T> {
    fn sized(self) -> impl Iterator<Item = (Option<NonZeroI8>, T)>;

    fn r#unsized(self) -> impl Iterator<Item = T>;

    fn explicit(self) -> impl Iterator<Item = (NonZeroI8, T)>;

    fn implicit(self) -> impl Iterator<Item = (Option<Sized>, T)>;
}

impl<I: Iterator<Item = (Option<Option<NonZeroI8>>, T)>, T> IndexIteratorExt<T> for I {
    fn sized(self) -> impl Iterator<Item = (Option<NonZeroI8>, T)> {
        self.filter_map(sized)
    }

    fn r#unsized(self) -> impl Iterator<Item = T> {
        self.filter_map(r#unsized)
    }

    fn explicit(self) -> impl Iterator<Item = (NonZeroI8, T)> {
        self.filter_map(explicit)
    }

    fn implicit(self) -> impl Iterator<Item = (Option<Sized>, T)> {
        self.filter_map(implicit)
    }
}

/// Extension methods for [`Iterator`]
pub trait IdentifierIteratorExt<T> {
    fn is_saturated(&mut self) -> bool;

    fn is_unsaturated(&mut self) -> bool;

    fn saturated(self) -> impl Iterator<Item = T>;

    fn unsaturated(self) -> impl Iterator<Item = (T, Unsaturated)>;

    fn not_saturated(self) -> impl Iterator<Item = (T, Option<Unsaturated>)>;

    fn not_unsaturated(self) -> impl Iterator<Item = (T, Option<Saturated>)>;
}

impl<I: Iterator<Item = (T, Option<Bound>)>, T> IdentifierIteratorExt<T> for I {
    fn is_saturated(&mut self) -> bool {
        self.all(is_saturated)
    }

    fn is_unsaturated(&mut self) -> bool {
        self.any(is_unsaturated)
    }

    fn saturated(self) -> impl Iterator<Item = T> {
        self.filter_map(saturated)
    }

    fn unsaturated(self) -> impl Iterator<Item = (T, Unsaturated)> {
        self.filter_map(unsaturated)
    }

    fn not_saturated(self) -> impl Iterator<Item = (T, Option<Unsaturated>)> {
        self.filter_map(not_saturated)
    }

    fn not_unsaturated(self) -> impl Iterator<Item = (T, Option<Saturated>)> {
        self.filter_map(not_unsaturated)
    }
}

#[inline]
fn sized<T>((index, bound): (Option<Option<NonZeroI8>>, T)) -> Option<(Option<NonZeroI8>, T)> {
    index.map(|index| (index, bound))
}

#[inline]
fn r#unsized<T>((index, bound): (Option<Option<NonZeroI8>>, T)) -> Option<T> {
    if index.is_none() { Some(bound) } else { None }
}

#[inline]
fn explicit<T>((index, bound): (Option<Option<NonZeroI8>>, T)) -> Option<(NonZeroI8, T)> {
    index?.map(|index| (index, bound))
}

#[inline]
fn implicit<T>((index, bound): (Option<Option<NonZeroI8>>, T)) -> Option<(Option<Sized>, T)> {
    match index {
        None => Some((None, bound)),
        Some(None) => Some((Some(Sized), bound)),
        _ => None,
    }
}

#[inline]
fn is_saturated<T>((_index, bound): (T, Option<Bound>)) -> bool {
    bound.is_some_and(Bound::is_saturated)
}

#[inline]
fn is_unsaturated<T>((_index, bound): (T, Option<Bound>)) -> bool {
    bound.is_some_and(Bound::is_unsaturated)
}

#[inline]
fn saturated<T>((index, bound): (T, Option<Bound>)) -> Option<T> {
    bound?.is_saturated().then_some(index)
}

#[inline]
fn unsaturated<T>((index, bound): (T, Option<Bound>)) -> Option<(T, Unsaturated)> {
    Some((index, bound?.as_unsaturated()?))
}

#[inline]
fn not_saturated<T>((index, bound): (T, Option<Bound>)) -> Option<(T, Option<Unsaturated>)> {
    match bound {
        None => Some((index, None)),
        Some(Bound::Saturated) => None,
        Some(Bound::Unsaturated(unsaturated)) => Some((index, Some(unsaturated))),
    }
}

#[inline]
pub fn not_unsaturated<T>((index, bound): (T, Option<Bound>)) -> Option<(T, Option<Saturated>)> {
    match bound {
        None => Some((index, None)),
        Some(Bound::Saturated) => Some((index, Some(Saturated))),
        Some(Bound::Unsaturated(_)) => None,
    }
}

#[cfg(feature = "atomic")]
impl Atomic for &FattyAcidChunked {
    type Output = u8;

    #[inline]
    fn carbons(self) -> u8 {
        self.identifier.carbons()
    }

    #[inline]
    fn hydrogens(self) -> u8 {
        self.identifier.hydrogens()
    }
}

#[cfg(feature = "ecn")]
impl EquivalentCarbonNumber for &FattyAcidChunked {
    type Output = u8;

    #[inline]
    fn equivalent_carbon_number(self) -> u8 {
        self.identifier.equivalent_carbon_number()
    }
}

mod kind;
#[cfg(feature = "mask")]
mod mask;
#[cfg(feature = "mass")]
mod mass;
