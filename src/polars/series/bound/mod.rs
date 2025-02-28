use self::display::{Common, Delta, Display, Kind, Options, System};
use crate::{polars::bound, prelude::*};
use polars::prelude::*;

/// A struct representing a series of bounds.
///
/// This struct wraps a [`Series`] and provides methods to work with bound
/// series.
#[derive(Clone, Debug)]
pub struct BoundSeries(pub(crate) Series);

impl BoundSeries {
    /// Creates a new [`BoundSeries`] from a given [`Series`].
    ///
    /// Ensures that the series has the correct data type.
    ///
    /// # Errors
    ///
    /// Returns an error if the series does not have the expected `Bound` data
    /// type.
    ///
    /// # Arguments
    ///
    /// * `series` - The [`Series`] to wrap in the [`BoundSeries`].
    ///
    /// # Returns
    ///
    /// A [`PolarsResult`] containing the new [`BoundSeries`].
    pub fn new(series: Series) -> PolarsResult<Self> {
        polars_ensure!(
            *series.dtype() == *Bound::DATA_TYPE,
            SchemaMismatch: "invalid bound series datatype: expected `Bound`, got = `{}`",
            series.dtype(),
        );
        Ok(Self(series))
    }

    /// Returns the number of carbons in the series.
    ///
    /// The length of the series plus one.
    ///
    /// # Returns
    ///
    /// The number of carbons as a [`u8`].
    pub fn carbons(&self) -> u8 {
        assert!(self.0.len() < u8::MAX as _);
        self.0.len() as u8 + 1
    }

    /// Returns the number of hydrogens in the series.
    ///
    /// The length of the series plus one.
    ///
    /// # Returns
    ///
    /// The number of hydrogens as a [`u8`].
    pub fn hydrogens(&self) -> u8 {
        // pub fn hydrogens
        // lit(2) * self.clone().carbons() - lit(2) * self.unsaturated().sum()
        // let u = self.unsaturation().unwrap_or_default();
        let u = 0;
        2 * self.carbons() - 2 * u
    }

    // pub fn unsaturation(&self) -> PolarsResult<u8> {
    //     Ok(self
    //         .0
    //         .cast(&Bound::DATA_TYPE)?
    //         .categorical()?
    //         .iter_str()
    //         .filter_map(|id| match Bound::new(id?) {
    //             Bound::Saturated => None,
    //             Bound::Unaturated(unsaturated) => Some(unsaturated.unsaturation?.into()),
    //         })
    //         .sum())
    // }

    /// Returns a [`Display`] representation of the series with the given
    /// options.
    ///
    /// # Arguments
    ///
    /// * `options` - The options to use for the display.
    ///
    /// # Errors
    ///
    /// Returns an error if the series cannot be cast to the `Bound` data type
    /// or if there is an issue with categorical conversion.
    ///
    /// # Returns
    ///
    /// A [`PolarsResult`] containing the [`Display`] representation of the
    /// series.
    pub fn display(&self, options: Options) -> PolarsResult<Display> {
        let carbons = self.carbons();
        let unsaturated = self
            .0
            .cast(&Bound::DATA_TYPE)?
            .categorical()?
            .to_local()
            .iter_str()
            .enumerate()
            .filter_map(|(index, id)| match id {
                Some(id) => match Bound::new(id) {
                    Bound::Saturated => None,
                    Bound::Unaturated(unsaturated) => Some((index + 1, Some(unsaturated))),
                },
                None => Some((index, None)),
            })
            .collect();
        Ok(match options.kind {
            Kind::Delta => Display::Common(Common::Delta(Delta {
                carbons,
                unsaturated,
                options,
            })),
            Kind::System => Display::System(System {
                carbons,
                unsaturated,
            }),
        })
    }
}

/// Checks if the given series of bounds contains only saturated bounds.
///
/// # Arguments
///
/// * `bounds` - A reference to a [`Series`] containing bounds.
///
/// # Returns
///
/// A [`PolarsResult`] containing a boolean indicating whether the bounds
/// contains only saturated bounds.
///
/// [bound::is_saturated]
#[inline]
pub fn is_saturated(bounds: &Series) -> PolarsResult<bool> {
    Ok(bounds.categorical()?.iter_str().all(bound::is_saturated))
}

/// Checks if the given series of bounds contains only saturated bounds or
/// [`None`].
///
/// # Arguments
///
/// * `bounds` - A reference to a [`Series`] containing bounds.
///
/// # Returns
///
/// A [`PolarsResult`] containing a boolean indicating whether the series
/// contains only saturated bounds or [`None`].
///
/// [bound::is_not_unsaturated]
#[inline]
pub fn is_not_unsaturated(bounds: &Series) -> PolarsResult<bool> {
    Ok(bounds
        .categorical()?
        .iter_str()
        .all(bound::is_not_unsaturated))
}

/// Checks if the given series of bounds contains any unsaturated bounds.
///
/// # Arguments
///
/// * `bounds` - A reference to a [`Series`] containing bounds.
///
/// # Returns
///
/// A [`PolarsResult`] containing a boolean indicating whether the series
/// contains any unsaturated bounds.
///
/// [bound::is_unsaturated]
#[inline]
pub fn is_unsaturated(bounds: &Series) -> PolarsResult<bool> {
    Ok(bounds.categorical()?.iter_str().any(bound::is_unsaturated))
}

/// Checks if the given series of bounds contains any unsaturated bounds or
/// [`None`].
///
/// # Arguments
///
/// * `bounds` - A reference to a [`Series`] containing bounds.
///
/// # Returns
///
/// A [`PolarsResult`] containing a boolean indicating whether the series
/// contains any unsaturated bounds or [`None`].
///
/// [bound::is_not_saturated]
#[inline]
pub fn is_not_saturated(bounds: &Series) -> PolarsResult<bool> {
    Ok(bounds
        .categorical()?
        .iter_str()
        .any(bound::is_not_saturated))
}

pub mod display;
pub mod kind;
pub mod mass;
