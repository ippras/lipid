use super::FattyAcidExpr;
use crate::prelude::SeriesExt;
use polars::prelude::*;
use std::iter::zip;

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

/// See [`enrichment`]
pub fn ef(mag2: Expr, tag: Expr) -> Expr {
    enrichment(mag2, tag)
}

/// Selectivity factor methods for [`FattyAcid`]
pub trait Selectivity: Sized {
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

    /// See [`Self::selectivity`]
    fn sf(self, mag2: Expr, tag: Expr) -> Expr {
        self.selectivity(mag2, tag)
    }
}

impl Selectivity for FattyAcidExpr {
    fn selectivity(self, mag2: Expr, tag: Expr) -> Expr {
        // let mag2_sum = mag2.clone().filter(self.clone().is_saturated().not()).sum();
        // let tag_sum = tag.clone().filter(self.clone().is_saturated().not()).sum();
        // enrichment(mag2.clone(), tag.clone()) / enrichment(mag2_sum, tag_sum)
        as_struct(vec![self.0, mag2, tag]).apply(
            column(selectivity()),
            GetOutput::from_type(DataType::Float64),
        )
    }
}

pub fn column(
    function: impl Fn(&Series) -> PolarsResult<Series>,
) -> impl Fn(Column) -> PolarsResult<Option<Column>> {
    move |column| {
        let Some(series) = column.as_series() else {
            return Ok(None);
        };
        Ok(Some(function(series)?.into_column()))
    }
}

/// Struct
/// 0. FattyAcid
/// 1. MAG2
/// 2. TAG
pub fn selectivity() -> impl Fn(&Series) -> PolarsResult<Series> {
    move |series| {
        let r#struct = series.struct_()?;
        let fields = r#struct.fields_as_series();
        let fatty_acid = fields[0].fatty_acid();
        let mag2 = fields[1].f64()?;
        let tag = fields[2].f64()?;
        let mag2_unsaturated_sum = mag2.filter(&fatty_acid.unsaturated_filter()?)?.sum();
        let tag_unsaturated_sum = tag.filter(&fatty_acid.unsaturated_filter()?)?.sum();
        Ok(zip(tag, mag2)
            .map(|(tag, mag2)| {
                let Some(tag) = tag else {
                    return Ok(None);
                };
                let Some(mag2) = mag2 else {
                    return Ok(None);
                };
                let Some(mag2_unsaturated_sum) = mag2_unsaturated_sum else {
                    return Ok(None);
                };
                let Some(tag_unsaturated_sum) = tag_unsaturated_sum else {
                    return Ok(None);
                };
                Ok(Some(
                    (mag2 / tag) / (mag2_unsaturated_sum / tag_unsaturated_sum),
                ))
            })
            .collect::<PolarsResult<Float64Chunked>>()?
            .into_series())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::fatty_acid::polars::ExprExt as _;
    use anyhow::Result;

    // TAG = 17.2
    // MAG2 = 11.2
    #[test]
    fn test_selectivity() -> Result<()> {
        let mut data_frame = df! {
            "FattyAcid" => df! {
                "Carbons" => &[
                    14u8,
                    16u8,
                    18u8,
                ],
                "Unsaturated" => &[
                    df! {
                        "Index" => Series::new_empty(PlSmallStr::EMPTY, &DataType::UInt8),
                        "Isomerism" => Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8),
                        "Unsaturation" => Series::new_empty(PlSmallStr::EMPTY, &DataType::UInt8),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index" => Series::new_empty(PlSmallStr::EMPTY, &DataType::UInt8),
                        "Isomerism" => Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8),
                        "Unsaturation" => Series::new_empty(PlSmallStr::EMPTY, &DataType::UInt8),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                    df! {
                        "Index" => Series::from_iter([9u8]),
                        "Isomerism" => Series::from_iter([1i8]),
                        "Unsaturation" => Series::from_iter([1u8]),
                    }?.into_struct(PlSmallStr::EMPTY).into_series(),
                ],
            }?.into_struct(PlSmallStr::EMPTY),
            "TAG" => [
                Some(1f64),
                Some(2f64),
                Some(3f64),
            ],
            "MAG2" => [
                Some(4f64),
                Some(5f64),
                Some(6f64),
            ],
        }?;
        println!("data_frame: {}", data_frame);
        data_frame = data_frame
            .lazy()
            .with_columns([
                as_struct(vec![col("FattyAcid"), col("MAG2"), col("TAG")])
                    .apply(
                        column(selectivity()),
                        GetOutput::from_type(DataType::Float64),
                    )
                    .alias("TEMP"),
                enrichment(col("MAG2"), col("TAG")).alias("Enrichment"),
                col("FattyAcid")
                    .fatty_acid()
                    .selectivity(col("MAG2"), col("TAG"))
                    .alias("Selectivity"),
            ])
            .collect()?;
        println!("data_frame: {}", data_frame);
        Ok(())
    }
}
