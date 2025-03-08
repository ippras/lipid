// use super::{FattyAcidExpr, r#const::*};
// use polars::prelude::*;

// // SFA, ∑MUFA, ∑PUFA, ∑n-6 PUFA, ∑n-3 PUFA, and n-6 PUFA/n-3 PUFA
// // col("FA").fa().ufa(col("Value"))
// /// Fatty acid indices
// /// ∑SFA, ∑MUFA, ∑PUFA, ∑n-6 PUFA, ∑n-3 PUFA, and n-6 PUFA/n-3 PUFA. The present review may help researchers to evaluate the
// impl FattyAcidExpr {
//     fn monoenoics(self, expr: Expr) -> Expr {
//         expr.filter(self.unsaturated(true).len().eq(1))
//     }

//     fn dienoics(self, expr: Expr) -> Expr {
//         expr.filter(self.unsaturated(true).len().eq(2))
//     }

//     fn trienoics(self, expr: Expr) -> Expr {
//         expr.filter(self.unsaturated(true).len().eq(3))
//     }

//     fn tetraenoics(self, expr: Expr) -> Expr {
//         expr.filter(self.unsaturated(true).len().eq(4))
//     }

//     fn pentaenoics(self, expr: Expr) -> Expr {
//         expr.filter(self.unsaturated(true).len().eq(5))
//     }

//     fn hexaenoics(self, expr: Expr) -> Expr {
//         expr.filter(self.unsaturated(true).len().eq(6))
//     }

//     /// SFA
//     ///
//     /// All saturated fatty acids
//     fn sfa(self, expr: Expr) -> Expr {
//         expr.filter(self.is_saturated())
//     }

//     /// UFA
//     ///
//     /// All unsaturated fatty acids
//     fn ufa(self, expr: Expr) -> Expr {
//         expr.filter(self.is_unsaturated())
//     }

//     /// PUFA n-3, n-6, ...
//     fn pufan(self, n: u8) -> Expr {
//         self.clone().unsaturated(true).len().list().eval(
//             col("")
//                 .struct_()
//                 .field_by_name("Index")
//                 .eq(self.carbons() - lit(n)),
//             true,
//         )
//     }

//     /// Monounsaturated fatty acids (MUFA).
//     ///
//     /// All unsaturated fatty acids having only one unsaturated bond.
//     fn monounsaturated(self, expr: Expr) -> Expr {
//         expr.filter(self.unsaturated(true).len().eq(1))
//     }

//     /// Polyunsaturated fatty acids (PUFA).
//     ///
//     /// All unsaturated fatty acids having more than one unsaturated bond.
//     fn polyunsaturated(self, expr: Expr) -> Expr {
//         expr.filter(self.unsaturated(true).len().gt(1))
//     }
// }

// /// Complex fatty acid indices.
// ///
// /// Nutritional indices.
// impl FattyAcidExpr {
//     /// Polyunsaturated fatty acid/saturated fatty acid ratio (PUFA/SFA).
//     ///
//     /// All unsaturated fatty acids having only one unsaturated bond.
//     fn polyunsaturated_to_saturated(self, expr: Expr) -> Expr {
//         self.clone().polyunsaturated(expr.clone()).sum() / expr.filter(self.is_saturated()).sum()
//     }

//     /// Index of atherogenicity (IA).
//     ///
//     /// (C12:0 + 4 * C14:0 + C16:0) / ΣUFA
//     fn index_of_atherogenicity(self, expr: Expr) -> Expr {
//         (expr.clone().filter(self.clone().equal(C12U0.clone())).sum()
//             + lit(4) * expr.clone().filter(self.clone().equal(C14U0.clone())).sum()
//             + expr.clone().filter(self.clone().equal(C16U0.clone())).sum())
//             / self.ufa(expr)
//     }

//     /// Index of thrombogenicity (IT)
//     ///
//     /// (C14:0 + C16:0 + C18:0)/((0.5*ΣMUFA) + (0.5*ΣPUFAN6) + (3*ΣPUFAN3) + (n-3/n-6)]
//     fn index_of_thrombogenicity(self, expr: Expr) -> Expr {
//         (expr.clone().filter(self.clone().equal(C14U0.clone()))
//             + expr.clone().filter(self.clone().equal(C16U0.clone()))
//             + expr.clone().filter(self.clone().equal(C18U0.clone())))
//             / (lit(0.5) * self.ufa(expr) + lit(0.5))
//     }

//     /// Hypercholesterolemic/hypocholesterolemic ratio (HH)
//     ///
//     /// (cis-C18:1 + ΣPUFA)/(C12:0 + C14:0 + C16:0)
//     fn hypercholesterolemic_ratio(self, expr: Expr) -> Expr {
//         (expr.clone().filter(self.clone().equal(C18U1Z9.clone())) // c18u1?
//             + expr.clone().filter(self.clone().polyunsaturated(expr.clone())))
//             / (expr.clone().filter(self.clone().equal(C12U0.clone()))
//                 + expr.clone().filter(self.clone().equal(C14U0.clone()))
//                 + expr.clone().filter(self.equal(C16U0.clone())))
//     }

//     /// [`Self::hypercholesterolemic_ratio`]
//     fn hypocholesterolemic_ratio(self, expr: Expr) -> Expr {
//         (expr.clone().filter(self.clone().equal(C18U1Z9.clone())) // c18u1?
//             + expr.clone().filter(self.clone().polyunsaturated(expr.clone())))
//             / (expr.clone().filter(self.clone().equal(C12U0.clone()))
//                 + expr.clone().filter(self.clone().equal(C14U0.clone()))
//                 + expr.clone().filter(self.equal(C16U0.clone())))
//     }

//     /// HPI (Health-promoting index)
//     ///
//     /// ΣUFA / (C12:0 + 4 * C14:0 + C16:0)
//     fn health_promoting_index(self, expr: Expr) -> Expr {
//         expr.clone().filter(self.clone().ufa(expr.clone()))
//             / (expr.clone().filter(self.clone().equal(C12U0.clone()))
//                 + lit(4) * expr.clone().filter(self.clone().equal(C14U0.clone()))
//                 + expr.clone().filter(self.equal(C12U0.clone())))
//     }

//     /// Unsaturation index (UI)
//     fn unsaturation_index(self, expr: Expr) -> Expr {
//         (self.unsaturated(true).len() * expr).sum()
//     }

//     /// Fish lipid quality/flesh lipid quality (FLQ)
//     ///
//     /// (EPA + DHA) / ΣFA
//     fn fish_lipid_quality(self, expr: Expr) -> Expr {
//         (expr.clone().filter(self.clone().eicosapentaenoic())
//             + expr.clone().filter(self.docosahexaenoic()))
//             / expr.sum()
//     }

//     /// [`Self::fish_lipid_quality`]
//     fn flesh_lipid_quality(self, expr: Expr) -> Expr {
//         self.fish_lipid_quality(expr)
//     }

//     /// Trans fatty acid (TFA)
//     fn tfa(self, expr: Expr) -> Expr {
//         expr.filter(
//             self.clone()
//                 .carbons()
//                 .eq(18)
//                 .and(self.unsaturated(true).len().eq(3)),
//         )
//     }
// }
