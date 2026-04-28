use crate::prelude::*;
use polars::prelude::*;

/// Fatty acids biodiesel properties.
pub trait BiodieselProperties {
    /// Cetane number (CN)
    ///
    /// `CN = -0.1209 * DU + 65.0958` ([Wang *et al.*, 2012])
    fn cetane_number(self, expr: Expr) -> Expr;

    /// Cold filter plugging point (CFPP)
    ///
    /// `CFPP = 3.1417 * LCSF - 16.477` ([Ramos *et al.*, 2009])
    fn cold_filter_plugging_point(self, expr: Expr) -> Expr;

    /// Degree of unsaturation (DU)
    ///
    /// `DU = w_{monounsaturated} + 2 * w_{polyunsaturated(2)} + 3 * w_{polyunsaturated(3)} + 4 * w_{polyunsaturated(4)}` ([Wang *et al.*, 2012])
    fn degree_of_unsaturation(self, expr: Expr) -> Expr;

    /// Iodine value (IV)
    ///
    /// `IV = 0.6683 * DU + 25.0364` ([Wang *et al.*, 2012])
    fn iodine_value(self, expr: Expr) -> Expr;

    /// Long Chain Saturated Factor (LCSF)
    ///
    /// `LCSF = 0.1 * w_{C16:0} + 0.5 * w_{C18:0} + 1 * w_{C20:0} + 1.5 * w_{C22:0} + 2 * w_{C24:0}` ([Ramos *et al.*, 2009])
    fn long_chain_saturated_factor(self, expr: Expr) -> Expr;

    ///  Oxidation stability (OS)
    ///   
    /// `OS = -0.0384 * DU + 7.770` ([Wang *et al.*, 2012])
    fn oxidation_stability(self, expr: Expr) -> Expr;
}

impl BiodieselProperties for FattyAcidExpr {
    fn cetane_number(self, expr: Expr) -> Expr {
        self.degree_of_unsaturation(expr) * lit(-0.1209) + lit(0.650958)
    }

    fn cold_filter_plugging_point(self, expr: Expr) -> Expr {
        self.long_chain_saturated_factor(expr) * lit(3.1417) - lit(0.16477)
    }

    fn degree_of_unsaturation(self, expr: Expr) -> Expr {
        self.clone().sum_monounsaturated(expr.clone())
            + self.clone().sum_dienoics(expr.clone()) * lit(2)
            + self.clone().sum_trienoic(expr.clone()) * lit(3)
            + self.sum_tetraenoics(expr) * lit(4)
    }

    fn iodine_value(self, expr: Expr) -> Expr {
        self.degree_of_unsaturation(expr) * lit(0.6683) + lit(0.250364)
    }

    fn long_chain_saturated_factor(self, expr: Expr) -> Expr {
        let c16 = expr.clone().filter(self.clone().equal(C16.clone())).sum();
        let c18 = expr.clone().filter(self.clone().equal(C18.clone())).sum();
        let c20 = expr.clone().filter(self.clone().equal(C20.clone())).sum();
        let c22 = expr.clone().filter(self.clone().equal(C22.clone())).sum();
        let c24 = expr.filter(self.equal(C24.clone())).sum();
        c16 * lit(0.1) + c18 * lit(0.5) + c20 * lit(1) + c22 * lit(1.5) + c24 * lit(2)
    }

    fn oxidation_stability(self, expr: Expr) -> Expr {
        self.degree_of_unsaturation(expr) * lit(-0.0384) + lit(0.07770)
    }
}
