use super::FattyAcidExpr;
use polars::prelude::*;

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
        mag2 / tag
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
        let mag2_unsaturated = mag2.clone().filter(self.clone().is_unsaturated()).sum();
        let tag_unsaturated = tag.clone().filter(self.is_unsaturated()).sum();
        (mag2 / tag) / (mag2_unsaturated / tag_unsaturated)
        // as_struct(vec![self.0, mag2, tag]).apply(
        //     |column| {
        //         let r#struct = column.struct_()?;
        //         let fields = r#struct.fields_as_series();
        //         let fatty_acid = fields[0].fatty_acid()?;
        //         let mag2 = fields[1].f64()?;
        //         let tag = fields[2].f64()?;
        //         let mag2_unsaturated_sum = mag2.filter(&fatty_acid.is_unsaturated()?)?.sum();
        //         let tag_unsaturated_sum = tag.filter(&fatty_acid.is_unsaturated()?)?.sum();
        //         Ok(Some(
        //             zip(tag, mag2)
        //                 .map(|(tag, mag2)| {
        //                     let Some(tag) = tag else {
        //                         return Ok(None);
        //                     };
        //                     let Some(mag2) = mag2 else {
        //                         return Ok(None);
        //                     };
        //                     let Some(mag2_unsaturated_sum) = mag2_unsaturated_sum else {
        //                         return Ok(None);
        //                     };
        //                     let Some(tag_unsaturated_sum) = tag_unsaturated_sum else {
        //                         return Ok(None);
        //                     };
        //                     Ok(Some(
        //                         (mag2 / tag) / (mag2_unsaturated_sum / tag_unsaturated_sum),
        //                     ))
        //                 })
        //                 .collect::<PolarsResult<Float64Chunked>>()?
        //                 .into_column(),
        //         ))
        //     },
        //     GetOutput::from_type(DataType::Float64),
        // )
    }
}
