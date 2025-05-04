use super::FattyAcidExpr;
use polars::prelude::*;
use polars_ext::prelude::*;

impl FattyAcidExpr {
    pub fn select(self, mask: Expr, filter: bool) -> Expr {
        if filter {
            self.filter(mask)
        } else {
            self.nullify(mask)
        }
    }

    pub fn filter(self, mask: Expr) -> Expr {
        self.0.filter(mask)
    }

    pub fn nullify(self, mask: Expr) -> Expr {
        self.0.nullify(mask)
    }
}
