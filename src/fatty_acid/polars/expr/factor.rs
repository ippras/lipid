use super::FattyAcidExpr;
use polars::prelude::*;

/// Enrichment factor (EF)
///
/// ## [DOI: 10.1007/s11746-014-2553-8](https://10.1007/s11746-014-2553-8)
///
/// The EF is the ratio of the molar concentration of an acyl group in the sn-2
/// position to its concentration in the total TAG.
///
/// ## [DOI: 10.1007/BF02632456](https://doi.org/10.1007/BF02632456)
///
/// The enrichment factor is the ratio of the concentration (molar) of an acid
/// group in the 2-position to its concentration in the total triglyceride.
///
/// This is useful when comparing values for acids competing for the 2-position
/// in the same fat, it is less convenient for discussing the behaviour of acids
/// in several different fats.
pub fn enrichment(mag2: Expr, tag: Expr) -> Expr {
    mag2 / tag
}

/// Selectivity factor methods for [`FattyAcid`]
pub trait Selectivity {
    /// Selectivity factor (SF)
    ///
    /// ## [DOI: 10.1007/s11746-014-2553-8](https://10.1007/s11746-014-2553-8)
    ///
    /// `EF / ([U]_2 / [U]_T)`
    ///
    /// The SF is an EF of a particular FA divided by the EF for all FA which
    /// are preferentially esterified at the sn-2 position.
    ///
    /// ## [DOI: 10.1007/BF02632456](https://doi.org/10.1007/BF02632456)
    ///
    /// The selectivity factor is the enrichment factor of a particular acid
    /// divided by the enrichment factor for all the Category II acids present
    /// in the fat under consideration.
    ///
    /// This is useful for discussing the behaviour of acids in several
    /// different fats.
    fn selectivity(self, mag2: Expr, tag: Expr) -> Expr;
}

impl Selectivity for FattyAcidExpr {
    fn selectivity(self, mag2: Expr, tag: Expr) -> Expr {
        enrichment(mag2.clone(), tag.clone())
            / enrichment(
                mag2.filter(self.clone().is_saturated().not()).sum(),
                tag.filter(self.is_saturated().not()).sum(),
            )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;

    // TAG = 17.2
    // MAG2 = 11.2
    #[test]
    fn test() -> Result<()> {
        let data_frame = df! {
            "FattyAcid" => df! {
                "Carbons" => &[
                    18u8,
                ],
                "Unsaturated" => &[
                    df! {
                        "Index"        => Series::from_iter([9u8]),
                        "Isomerism"    => Series::from_iter([1i8]),
                        "Unsaturation" => Series::from_iter([1u8]),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                ],
            }?.into_struct(PlSmallStr::EMPTY),
            "Value" => [
                Some(1f64),
                Some(2f64),
                Some(3f64),
                Some(4f64),
                Some(5f64),
            ],

        }?;
        Ok(())
    }
}
