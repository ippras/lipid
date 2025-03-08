use super::FattyAcidExpr;
use crate::prelude::*;
use polars::prelude::*;
use polars_ext::series::column;

/// Selectivity and enrichment factor methods for [`FattyAcidExpr`]
impl FattyAcidExpr {
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
    pub fn enrichment_factor(mag2: Expr, tag: Expr) -> Expr {
        mag2 / tag
    }

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
    pub fn selectivity_factor(self, mag2: Expr, tag: Expr) -> Expr {
        as_struct(vec![self.0, mag2, tag]).apply(
            column(selectivity_factor),
            GetOutput::from_type(DataType::Float64),
        )
    }
}
