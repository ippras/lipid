use crate::prelude::*;
use polars::prelude::*;

impl FattyAcidExpr {
    pub fn enoics(self, n: u8) -> Expr {
        self.0.map(
            move |column| {
                Ok(Some(
                    column
                        .fatty_acid()?
                        .nullify(
                            &column
                                .fatty_acid()?
                                .mask(|bounds| bounds.unsaturated().count() == n as usize)?,
                        )?
                        .into_column(),
                ))
            },
            GetOutput::same_type(),
        )
    }

    pub fn monoenoics(self) -> Expr {
        self.enoics(1)
    }

    pub fn dienoics(self) -> Expr {
        self.enoics(2)
    }

    pub fn trienoics(self) -> Expr {
        self.enoics(3)
    }

    pub fn tetraenoics(self) -> Expr {
        self.enoics(4)
    }

    pub fn pentaenoics(self) -> Expr {
        self.enoics(5)
    }

    pub fn hexaenoics(self) -> Expr {
        self.enoics(6)
    }
}

// SFA, ∑MUFA, ∑PUFA, ∑n-6 PUFA, ∑n-3 PUFA, and n-6 PUFA/n-3 PUFA
// col("FA").fa().ufa(col("Value"))
/// Fatty acid indices
/// ∑SFA, ∑MUFA, ∑PUFA, ∑n-6 PUFA, ∑n-3 PUFA, and n-6 PUFA/n-3 PUFA. The present review may help researchers to evaluate the

// /// PUFA n-3, n-6, ...
// fn pufan(self, n: u8) -> Expr {
//     self.clone().unsaturated(true).len().list().eval(
//         col("")
//             .struct_()
//             .field_by_name("Index")
//             .eq(self.carbons() - lit(n)),
//         true,
//     )
// }

/// Complex fatty acid indices.
///
/// Nutritional indices.
impl FattyAcidExpr {
    /// Polyunsaturated fatty acid/saturated fatty acid ratio (PUFA/SFA).
    ///
    /// All unsaturated fatty acids having only one unsaturated bond.
    pub fn polyunsaturated_to_saturated(self, expr: Expr) -> Expr {
        let polyunsaturated = expr.clone().filter(self.clone().is_polyunsaturated()).sum();
        let saturated = expr.filter(self.is_saturated()).sum();
        polyunsaturated / saturated
    }

    /// Index of atherogenicity (IA).
    ///
    /// (C12:0 + 4 * C14:0 + C16:0) / ΣUFA
    pub fn index_of_atherogenicity(self, expr: Expr) -> Expr {
        let c12u0 = expr.clone().filter(self.clone().equal(C12U0)).sum();
        let c14u0 = expr.clone().filter(self.clone().equal(C14U0)).sum();
        let c16u0 = expr.clone().filter(self.clone().equal(C16U0)).sum();
        let unsaturated = expr.filter(self.is_unsaturated()).sum();
        (c12u0 + lit(4) * c14u0 + c16u0) / unsaturated
    }

    /// Index of thrombogenicity (IT)
    ///
    /// (C14:0 + C16:0 + C18:0)/(0.5 * ΣMUFA + 0.5 * ΣPUFAN6 + 3 * ΣPUFAN3 + (n-3/n-6)]
    pub fn index_of_thrombogenicity(self, expr: Expr) -> Expr {
        let c14u0 = expr.clone().filter(self.clone().equal(C14U0)).sum();
        let c16u0 = expr.clone().filter(self.clone().equal(C16U0)).sum();
        let c18u0 = expr.clone().filter(self.clone().equal(C18U0)).sum();
        let monounsaturated = expr.filter(self.is_monounsaturated()).sum();
        (c14u0 + c16u0 + c18u0) / (lit(0.5) * monounsaturated + lit(0.5))
    }

    /// Hypercholesterolemic/hypocholesterolemic ratio (HH)
    ///
    /// (cis-C18:1 + ΣPUFA)/(C12:0 + C14:0 + C16:0)
    pub fn hypercholesterolemic_ratio(self, expr: Expr) -> Expr {
        let c18u1 = expr.clone().filter(self.clone().equal(C18U1DC9));
        let polyunsaturated = expr.clone().filter(self.clone().is_polyunsaturated());
        let c12u0 = expr.clone().filter(self.clone().equal(C12U0));
        let c14u0 = expr.clone().filter(self.clone().equal(C14U0));
        let c16u0 = expr.filter(self.equal(C16U0));
        (c18u1 + polyunsaturated) / (c12u0 + c14u0 + c16u0)
    }

    /// [`Self::hypercholesterolemic_ratio`]
    pub fn hypocholesterolemic_ratio(self, expr: Expr) -> Expr {
        self.hypercholesterolemic_ratio(expr)
    }

    /// HPI (Health-promoting index)
    ///
    /// ΣUFA / (C12:0 + 4 * C14:0 + C16:0)
    pub fn health_promoting_index(self, expr: Expr) -> Expr {
        let unsaturated = expr.clone().filter(self.clone().is_unsaturated());
        let c12u0 = expr.clone().filter(self.clone().equal(C12U0));
        let c14u0 = expr.clone().filter(self.clone().equal(C14U0));
        let c16u0 = expr.filter(self.equal(C16U0));
        unsaturated / (c12u0 + lit(4) * c14u0 + c16u0)
    }

    /// Unsaturation index (UI)
    ///
    /// UI = 1 * (% monoenoics) + 2 * (% dienoics) + 3 * (% trienoics) + 4 * (%
    /// tetraenoics) +5 * (% pentaenoics) + 6 * (% hexaenoics)
    pub fn unsaturation_index(self, expr: Expr) -> Expr {
        (self.unsaturation() * expr).sum()
    }

    /// Fish lipid quality/flesh lipid quality (FLQ)
    ///
    /// (EPA + DHA) / ΣFA
    pub fn fish_lipid_quality(self, expr: Expr) -> Expr {
        let eicosapentaenoic = expr.clone().filter(self.clone().eicosapentaenoic());
        let docosahexaenoic = expr.clone().filter(self.docosahexaenoic());
        (eicosapentaenoic + docosahexaenoic) / expr.sum()
    }

    /// [`Self::fish_lipid_quality`]
    pub fn flesh_lipid_quality(self, expr: Expr) -> Expr {
        self.fish_lipid_quality(expr)
    }

    /// Trans fatty acids (TFA)
    pub fn trans(self, expr: Expr) -> Expr {
        expr.filter(self.is_trans()).sum()
    }
}
