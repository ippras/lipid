use crate::prelude::*;
use polars::prelude::*;
use std::num::NonZeroI8;

impl FattyAcidMaskByBounds for FattyAcidExpr {
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

    fn is_monounsaturated(self) -> Self::Expr {
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

impl FattyAcidMaskByDoubleBounds for FattyAcidExpr {
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
}

impl FattyAcidMaskByParity for FattyAcidExpr {
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
}

// impl FattyAcidMaskByName for FattyAcidExpr {
//     fn is_alpha_linolenic(self) -> Expr {
//         self.equal(C18C9C12C15.clone())
//     }

//     fn is_butyric(self) -> Expr {
//         self.equal(C18C9C12.clone())
//     }

//     fn is_docosahexaenoic(self) -> Expr {
//         self.equal(C22C4C7C10C13C16C19.clone())
//     }

//     fn is_eicosapentaenoic(self) -> Expr {
//         self.equal(C20C5C8C11C14C17.clone())
//     }

//     fn is_linoleic(self) -> Expr {
//         self.equal(C18C9C12.clone())
//     }

//     fn is_oleic(self) -> Expr {
//         self.equal(C18C9.clone())
//     }
// }
