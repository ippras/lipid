use super::FattyAcidExpr;
use crate::prelude::*;
use atom::prelude::isotopes::*;
use polars::prelude::*;

const I: f64 = I::OneHundredTwentySeven.relative_atomic_mass().value;

impl FattyAcidExpr {
    pub fn iodine_value(self, mut expr: Expr, sum: bool) -> Expr {
        expr = expr * self.clone().unsaturation() * lit(I * 2.0) / self.relative_atomic_mass(None);
        if sum {
            expr = expr.sum()
        }
        expr
    }
}

/// Selectivity and enrichment factor methods for [`FattyAcidExpr`]
impl FattyAcidExpr {
    /// Enrichment factor (EF).
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
        mag2 / (lit(3) * tag)
    }

    /// Selectivity factor (SF).
    ///
    /// ## [DOI: 10.1007/s11746-014-2553-8](https://10.1007/s11746-014-2553-8)
    ///
    /// `([A]_2 / [A]_{123}) / ([U]_2 / [U]_{123})`
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
        let unsaturated_mag2 = mag2.clone().filter(self.clone().is_unsaturated(None)).sum();
        let unsaturated_tag = tag.clone().filter(self.is_unsaturated(None)).sum();
        // (mag2 / tag) / (unsaturated_mag2 / unsaturated_tag)
        (mag2 * unsaturated_tag) / (tag * unsaturated_mag2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iodine_value() -> PolarsResult<()> {
        let data_frame = df!(
            FATTY_ACID => [
                // fatty_acid!(C12 {3 => C})?,
                // fatty_acid!(C14 {5 => C})?,
                // fatty_acid!(C15 {6 => T})?,
                // fatty_acid!(C15 {6 => C})?,
                // fatty_acid!(C16 {7 => T})?,
                // fatty_acid!(C16 {7 => C})?,
                // fatty_acid!(C17 {8 => T})?,
                // fatty_acid!(C17 {8 => C})?,
                fatty_acid!(C18 {9 => T})?,
                fatty_acid!(C18 {9 => C})?,
                fatty_acid!(C18 {9 => T, 12 => T})?,
                fatty_acid!(C18 {9 => C, 12 => C})?,
                fatty_acid!(C18 {9 => T, 12 => T, 15 => T})?,
                fatty_acid!(C18 {9 => C, 12 => C, 15 => C})?,
                fatty_acid!(C20 {11 => C})?,
            ],
        )?;
        let mut lazy_frame = data_frame.lazy();
        lazy_frame = lazy_frame.select([
            col(FATTY_ACID).fatty_acid().relative_atomic_mass(None),
            col(FATTY_ACID)
                .fatty_acid()
                .iodine_value(lit(1), false)
                .alias("IV"),
        ]);
        println!("lazy_frame: {}", lazy_frame.collect()?);
        Ok(())
    }
}
