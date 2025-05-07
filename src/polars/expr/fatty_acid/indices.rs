use crate::prelude::*;
use polars::prelude::*;
use std::num::NonZeroU8;

impl FattyAcidExpr {
    pub fn enoics(self, n: u8) -> Expr {
        self.0.map(
            move |column| {
                Ok(Some(
                    column
                        .try_fatty_acid_list()?
                        .nullify(&column.try_fatty_acid_list()?.mask(|fatty_acid| {
                            Ok(fatty_acid.iter().unsaturated().count() == n as usize)
                        })?)?
                        .into_list()
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

/// Simple fatty acid indices.
impl FattyAcidExpr {
    /// Saturated fatty acids (SFA).
    ///
    /// All saturated fatty acids
    pub fn saturated(self, expr: Expr) -> Expr {
        expr.filter(self.is_saturated()).sum()
    }

    /// Unsaturated fatty acids (UFA).
    ///
    /// All unsaturated fatty acids
    pub fn unsaturated(self, expr: Expr) -> Expr {
        expr.filter(self.is_unsaturated()).sum()
    }

    /// Monounsaturated fatty acids (MUFA).
    ///
    /// All unsaturated fatty acids having only one unsaturated bond.
    pub fn monounsaturated(self, expr: Expr) -> Expr {
        expr.filter(self.is_monounsaturated()).sum()
    }

    /// Polyunsaturated fatty acids (PUFA).
    ///
    /// All unsaturated fatty acids having more than one unsaturated bond.
    pub fn polyunsaturated(self, expr: Expr) -> Expr {
        expr.filter(self.is_polyunsaturated()).sum()
    }
}

/// Complex fatty acid indices.
///
/// Nutritional indices.
impl FattyAcidExpr {
    /// Polyunsaturated fatty acids to saturated fatty acids ratio (PUFA / SFA).
    ///
    /// All unsaturated fatty acids having only one unsaturated bond.
    pub fn polyunsaturated_to_saturated(self, expr: Expr) -> Expr {
        let saturated = self.clone().saturated(expr.clone());
        let polyunsaturated = self.polyunsaturated(expr);
        polyunsaturated / saturated
    }

    /// Index of atherogenicity (IA).
    ///
    /// (C12:0 + 4 * C14:0 + C16:0) / ΣUFA
    pub fn index_of_atherogenicity(self, expr: Expr) -> Expr {
        let c12u0 = expr.clone().filter(self.clone().equal(&C12));
        let c14u0 = expr.clone().filter(self.clone().equal(&C14));
        let c16u0 = expr.clone().filter(self.clone().equal(&C16));
        let unsaturated = self.unsaturated(expr);
        (c12u0 + lit(4) * c14u0 + c16u0) / unsaturated
    }

    /// Index of thrombogenicity (IT).
    ///
    /// `(C14:0 + C16:0 + C18:0) / [(0.5 * ΣMUFA + 0.5 * ΣPUFA(n-6) + 3 * ΣPUFA(n-3) + ΣUFA(n-3) / ΣUFA(n-6)]`
    pub fn index_of_thrombogenicity(self, expr: Expr) -> Expr {
        let c14u0 = expr.clone().filter(self.clone().equal(&C14));
        let c16u0 = expr.clone().filter(self.clone().equal(&C16));
        let c18u0 = expr.clone().filter(self.clone().equal(&C18));
        let monounsaturated = self.clone().monounsaturated(expr.clone());
        let unsaturated_minus_3 = expr
            .clone()
            .filter(self.clone().is_omega_unsaturated(NonZeroU8::new(3).unwrap()))
            .sum();
        let unsaturated_minus_6 = expr
            .clone()
            .filter(self.clone().is_omega_unsaturated(NonZeroU8::new(6).unwrap()))
            .sum();
        let polyunsaturated_minus_3 = expr
            .clone()
            .filter(
                self.clone()
                    .is_polyunsaturated()
                    .and(self.clone().is_omega_unsaturated(NonZeroU8::new(3).unwrap())),
            )
            .sum();
        let polyunsaturated_minus_6 = expr
            .filter(
                self.clone()
                    .is_polyunsaturated()
                    .and(self.is_omega_unsaturated(NonZeroU8::new(6).unwrap())),
            )
            .sum();
        (c14u0 + c16u0 + c18u0)
            / (lit(0.5) * monounsaturated
                + lit(0.5) * polyunsaturated_minus_6
                + lit(3) * polyunsaturated_minus_3
                + unsaturated_minus_3 / unsaturated_minus_6)
    }

    /// Hypercholesterolemic or hypocholesterolemic ratio (HH).
    ///
    /// `(cis-C18:1 + ΣPUFA) / (C12:0 + C14:0 + C16:0)` TODO:cis-C18:1???
    pub fn hypercholesterolemic_ratio(self, expr: Expr) -> Expr {
        let c12u0 = expr.clone().filter(self.clone().equal(&C12));
        let c14u0 = expr.clone().filter(self.clone().equal(&C14));
        let c16u0 = expr.clone().filter(self.clone().equal(&C16));
        let c18u1 = expr.clone().filter(self.clone().equal(&C18DC9));
        let polyunsaturated = self.polyunsaturated(expr);
        (c18u1 + polyunsaturated) / (c12u0 + c14u0 + c16u0)
    }

    /// [`Self::hypercholesterolemic_ratio`]
    pub fn hypocholesterolemic_ratio(self, expr: Expr) -> Expr {
        self.hypercholesterolemic_ratio(expr)
    }

    /// Health-promoting index (HPI).
    ///
    /// `ΣUFA / (C12:0 + 4 * C14:0 + C16:0)`
    pub fn health_promoting_index(self, expr: Expr) -> Expr {
        let c12u0 = expr.clone().filter(self.clone().equal(&C12));
        let c14u0 = expr.clone().filter(self.clone().equal(&C14));
        let c16u0 = expr.clone().filter(self.clone().equal(&C16));
        let unsaturated = self.unsaturated(expr);
        unsaturated / (c12u0 + lit(4) * c14u0 + c16u0)
    }

    /// Unsaturation index (UI).
    ///
    /// `1 * (% monoenoics) + 2 * (% dienoics) + 3 * (% trienoics) + 4 * (% tetraenoics) + 5 * (% pentaenoics) + 6 * (% hexaenoics) ...`
    pub fn unsaturation_index(self, expr: Expr) -> Expr {
        (self.unsaturation() * expr).sum()
    }

    /// Sum of eicosapentaenoic acid and docosahexaenoic acid (EPA + DHA).
    ///
    /// `C22:6(n-3) + C20:5(n-3)`
    pub fn eicosapentaenoic_and_docosahexaenoic_sum(self, expr: Expr) -> Expr {
        let eicosapentaenoic = expr.clone().filter(self.clone().eicosapentaenoic());
        let docosahexaenoic = expr.clone().filter(self.docosahexaenoic());
        eicosapentaenoic + docosahexaenoic
    }

    /// Fish lipid quality or flesh lipid quality (FLQ).
    ///
    /// `(C22:6(n-3) + C20:5(n-3)) / ΣFA`
    pub fn fish_lipid_quality(self, expr: Expr) -> Expr {
        let eicosapentaenoic = expr.clone().filter(self.clone().eicosapentaenoic());
        let docosahexaenoic = expr.clone().filter(self.docosahexaenoic());
        (eicosapentaenoic + docosahexaenoic) / expr.sum()
    }

    /// [`Self::fish_lipid_quality`]
    pub fn flesh_lipid_quality(self, expr: Expr) -> Expr {
        self.fish_lipid_quality(expr)
    }

    /// Linoleic fatty acid to α-linolenic fatty acid ratio (LA / ALA).
    ///
    /// `C18:2(n-6) / C18:3(n-3)`
    pub fn linoleic_to_alpha_linolenic(self, expr: Expr) -> Expr {
        let linoleic = expr.clone().filter(self.clone().linoleic());
        let alpha_linolenic = expr.clone().filter(self.alpha_linolenic());
        linoleic / alpha_linolenic
    }

    /// Trans fatty acids (TFA).
    ///
    /// `ΣTFA`
    pub fn trans(self, expr: Expr) -> Expr {
        expr.filter(self.is_trans()).sum()
    }
}
