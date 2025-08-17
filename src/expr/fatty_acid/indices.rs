use crate::prelude::*;
use polars::prelude::*;
use std::num::NonZeroI8;

impl FattyAcidExpr {
    pub fn is_enoics(self, n: u8) -> Expr {
        self.indices().list().len().eq(n)
    }

    pub fn is_monoenoics(self) -> Expr {
        self.is_enoics(1).alias("IsMonoenoics")
    }

    pub fn is_dienoics(self) -> Expr {
        self.is_enoics(2).alias("IsDienoics")
    }

    pub fn is_trienoics(self) -> Expr {
        self.is_enoics(3).alias("IsTrienoics")
    }

    pub fn is_tetraenoics(self) -> Expr {
        self.is_enoics(4).alias("IsTetraenoics")
    }

    pub fn is_pentaenoics(self) -> Expr {
        self.is_enoics(5).alias("IsPentaenoics")
    }

    pub fn is_hexaenoics(self) -> Expr {
        self.is_enoics(6).alias("IsHexaenoics")
    }
}

// SFA, ∑MUFA, ∑PUFA, ∑n-6 PUFA, ∑n-3 PUFA, and n-6 PUFA/n-3 PUFA
// col("FA").fa().ufa(col("Value"))
/// Fatty acid indices
/// ∑SFA, ∑MUFA, ∑PUFA, ∑n-6 PUFA, ∑n-3 PUFA, and n-6 PUFA/n-3 PUFA. The present review may help researchers to evaluate the

/// Simple fatty acid indices.
impl FattyAcidExpr {
    /// Monounsaturated fatty acids (MUFA).
    ///
    /// All unsaturated fatty acids having only one unsaturated bond.
    pub fn monounsaturated(self, expr: Expr) -> Expr {
        expr.filter(self.clone().is_monounsaturated())
            .sum()
            .alias("Monounsaturated")
    }

    /// Polyunsaturated fatty acids (PUFA).
    ///
    /// All unsaturated fatty acids having more than one unsaturated bond.
    pub fn polyunsaturated(self, expr: Expr) -> Expr {
        expr.filter(self.clone().is_polyunsaturated())
            .sum()
            .alias("Polyunsaturated")
    }

    /// Saturated fatty acids (SFA).
    ///
    /// All saturated fatty acids
    pub fn saturated(self, expr: Expr) -> Expr {
        expr.filter(self.is_saturated()).sum().alias("Saturated")
    }

    /// Trans fatty acids (TFA).
    ///
    /// All trans fatty acids.
    pub fn trans(self, expr: Expr) -> Expr {
        expr.filter(self.is_trans()).sum().alias("Trans")
    }

    /// Unsaturated fatty acids (UFA).
    ///
    /// All unsaturated fatty acids
    pub fn unsaturated(self, expr: Expr, offset: Option<NonZeroI8>) -> Expr {
        let name = match offset {
            Some(offset) if offset.is_negative() => format!("Unsaturated{offset}").into(),
            Some(offset) if offset.is_positive() => format!("Unsaturated{offset}").into(),
            _ => PlSmallStr::from_static("Unsaturated"),
        };
        expr.filter(self.is_unsaturated(offset)).sum().alias(name)
    }
}

/// Complex fatty acid indices.
///
/// Nutritional indices.
impl FattyAcidExpr {
    /// Sum of eicosapentaenoic acid and docosahexaenoic acid (EPA + DHA).
    ///
    /// `C22:6(n-3) + C20:5(n-3)`
    pub fn eicosapentaenoic_and_docosahexaenoic(self, expr: Expr) -> Expr {
        let epa = expr.clone().filter(self.clone().eicosapentaenoic()).sum();
        let dha = expr.clone().filter(self.docosahexaenoic()).sum();
        (epa + dha).alias("EicosapentaenoicAndDocosahexaenoic")
    }

    /// Fish lipid quality or flesh lipid quality (FLQ).
    ///
    /// `(C22:6(n-3) + C20:5(n-3)) / ΣFA`
    pub fn fish_lipid_quality(self, expr: Expr) -> Expr {
        let epa = expr.clone().filter(self.clone().eicosapentaenoic()).sum();
        let dha = expr.clone().filter(self.docosahexaenoic()).sum();
        ((epa + dha) / expr.sum()).alias("FishLipidQuality")
    }

    /// [`Self::fish_lipid_quality`]
    pub fn flesh_lipid_quality(self, expr: Expr) -> Expr {
        self.fish_lipid_quality(expr).alias("FleshLipidQuality")
    }

    /// Health-promoting index (HPI).
    ///
    /// `ΣUFA / (C12:0 + 4 * C14:0 + C16:0)`
    pub fn health_promoting_index(self, expr: Expr) -> Expr {
        let c12 = expr.clone().filter(self.clone().equal(C12.clone())).sum();
        let c14 = expr.clone().filter(self.clone().equal(C14.clone())).sum();
        let c16 = expr.clone().filter(self.clone().equal(C16.clone())).sum();
        let ufa = self.unsaturated(expr, None);
        (ufa / (c12 + lit(4) * c14 + c16)).alias("HealthPromotingIndex")
    }

    /// Hypocholesterolemic to hypercholesterolemic ratio (HH).
    ///
    /// `(cis-C18:1 + ΣPUFA) / (C12:0 + C14:0 + C16:0)` TODO:cis-C18:1???
    pub fn hypocholesterolemic_to_hypercholesterolemic(self, expr: Expr) -> Expr {
        let c12 = expr.clone().filter(self.clone().equal(C12.clone())).sum();
        let c14 = expr.clone().filter(self.clone().equal(C14.clone())).sum();
        let c16 = expr.clone().filter(self.clone().equal(C16.clone())).sum();
        let c18dc9 = expr
            .clone()
            .filter(self.clone().equal(C18DC9.clone()))
            .sum();
        let pufa = self.polyunsaturated(expr);
        ((c18dc9 + pufa) / (c12 + c14 + c16)).alias("HypocholesterolemicToHypercholesterolemic")
    }

    /// Index of atherogenicity (IA).
    ///
    /// (C12:0 + 4 * C14:0 + C16:0) / ΣUFA
    pub fn index_of_atherogenicity(self, expr: Expr) -> Expr {
        let c12 = expr.clone().filter(self.clone().equal(C12.clone())).sum();
        let c14 = expr.clone().filter(self.clone().equal(C14.clone())).sum();
        let c16 = expr.clone().filter(self.clone().equal(C16.clone())).sum();
        let ufa = self.unsaturated(expr, None);
        ((c12 + lit(4) * c14 + c16) / ufa).alias("IndexOfAtherogenicity")
    }

    /// Index of thrombogenicity (IT).
    ///
    /// `(C14:0 + C16:0 + C18:0) / [(0.5 * ΣMUFA + 0.5 * ΣPUFA(n-6) + 3 * ΣPUFA(n-3) + ΣUFA(n-3) / ΣUFA(n-6)]`
    pub fn index_of_thrombogenicity(self, expr: Expr) -> Expr {
        let c14 = expr.clone().filter(self.clone().equal(C14.clone())).sum();
        let c16 = expr.clone().filter(self.clone().equal(C16.clone())).sum();
        let c18 = expr.clone().filter(self.clone().equal(C18.clone())).sum();
        let mufa = self.clone().monounsaturated(expr.clone());
        let pufa_3 = expr
            .clone()
            .filter(
                self.clone()
                    .is_polyunsaturated()
                    .and(self.clone().is_unsaturated(NonZeroI8::new(-3))),
            )
            .sum();
        let pufa_6 = expr
            .filter(
                self.clone()
                    .is_polyunsaturated()
                    .and(self.is_unsaturated(NonZeroI8::new(-6))),
            )
            .sum();
        ((c14 + c16 + c18)
            / (lit(0.5) * mufa
                + lit(0.5) * pufa_6.clone()
                + lit(3) * pufa_3.clone()
                + pufa_3 / pufa_6))
            .alias("IndexOfThrombogenicity")
    }

    /// Linoleic fatty acid to α-linolenic fatty acid ratio (LA / ALA).
    ///
    /// `C18:2(n-6) / C18:3(n-3)`
    pub fn linoleic_to_alpha_linolenic(self, expr: Expr) -> Expr {
        let la = expr.clone().filter(self.clone().linoleic()).sum();
        let ala = expr.clone().filter(self.alpha_linolenic()).sum();
        (la / ala).alias("LinoleicToAlphaLinolenic")
    }

    /// Polyunsaturated fatty acids to saturated fatty acids ratio (PUFA / SFA).
    ///
    /// All unsaturated fatty acids having only one unsaturated bond.
    pub fn polyunsaturated_to_saturated(self, expr: Expr) -> Expr {
        let sfa = self.clone().saturated(expr.clone());
        let pufa = self.polyunsaturated(expr);
        (pufa / sfa).alias("PolyunsaturatedToSaturated")
    }

    /// Polyunsaturated (n-6) to polyunsaturated (n-3)
    ///
    /// `PUFA(n-6) / PUFA(n-3)`
    pub fn polyunsaturated_6_to_polyunsaturated_3(self, expr: Expr) -> Expr {
        let pufa_3 = expr
            .clone()
            .filter(
                self.clone()
                    .is_polyunsaturated()
                    .and(self.clone().is_unsaturated(NonZeroI8::new(-3))),
            )
            .sum();
        let pufa_6 = expr
            .filter(
                self.clone()
                    .is_polyunsaturated()
                    .and(self.is_unsaturated(NonZeroI8::new(-6))),
            )
            .sum();
        (pufa_6 / pufa_3).alias("Polyunsaturated-6ToPolyunsaturated-3")
    }

    /// Unsaturation index (UI).
    ///
    /// `1 * (% monoenoics) + 2 * (% dienoics) + 3 * (% trienoics) + 4 * (% tetraenoics) + 5 * (% pentaenoics) + 6 * (% hexaenoics) ...`
    pub fn unsaturation_index(self, expr: Expr) -> Expr {
        (self.unsaturation() * expr)
            .sum()
            .alias("UnsaturationIndex")
    }
}
