use self::unsaturated::UnsaturatedSeries;
use crate::prelude::*;
use polars::prelude::*;
use std::fmt::{self, Formatter};

/// Fatty acid series
#[derive(Clone, Debug)]
pub struct FattyAcidSeries<'a>(&'a Series);

impl<'a> FattyAcidSeries<'a> {
    pub fn new(series: &'a Series) -> PolarsResult<Self> {
        let fatty_acids = series.list()?.get_as_series(0);
        assert_eq!(*fatty_acids.inner_dtype(), *Bound::DATA_TYPE);
        Ok(Self(series))
    }
}

impl FattyAcidSeries<'_> {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn display(&self, index: usize, options: Options) -> PolarsResult<Display> {
        // for unsaturated in &unsaturated_indexed(&bound_series.0)? {
        //     if let Some(unsaturated) = unsaturated {
        //         let index = unsaturated.struct_()?.field_by_name("Index")?.f64()?;
        //         let bound = unsaturated.struct_()?.field_by_name("")?.f64()?;
        //     }
        // }
        match self.0.list()?.get_as_series(index) {
            Some(bounds) => BoundSeries(bounds).display(),
            None => todo!(),
        }
    }

    // pub fn get(&self, index: usize) -> PolarsResult<Option<BoundSeries>> {
    //     let Some(carbons) = self.0.u8()?.get(index) else {
    //         return Ok(None);
    //     };
    //     let mut unsaturated = Vec::new();
    //     if let Some(series) = self.unsaturated.list()?.get_as_series(index) {
    //         let unsaturated_series = UnsaturatedSeries::new(&series)?;
    //         for index in 0..unsaturated_series.len() {
    //             unsaturated.push(unsaturated_series.get(index)?);
    //         }
    //     };
    //     Ok(Some(FattyAcid {
    //         carbons,
    //         unsaturated,
    //     }))
    // }

    /// Return bool chunked array with is unsaturated values
    pub fn is_unsaturated(&self) -> PolarsResult<BooleanChunked> {
        filter(self.0, is_unsaturated)
    }

    pub fn unsaturated(&self) -> PolarsResult<ListChunked> {
        unsaturated_indexed(self.0)
    }

    // pub fn unsaturated(&self, index: usize) -> PolarsResult<Option<UnsaturatedSeries>> {
    //     let Some(unsaturated) = self.unsaturated.list()?.get_as_series(index) else {
    //         return Ok(None);
    //     };
    //     Ok(Some(UnsaturatedSeries::new(&unsaturated)?))
    // }
}

// pub fn is_unsaturated(series: &Series) -> PolarsResult<BooleanChunked> {
//     let mut builder = BooleanChunkedBuilder::new(PlSmallStr::EMPTY, series.len());
//     for bounds in series.list()? {
//         match bounds {
//             Some(bounds) => {
//                 let is_unsaturated = bounds.categorical()?.iter_str().any(|id| id != Some(S));
//                 builder.append_value(is_unsaturated)
//             }
//             None => builder.append_null(),
//         }
//     }
//     Ok(builder.finish())
// }

impl IntoIterator for FattyAcidSeries<'_> {
    type Item = Option<BoundSeries>;

    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0
            .list()
            .unwrap()
            .into_iter()
            .map(|bounds| Some(BoundSeries(bounds?)))
    }
}

// pub fn is_unsaturated(series: &Series) -> PolarsResult<BooleanChunked> {
//     let mut builder = BooleanChunkedBuilder::new(PlSmallStr::EMPTY, series.len());
//     for bounds in series.list()? {
//         match bounds {
//             Some(bounds) => {
//                 let is_unsaturated = bounds.categorical()?.iter_str().any(|id| id != Some(S));
//                 builder.append_value(is_unsaturated)
//             }
//             None => builder.append_null(),
//         }
//     }
//     Ok(builder.finish())
// }

/// Bound series
#[derive(Clone, Debug)]
pub struct BoundSeries(Series);

impl BoundSeries {
    pub fn new(series: Series) -> Self {
        Self(series)
    }

    pub fn carbons(&self) -> u8 {
        assert!(self.0.len() < u8::MAX);
        self.0.len() as u8 + 1
    }

    pub fn display(&self) -> PolarsResult<Display> {
        let carbons = self.carbons();
        let index = self.0.struct_()?.field_by_name("Index")?.u32()?;
        let bound = self
            .0
            .struct_()?
            .field_by_name("")?
            .categorical()?
            .iter_str();
        for (index, bound) in std::iter::zip(index, bound) {}
        Ok(Display::Common(Delta {
            carbons,
            unsaturated,
            options: Default::default(),
        }))
    }
}

/// Display
#[derive(Clone, Debug)]
pub enum Display {
    Common(Delta),
    System(Delta),
}

/// Delta display
#[derive(Clone, Debug)]
pub struct Delta {
    pub carbons: u8,
    pub unsaturated: ListChunked,
    pub options: Options,
}

impl Delta {}

// c18u3dc9dc12dc15

// 18:2Δ9,≡12t
// 18:2Δ9c,≡12t
// 18:2Δ=9c,≡12t
// 18:2Δ=9c,≡12t
impl fmt::Display for Delta {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("C")?;
        fmt::Display::fmt(&self.carbons, f)?;
        f.write_str(":")?;
        fmt::Display::fmt(&self.unsaturated.len(), f)?;
        if f.alternate() {
            if let Some(unsaturated) = self.unsaturated.next() {
                f.write_str("Δ")?;
                fmt::Display::fmt(
                    &unsaturated::Display::new(
                        unsaturated,
                        self.options.notation,
                        self.options.elision,
                    ),
                    f,
                )?;
                for unsaturated in iter {
                    f.write_str(self.options.separators.i[1])?;
                    fmt::Display::fmt(
                        &unsaturated::Display::new(
                            unsaturated,
                            self.options.notation,
                            self.options.elision,
                        ),
                        f,
                    )?;
                }
            }
        }
        Ok(())
    }
}

/// Display options
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Options {
    pub bounds: Elision,
    pub isomerism: Elision,
}

/// Elision
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub enum Elision {
    Explicit,
    #[default]
    Implicit,
}

mod unsaturated;
