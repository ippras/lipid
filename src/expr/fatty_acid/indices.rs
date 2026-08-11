use crate::prelude::*;
use polars::prelude::*;
use std::num::NonZeroI8;

// SFA, ∑MUFA, ∑PUFA, ∑n-6 PUFA, ∑n-3 PUFA, and n-6 PUFA/n-3 PUFA
// col("FA").fa().ufa(col("Value"))
// ∑SFA, ∑MUFA, ∑PUFA, ∑n-6 PUFA, ∑n-3 PUFA, and n-6 PUFA/n-3 PUFA. The present review may help researchers to evaluate the

/// Sum
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

/// Nutritional sum
impl FattyAcidExpr {
    /// [Sum of antiatherogenic fatty acids](https://ippras.github.io/mathematical_expressions_of_fatty_acids.book/Sum/Nutritional/AntiAtherogenic.html)
    pub fn sum_of_antiatherogenic_fatty_acids(self, expr: Expr) -> Expr {
        let mufa = self.clone().sum_monounsaturated(expr.clone());
        let pufa_o6 = expr
            .filter(
                self.clone()
                    .is_polyunsaturated()
                    .and(self.is_unsaturated(NonZeroI8::new(-6))),
            )
            .sum();
        let pufa_o3 = expr
            .clone()
            .filter(
                self.clone()
                    .is_polyunsaturated()
                    .and(self.clone().is_unsaturated(NonZeroI8::new(-3))),
            )
            .sum();
        mufa + pufa_o6 + pufa_o3
    }

    /// [Sum of anticholesterolemic fatty acids](https://ippras.github.io/mathematical_expressions_of_fatty_acids.book/Sum/Nutritional/AntiCholesterolemic.html)
    pub fn sum_of_anticholesterolemic_fatty_acids(self, expr: Expr) -> Expr {
        let c18u1c9 = expr
            .clone()
            .filter(self.clone().equal(C18U1C9.clone()))
            .sum();
        let pufa = self.sum_polyunsaturated(expr);
        c18u1c9 + pufa
    }

    /// [Sum of antithrombogenic fatty acids](https://ippras.github.io/mathematical_expressions_of_fatty_acids.book/Sum/Nutritional/AntiThrombogenic.html)
    pub fn sum_of_antithrombogenic_fatty_acids(self, expr: Expr, weighted: bool) -> Expr {
        let mufa = self.clone().sum_monounsaturated(expr.clone());
        let pufa_o6 = expr
            .filter(
                self.clone()
                    .is_polyunsaturated()
                    .and(self.is_unsaturated(NonZeroI8::new(-6))),
            )
            .sum();
        let pufa_o3 = expr
            .clone()
            .filter(
                self.clone()
                    .is_polyunsaturated()
                    .and(self.clone().is_unsaturated(NonZeroI8::new(-3))),
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
    pub fn sum_of_proatherogenic_fatty_acids(self, expr: Expr, weighted: bool) -> Expr {
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
    pub fn sum_of_procholesterolemic_fatty_acids(self, expr: Expr) -> Expr {
        let c12u0 = expr.clone().filter(self.clone().equal(C12U0.clone())).sum();
        let c14u0 = expr.clone().filter(self.clone().equal(C14U0.clone())).sum();
        let c16u0 = expr.clone().filter(self.clone().equal(C16U0.clone())).sum();
        c12u0 + c14u0 + c16u0
    }

    /// [Sum of prothrombogenic fatty acids](https://ippras.github.io/mathematical_expressions_of_fatty_acids.book/Sum/Nutritional/ProThrombogenic.html)
    pub fn sum_of_prothrombogenic_fatty_acids(self, expr: Expr) -> Expr {
        let c14u0 = expr.clone().filter(self.clone().equal(C14U0.clone())).sum();
        let c16u0 = expr.clone().filter(self.clone().equal(C16U0.clone())).sum();
        let c18u0 = expr.clone().filter(self.clone().equal(C18U0.clone())).sum();
        c14u0 + c16u0 + c18u0
    }
}

/// Complex fatty acid indices.
///
/// Nutritional indices.
impl FattyAcidExpr {
    /// Fish lipid quality or flesh lipid quality (FLQ).
    ///
    /// `(C22:6(n-3) + C20:5(n-3)) / ΣFA`
    pub fn fish_lipid_quality(self, expr: Expr) -> Expr {
        let epa = expr
            .clone()
            .filter(self.clone().equal(C20U5C5C8C11C14C17.clone()))
            .sum();
        let dha = expr
            .clone()
            .filter(self.equal(C22U6C4C7C10C13C16C19.clone()))
            .sum();
        (epa + dha) / expr.sum()
    }

    /// [`Self::fish_lipid_quality`]
    pub fn flesh_lipid_quality(self, expr: Expr) -> Expr {
        self.fish_lipid_quality(expr)
    }

    /// Health-promoting index (HPI).
    ///
    /// `ΣUFA / (C12:0 + 4 * C14:0 + C16:0)`
    pub fn health_promoting_index(self, expr: Expr) -> Expr {
        let c12u0 = expr.clone().filter(self.clone().equal(C12U0.clone())).sum();
        let c14u0 = expr.clone().filter(self.clone().equal(C14U0.clone())).sum();
        let c16u0 = expr.clone().filter(self.clone().equal(C16U0.clone())).sum();
        let ufa = self.sum_unsaturated(expr, None);
        ufa / (c12u0 + lit(4) * c14u0 + c16u0)
    }

    /// Hypocholesterolemic to hypercholesterolemic ratio (HH).
    ///
    /// `(cis-C18:1 + ΣPUFA) / (C12:0 + C14:0 + C16:0)` TODO:cis-C18:1???
    pub fn hypocholesterolemic_to_hypercholesterolemic(self, expr: Expr) -> Expr {
        let c12u0 = expr.clone().filter(self.clone().equal(C12U0.clone())).sum();
        let c14u0 = expr.clone().filter(self.clone().equal(C14U0.clone())).sum();
        let c16u0 = expr.clone().filter(self.clone().equal(C16U0.clone())).sum();
        let c18u1c9 = expr
            .clone()
            .filter(self.clone().equal(C18U1C9.clone()))
            .sum();
        let pufa = self.sum_polyunsaturated(expr);
        (c18u1c9 + pufa) / (c12u0 + c14u0 + c16u0)
    }

    /// Index of atherogenicity (IA).
    ///
    /// (C12:0 + 4 * C14:0 + C16:0) / ΣUFA
    pub fn index_of_atherogenicity(self, expr: Expr) -> Expr {
        let c12u0 = expr.clone().filter(self.clone().equal(C12U0.clone())).sum();
        let c14u0 = expr.clone().filter(self.clone().equal(C14U0.clone())).sum();
        let c16u0 = expr.clone().filter(self.clone().equal(C16U0.clone())).sum();
        let ufa = self.sum_unsaturated(expr, None);
        (c12u0 + lit(4) * c14u0 + c16u0) / ufa
    }

    /// Index of thrombogenicity (IT).
    ///
    /// `(C14:0 + C16:0 + C18:0) / [(0.5 * ΣMUFA + 0.5 * ΣPUFA(n-6) + 3 * ΣPUFA(n-3) + ΣUFA(n-3) / ΣUFA(n-6)]`
    pub fn index_of_thrombogenicity(self, expr: Expr) -> Expr {
        let c14u0 = expr.clone().filter(self.clone().equal(C14U0.clone())).sum();
        let c16u0 = expr.clone().filter(self.clone().equal(C16U0.clone())).sum();
        let c18u0 = expr.clone().filter(self.clone().equal(C18U0.clone())).sum();
        let mufa = self.clone().sum_monounsaturated(expr.clone());
        let pufa_o3 = expr
            .clone()
            .filter(
                self.clone()
                    .is_polyunsaturated()
                    .and(self.clone().is_unsaturated(NonZeroI8::new(-3))),
            )
            .sum();
        let pufa_o6 = expr
            .filter(
                self.clone()
                    .is_polyunsaturated()
                    .and(self.is_unsaturated(NonZeroI8::new(-6))),
            )
            .sum();
        (c14u0 + c16u0 + c18u0)
            / (lit(0.5) * mufa
                + lit(0.5) * pufa_o6.clone()
                + lit(3) * pufa_o3.clone()
                + pufa_o3 / pufa_o6)
    }

    /// Linoleic fatty acid to α-linolenic fatty acid ratio (LA / ALA).
    ///
    /// `C18:2(n-6) / C18:3(n-3)`
    pub fn linoleic_to_alpha_linolenic(self, expr: Expr) -> Expr {
        let la = expr
            .clone()
            .filter(self.clone().equal(C18U2C9C12.clone()))
            .sum();
        let ala = expr.clone().filter(self.equal(C18U3C9C12C15.clone())).sum();
        la / ala
    }

    /// Polyunsaturated fatty acids to saturated fatty acids ratio (PUFA / SFA).
    ///
    /// All unsaturated fatty acids having only one unsaturated bond.
    pub fn polyunsaturated_to_saturated(self, expr: Expr) -> Expr {
        let sfa = self.clone().sum_saturated(expr.clone());
        let pufa = self.sum_polyunsaturated(expr);
        pufa / sfa
    }

    /// Polyunsaturated (n-6) to polyunsaturated (n-3)
    ///
    /// `PUFA(n-6) / PUFA(n-3)`
    pub fn polyunsaturated_6_to_polyunsaturated_3(self, expr: Expr) -> Expr {
        let pufa_o3 = expr
            .clone()
            .filter(
                self.clone()
                    .is_polyunsaturated()
                    .and(self.clone().is_unsaturated(NonZeroI8::new(-3))),
            )
            .sum();
        let pufa_o6 = expr
            .filter(
                self.clone()
                    .is_polyunsaturated()
                    .and(self.is_unsaturated(NonZeroI8::new(-6))),
            )
            .sum();
        pufa_o6 / pufa_o3
    }

    /// Unsaturation index (UI).
    ///
    /// `1 * (% monoenoics) + 2 * (% dienoics) + 3 * (% trienoics) + 4 * (% tetraenoics) + 5 * (% pentaenoics) + 6 * (% hexaenoics) ...`
    pub fn unsaturation_index(self, expr: Expr) -> Expr {
        (expr * self.unsaturation()).sum()
    }
}
