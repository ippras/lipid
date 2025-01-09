use super::FattyAcidExpr;
use polars::prelude::*;

/// Enrichment factor (EF)
///
/// The enrichment factor (EF) is used to determine the degree of
/// contamination of heavy metals in water. According to Springer, the EF is
/// calculated by dividing the concentration of a heavy metal in water by
/// the concentration of that metal in a reference sample.
pub fn enrichment(tag: Expr, expr: Expr) -> Expr {
    expr / tag
}

/// Selectivity factor methods for [`FattyAcid`]
pub trait Selectivity {
    /// Selectivity factor (SF)
    fn selectivity(self, tag: Expr, expr: Expr) -> Expr;
}

impl Selectivity for FattyAcidExpr {
    fn selectivity(self, tag: Expr, expr: Expr) -> Expr {
        expr.clone() * self.clone().filter_saturated(tag).sum() / self.filter_saturated(expr).sum()
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
