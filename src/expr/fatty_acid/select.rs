use crate::prelude::*;
use polars::prelude::*;
use polars_ext::prelude::*;
use std::num::NonZeroI8;

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

impl FattyAcidMask for FattyAcidExpr {
    type Output = Expr;

    // Parity (cis, trans)

    fn is_cis(self) -> Expr {
        self.clone().indices().list().len().gt(0).and(
            self.indices()
                .list()
                .agg(element().struct_().field_by_name(PARITY).any(false).not()),
        )
    }

    fn is_trans(self) -> Expr {
        self.indices()
            .list()
            .agg(element().struct_().field_by_name(PARITY).any(false))
    }

    // Complex

    fn is_conjugated(self, strict: bool) -> Expr {
        let index = if strict {
            ternary_expr(
                element().struct_().field_by_name(TRIPLE),
                lit(NULL),
                element().struct_().field_by_name(INDEX),
            )
        } else {
            element().struct_().field_by_name(INDEX)
        };
        let indices = self.indices().list().eval(index);
        (indices.clone() - indices.list().shift(lit(1)))
            .list()
            .contains(lit(2), false)
    }

    // Enoic

    fn is_dienoic(self) -> Expr {
        self.indices().list().len().eq(2)
    }

    fn is_hexaenoic(self) -> Expr {
        self.indices().list().len().eq(6)
    }

    fn is_monoenoic(self) -> Expr {
        self.indices().list().len().eq(1)
    }

    fn is_pentaenoic(self) -> Expr {
        self.indices().list().len().eq(5)
    }

    fn is_tetraenoic(self) -> Expr {
        self.indices().list().len().eq(4)
    }

    fn is_trienoic(self) -> Expr {
        self.indices().list().len().eq(3)
    }

    // Unsaturated

    fn is_monounsaturated(self) -> Self::Output {
        self.is_monoenoic()
    }

    fn is_polyunsaturated(self) -> Expr {
        self.indices().list().len().gt(1)
    }

    fn is_saturated(self) -> Expr {
        self.indices().list().len().eq(0)
    }

    fn is_unsaturated(self, offset: Option<NonZeroI8>) -> Expr {
        let indices = self.clone().indices().list();
        match offset {
            Some(offset) => match offset.get() {
                omega @ ..0 => {
                    let last = indices.last().struct_().field_by_name(INDEX);
                    last.eq_missing(self.carbon() - lit(omega.unsigned_abs()))
                }
                delta @ 0.. => {
                    let first = indices.first().struct_().field_by_name(INDEX);
                    first.eq_missing(delta)
                }
            },
            None => indices.len().neq(0),
        }
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
