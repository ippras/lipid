use crate::prelude::*;
use polars::prelude::*;
use polars_ext::prelude::*;

impl FattyAcidExpr {
    /// Select
    pub fn select(self, mask: Expr, filter: bool) -> Expr {
        if filter {
            self.filter(mask)
        } else {
            self.nullify(mask)
        }
    }

    /// Filter
    pub fn filter(self, mask: Expr) -> Expr {
        self.0.filter(mask)
    }

    /// Nullify
    pub fn nullify(self, mask: Expr) -> Expr {
        self.0.nullify(mask)
    }
}

impl FattyAcidFilter for FattyAcidExpr {
    fn dienoics(self, expr: Expr) -> Expr {
        expr.filter(self.clone().is_dienoic()).sum()
    }

    fn hexaenoics(self, expr: Expr) -> Expr {
        expr.filter(self.clone().is_hexaenoic()).sum()
    }

    fn monoenoics(self, expr: Expr) -> Expr {
        expr.filter(self.clone().is_monoenoic()).sum()
    }

    fn pentaenoics(self, expr: Expr) -> Expr {
        expr.filter(self.clone().is_pentaenoic()).sum()
    }

    fn tetraenoics(self, expr: Expr) -> Expr {
        expr.filter(self.clone().is_tetraenoic()).sum()
    }

    fn trienoic(self, expr: Expr) -> Expr {
        expr.filter(self.clone().is_trienoic()).sum()
    }
}
