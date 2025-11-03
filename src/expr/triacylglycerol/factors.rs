use super::TriacylglycerolExpr;
use crate::r#trait::RelativeAtomicMass as _;
use atom::prelude::isotopes::*;
use polars::prelude::*;

const H: f64 = H::One.relative_atomic_mass().value;
const I: f64 = I::OneHundredTwentySeven.relative_atomic_mass().value;

impl TriacylglycerolExpr {
    pub fn iodine_value(self, mut expr: Expr, sum: bool) -> Expr {
        expr = expr * self.clone().unsaturation() * lit(I * 2.0)
            / (self.relative_atomic_mass(Some(-lit(3.0 * H))));
        if sum {
            expr = expr.sum()
        }
        expr
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn iodine_value() -> PolarsResult<()> {
        let data_frame = df!(
            TRIACYLGLYCEROL => df!(
                STEREOSPECIFIC_NUMBERS1 => [
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
                STEREOSPECIFIC_NUMBERS2 => [
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
                STEREOSPECIFIC_NUMBERS3 => [
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
            )?.into_struct(TRIACYLGLYCEROL.into()),
        )?;
        let mut lazy_frame = data_frame.lazy();
        lazy_frame = lazy_frame.select([
            col(TRIACYLGLYCEROL)
                .triacylglycerol()
                .relative_atomic_mass(None),
            col(TRIACYLGLYCEROL)
                .triacylglycerol()
                .iodine_value(lit(1), false)
                .alias("IV"),
        ]);
        println!("lazy_frame: {}", lazy_frame.collect()?);
        Ok(())
    }
}
