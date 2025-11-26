use crate::prelude::*;
use polars::prelude::*;
use std::num::NonZeroI8;

/// Complex fatty acid indices.
///
/// Nutritional indices.
impl FattyAcidExpr {
    /// Degree of unsaturation (DU)
    ///
    /// `DU = w_{monounsaturated} + 2 * w_{polyunsaturated(2)} + 3 * w_{polyunsaturated(3)} + 4 * w_{polyunsaturated(4)}` ([Wang *et al.*, 2012])
    pub fn degree_of_unsaturation(self, expr: Expr) -> Expr {
        self.clone().monounsaturated(expr.clone())
            + lit(2)
                * expr
                    .clone()
                    .clone()
                    .filter(self.clone().indices().list().len().eq(2))
                    .sum()
            + lit(3)
                * expr
                    .clone()
                    .filter(self.clone().indices().list().len().eq(3))
                    .sum()
            + lit(4) * expr.filter(self.clone().indices().list().len().eq(4)).sum()

        // + lit(3) * self.polyunsaturated(expr)
        // * self.clone().polyunsaturated(expr.clone())
    }

    /// Cetane number (CN)
    ///
    /// $CN = -0.1209 * DU + 65.0958$ ([Wang *et al.*, 2012])
    pub fn cetane_number(self, expr: Expr) -> Expr {
        (lit(-0.1209) * self.degree_of_unsaturation(expr) + lit(0.650958)).alias("CetaneNumber")
    }
}
