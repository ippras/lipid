use super::{FattyAcidExpr, r#const::*, find::FindByName};
use polars::prelude::*;

// SFA, ∑MUFA, ∑PUFA, ∑n-6 PUFA, ∑n-3 PUFA, and n-6 PUFA/n-3 PUFA
// col("FA").fa().ufa(col("Value"))
/// Fatty acid indices
pub trait Filter {
    fn monoenoics(self, expr: Expr) -> Expr;

    fn dienoics(self, expr: Expr) -> Expr;

    fn trienoics(self, expr: Expr) -> Expr;

    fn tetraenoics(self, expr: Expr) -> Expr;

    fn pentaenoics(self, expr: Expr) -> Expr;

    fn hexaenoics(self, expr: Expr) -> Expr;

    /// SFA
    ///
    /// All saturated fatty acids
    fn sfa(self, expr: Expr) -> Expr;

    /// UFA
    ///
    /// All unsaturated fatty acids
    fn ufa(self, expr: Expr) -> Expr;

    /// MUFA
    ///
    /// All unsaturated fatty acids having only one unsaturated bond.
    fn mufa(self, expr: Expr) -> Expr;

    /// PUFA
    ///
    /// All unsaturated fatty acids having more than one unsaturated bond.
    fn pufa(self, expr: Expr) -> Expr;

    /// PUFA n-3, n-6, ...
    fn pufan(self, n: u8) -> Expr;

    /// IA (Index of atherogenicity)
    /// (C12:0 + 4 * C14:0 + C16:0) / ΣUFA
    fn ia(self, expr: Expr) -> Expr;

    /// IT (Index of thrombogenicity)
    /// (C14:0 + C16:0 + C18:0)/((0.5*ΣMUFA) + (0.5*ΣPUFAN6) + (3*ΣPUFAN3) + (n-3/n-6)]
    fn it(self, expr: Expr) -> Expr;

    /// HH (Hypocholesterolemic/hypercholesterolemic ratio)
    /// (cis-C18:1 + ΣPUFA)/(C12:0 + C14:0 + C16:0)
    fn hh(self, expr: Expr) -> Expr;

    /// Health-promoting index (HPI)
    /// ΣUFA / (C12:0 + 4 * C14:0 + C16:0)
    fn hpi(self, expr: Expr) -> Expr;

    /// Unsaturation index (UI)
    fn ui(self, expr: Expr) -> Expr;

    /// FLQ (Fish lipid quality/flesh lipid quality)
    /// (EPA + DHA) / ΣFA
    fn flq(self, expr: Expr) -> Expr;

    /// TFA (Trans fatty acid)
    ///
    fn tfa(self, expr: Expr) -> Expr;
}

impl Filter for FattyAcidExpr {
    fn monoenoics(self, expr: Expr) -> Expr {
        expr.filter(self.unsaturated().len().eq(1))
    }

    fn dienoics(self, expr: Expr) -> Expr {
        expr.filter(self.unsaturated().len().eq(2))
    }

    fn trienoics(self, expr: Expr) -> Expr {
        expr.filter(self.unsaturated().len().eq(3))
    }

    fn tetraenoics(self, expr: Expr) -> Expr {
        expr.filter(self.unsaturated().len().eq(4))
    }

    fn pentaenoics(self, expr: Expr) -> Expr {
        expr.filter(self.unsaturated().len().eq(5))
    }

    fn hexaenoics(self, expr: Expr) -> Expr {
        expr.filter(self.unsaturated().len().eq(6))
    }

    fn sfa(self, expr: Expr) -> Expr {
        expr.filter(self.unsaturated().len().eq(0))
    }

    fn ufa(self, expr: Expr) -> Expr {
        expr.filter(self.unsaturated().len().neq(0))
    }

    fn mufa(self, expr: Expr) -> Expr {
        expr.filter(self.unsaturated().len().eq(1))
    }

    fn pufa(self, expr: Expr) -> Expr {
        expr.filter(self.unsaturated().len().gt(1))
    }

    fn pufan(self, n: u8) -> Expr {
        self.clone().unsaturated().len().list().eval(
            col("")
                .struct_()
                .field_by_name("Index")
                .eq(self.carbons() - lit(n)),
            true,
        )
    }

    fn ia(self, expr: Expr) -> Expr {
        (expr.clone().filter(self.clone().equal(C12U0.clone())).sum()
            + lit(4) * expr.clone().filter(self.clone().equal(C14U0.clone())).sum()
            + expr.clone().filter(self.clone().equal(C16U0.clone())).sum())
            / self.ufa(expr)
    }

    fn it(self, expr: Expr) -> Expr {
        (expr.clone().filter(self.clone().equal(C14U0.clone()))
            + expr.clone().filter(self.clone().equal(C16U0.clone()))
            + expr.clone().filter(self.clone().equal(C18U0.clone())))
            / (lit(0.5) * self.ufa(expr) + lit(0.5))
    }

    fn hh(self, expr: Expr) -> Expr {
        (expr.clone().filter(self.clone().equal(C18U1Z9.clone())) // c18u1?
            + expr.clone().filter(self.clone().pufa(expr.clone())))
            / (expr.clone().filter(self.clone().equal(C12U0.clone()))
                + expr.clone().filter(self.clone().equal(C14U0.clone()))
                + expr.clone().filter(self.equal(C16U0.clone())))
    }

    fn hpi(self, expr: Expr) -> Expr {
        expr.clone().filter(self.clone().ufa(expr.clone()))
            / (expr.clone().filter(self.clone().equal(C12U0.clone()))
                + lit(4) * expr.clone().filter(self.clone().equal(C14U0.clone()))
                + expr.clone().filter(self.equal(C12U0.clone())))
    }

    fn ui(self, expr: Expr) -> Expr {
        (self.unsaturated().len() * expr).sum()
    }

    fn flq(self, expr: Expr) -> Expr {
        (expr.clone().filter(self.clone().eicosapentaenoic())
            + expr.clone().filter(self.docosahexaenoic()))
            / expr.sum()
    }

    fn tfa(self, expr: Expr) -> Expr {
        expr.filter(
            self.clone()
                .carbons()
                .eq(18)
                .and(self.unsaturated().len().eq(3)),
        )
    }
}
