use crate::prelude::*;
use polars::prelude::*;
use std::{num::NonZeroI8, ops::RangeBounds};

impl FattyAcidExpr {
    /// Sum of eicosapentaenoic acid and docosahexaenoic acid (EPA + DHA).
    ///
    /// `C22:6(n-3) + C20:5(n-3)`
    pub fn sum_of_eicosapentaenoic_and_docosahexaenoic(self, expr: Expr) -> Expr {
        let epa = expr
            .clone()
            .filter(self.clone().equal(C20U5C5C8C11C14C17.clone()))
            .sum();
        let dha = expr
            .clone()
            .filter(self.equal(C22U6C4C7C10C13C16C19.clone()))
            .sum();
        epa + dha
    }
}

/// [Sum by chain length](https://ippras.github.io/mathematical_expressions_of_fatty_acids.book/Sum/ByChainLength/index.html)
impl FattyAcidExpr {
    /// [Sum of short chain fatty acids](https://ippras.github.io/mathematical_expressions_of_fatty_acids.book/Sum/ByChainLength/ShortChain.html)
    fn sum_of_short_chain_fatty_acids(self, expr: Expr) -> Expr {
        expr.filter(self.is_short_chain_fatty_acid()).sum()
    }

    /// [Sum of medium chain fatty acids](https://ippras.github.io/mathematical_expressions_of_fatty_acids.book/Sum/ByChainLength/MediumChain.html)
    fn sum_of_medium_chain_fatty_acids(self, expr: Expr) -> Expr {
        expr.filter(self.is_medium_chain_fatty_acid()).sum()
    }

    /// [Sum of long chain fatty acids](https://ippras.github.io/mathematical_expressions_of_fatty_acids.book/Sum/ByChainLength/LongChain.html)
    fn sum_of_long_chain_fatty_acids(self, expr: Expr) -> Expr {
        expr.filter(self.is_long_chain_fatty_acid()).sum()
    }

    /// [Sum of very long chain fatty acids](https://ippras.github.io/mathematical_expressions_of_fatty_acids.book/Sum/ByChainLength/VeryLongChain.html)
    fn sum_of_very_long_chain_fatty_acids(self, expr: Expr) -> Expr {
        expr.filter(self.is_very_long_chain_fatty_acid()).sum()
    }
}

/// [Sum by unsaturated bounds count](https://ippras.github.io/mathematical_expressions_of_fatty_acids.book/Sum/ByUnsaturatedBounds/ByCount/index.html)
impl FattyAcidExpr {
    /// [Sum of saturated fatty acids](https://ippras.github.io/mathematical_expressions_of_fatty_acids.book/Sum/ByUnsaturatedBounds/ByCount/Saturated.html)
    pub fn sum_of_saturated_fatty_acids(self, expr: Expr) -> Expr {
        expr.filter(self.is_saturated_fatty_acid()).sum()
    }

    /// [Sum of monounsaturated fatty acids](https://ippras.github.io/mathematical_expressions_of_fatty_acids.book/Sum/ByUnsaturatedBounds/ByCount/MonoUnsaturated.html)
    pub fn sum_of_monounsaturated_fatty_acids(self, expr: Expr) -> Expr {
        expr.filter(self.is_monounsaturated_fatty_acid()).sum()
    }

    /// [Sum of N unsaturated fatty acids](https://ippras.github.io/mathematical_expressions_of_fatty_acids.book/Sum/ByUnsaturatedBounds/ByCount/MonoUnsaturated.html)
    pub fn sum_of_n_unsaturated_fatty_acids(self, expr: Expr, n: impl RangeBounds<u8>) -> Expr {
        expr.filter(self.is_n_unsaturated_fatty_acid(n)).sum()
    }

    /// [Sum of polyunsaturated fatty acids]()
    pub fn sum_of_polyunsaturated_fatty_acids(self, expr: Expr) -> Expr {
        expr.filter(self.is_polyunsaturated_fatty_acid()).sum()
    }

    /// [Sum of unsaturated fatty acids](https://ippras.github.io/mathematical_expressions_of_fatty_acids.book/Sum/ByUnsaturatedBounds/ByCount/Unsaturated.html)
    pub fn sum_of_unsaturated_fatty_acids(self, expr: Expr) -> Expr {
        expr.filter(self.is_unsaturated_fatty_acid()).sum()
    }
}

/// [Sum by unsaturated bounds offset](https://ippras.github.io/mathematical_expressions_of_fatty_acids.book/Sum/ByUnsaturatedBounds/ByOffset/index.html)
impl FattyAcidExpr {
    fn sum_of_offset_fatty_acids(self, expr: Expr, offset: Option<NonZeroI8>) -> Expr {
        expr.filter(self.is_unsaturated_offset_fatty_acid(offset))
            .sum()
    }

    fn sum_of_delta9_fatty_acids(self, expr: Expr) -> Expr {
        expr.filter(self.is_delta9_fatty_acid()).sum()
    }

    fn sum_of_delta12_fatty_acids(self, expr: Expr) -> Expr {
        expr.filter(self.is_delta12_fatty_acid()).sum()
    }

    fn sum_of_omega9_fatty_acids(self, expr: Expr) -> Expr {
        expr.filter(self.is_omega9_fatty_acid()).sum()
    }

    fn sum_of_omega6_fatty_acids(self, expr: Expr) -> Expr {
        expr.filter(self.is_omega6_fatty_acid()).sum()
    }

    fn sum_of_omega3_fatty_acids(self, expr: Expr) -> Expr {
        expr.filter(self.is_omega3_fatty_acid()).sum()
    }
}

/// [Sum by double bounds parity](https://ippras.github.io/mathematical_expressions_of_fatty_acids.book/Sum/ByUnsaturatedBounds/ByParity/index.html)
impl FattyAcidExpr {
    /// Sum of cis fatty acids
    fn sum_of_cis_fatty_acids(self, expr: Expr) -> Expr {
        expr.filter(self.is_cis_fatty_acid()).sum()
    }

    /// [Sum of trans fatty acids](https://ippras.github.io/mathematical_expressions_of_fatty_acids.book/Sum/ByUnsaturatedBounds/ByParity/Trans.html)
    fn sum_of_trans_fatty_acids(self, expr: Expr) -> Expr {
        expr.filter(self.is_trans_fatty_acid()).sum()
    }
}

/// Nutritional
impl FattyAcidExpr {
    /// [Sum of antiatherogenic fatty acids](https://ippras.github.io/mathematical_expressions_of_fatty_acids.book/Sum/Nutritional/AntiAtherogenic.html)
    pub fn sum_antiatherogenic_fatty_acids(self, expr: Expr) -> Expr {
        let mufa = self.clone().sum_monounsaturated(expr.clone());
        let pufa_o6 = expr
            .clone()
            .filter(
                self.clone()
                    .is_polyunsaturated()
                    .and(self.clone().is_unsaturated(NonZeroI8::new(-6))),
            )
            .sum();
        let pufa_o3 = expr
            .filter(
                self.clone()
                    .is_polyunsaturated()
                    .and(self.is_unsaturated(NonZeroI8::new(-3))),
            )
            .sum();
        mufa + pufa_o6 + pufa_o3
    }

    /// [Sum of anticholesterolemic fatty acids](https://ippras.github.io/mathematical_expressions_of_fatty_acids.book/Sum/Nutritional/AntiCholesterolemic.html)
    pub fn sum_anticholesterolemic_fatty_acids(self, expr: Expr) -> Expr {
        let c18u1c9 = expr
            .clone()
            .filter(self.clone().equal(C18U1C9.clone()))
            .sum();
        let pufa = self.sum_polyunsaturated(expr);
        c18u1c9 + pufa
    }

    /// [Sum of antithrombogenic fatty acids](https://ippras.github.io/mathematical_expressions_of_fatty_acids.book/Sum/Nutritional/AntiThrombogenic.html)
    pub fn sum_antithrombogenic_fatty_acids(self, expr: Expr, weighted: bool) -> Expr {
        let mufa = self.clone().sum_monounsaturated(expr.clone());
        let pufa_o6 = expr
            .clone()
            .filter(
                self.clone()
                    .is_polyunsaturated()
                    .and(self.clone().is_unsaturated(NonZeroI8::new(-6))),
            )
            .sum();
        let pufa_o3 = expr
            .filter(
                self.clone()
                    .is_polyunsaturated()
                    .and(self.is_unsaturated(NonZeroI8::new(-3))),
            )
            .sum();
        if weighted {
            lit(0.5) * mufa
                + lit(0.5) * pufa_o6.clone()
                + lit(3) * pufa_o3.clone()
                + pufa_o3 / pufa_o6
        } else {
            mufa + pufa_o6 + pufa_o3
        }
    }

    /// [Sum of proatherogenic fatty acids](https://ippras.github.io/mathematical_expressions_of_fatty_acids.book/Sum/Nutritional/ProAtherogenic.html)
    pub fn sum_proatherogenic_fatty_acids(self, expr: Expr, weighted: bool) -> Expr {
        let c12u0 = expr.clone().filter(self.clone().equal(C12U0.clone())).sum();
        let c14u0 = expr.clone().filter(self.clone().equal(C14U0.clone())).sum();
        let c16u0 = expr.clone().filter(self.clone().equal(C16U0.clone())).sum();
        if weighted {
            c12u0 + lit(4) * c14u0 + c16u0
        } else {
            c12u0 + c14u0 + c16u0
        }
    }

    /// [Sum of procholesterolemic fatty acids](https://ippras.github.io/mathematical_expressions_of_fatty_acids.book/Sum/Nutritional/ProCholesterolemic.html)
    pub fn sum_procholesterolemic_fatty_acids(self, expr: Expr) -> Expr {
        let c12u0 = expr.clone().filter(self.clone().equal(C12U0.clone())).sum();
        let c14u0 = expr.clone().filter(self.clone().equal(C14U0.clone())).sum();
        let c16u0 = expr.clone().filter(self.clone().equal(C16U0.clone())).sum();
        c12u0 + c14u0 + c16u0
    }

    /// [Sum of prothrombogenic fatty acids](https://ippras.github.io/mathematical_expressions_of_fatty_acids.book/Sum/Nutritional/ProThrombogenic.html)
    pub fn sum_prothrombogenic_fatty_acids(self, expr: Expr) -> Expr {
        let c14u0 = expr.clone().filter(self.clone().equal(C14U0.clone())).sum();
        let c16u0 = expr.clone().filter(self.clone().equal(C16U0.clone())).sum();
        let c18u0 = expr.clone().filter(self.clone().equal(C18U0.clone())).sum();
        c14u0 + c16u0 + c18u0
    }
}

impl FattyAcidSumByBounds for FattyAcidExpr {
    fn sum_conjugated(self, expr: Self::Output) -> Self::Output {
        expr.filter(self.is_conjugated(true)).sum()
    }

    fn sum_monounsaturated(self, expr: Self::Output) -> Self::Output {
        expr.filter(self.is_monounsaturated()).sum()
    }

    fn sum_polyunsaturated(self, expr: Self::Output) -> Self::Output {
        expr.filter(self.is_polyunsaturated()).sum()
    }

    fn sum_saturated(self, expr: Self::Output) -> Self::Output {
        expr.filter(self.is_saturated()).sum()
    }

    fn sum_trans(self, expr: Self::Output) -> Self::Output {
        expr.filter(self.is_trans_fatty_acid()).sum()
    }

    fn sum_unsaturated(self, expr: Expr, offset: Option<NonZeroI8>) -> Expr {
        expr.filter(self.is_unsaturated(offset)).sum()
    }
}

impl FattyAcidSumByDoubleBounds for FattyAcidExpr {
    fn sum_dienoics(self, expr: Expr) -> Expr {
        expr.filter(self.clone().is_dienoic()).sum()
    }

    fn sum_hexaenoics(self, expr: Expr) -> Expr {
        expr.filter(self.clone().is_hexaenoic()).sum()
    }

    fn sum_monoenoics(self, expr: Expr) -> Expr {
        expr.filter(self.clone().is_monoenoic()).sum()
    }

    fn sum_pentaenoics(self, expr: Expr) -> Expr {
        expr.filter(self.clone().is_pentaenoic()).sum()
    }

    fn sum_tetraenoics(self, expr: Expr) -> Expr {
        expr.filter(self.clone().is_tetraenoic()).sum()
    }

    fn sum_trienoic(self, expr: Expr) -> Expr {
        expr.filter(self.clone().is_trienoic()).sum()
    }
}
