use crate::prelude::*;
use polars::prelude::*;
use std::{
    num::NonZeroI8,
    ops::{Bound, RangeBounds},
};

impl FattyAcidExpr {
    /// [`Self::sum_of_short_chain_fatty_acids`]
    pub fn is_short_chain_fatty_acid(self) -> Expr {
        self.carbon().lt_eq(5)
    }

    /// [`Self::sum_of_medium_chain_fatty_acids`]
    pub fn is_medium_chain_fatty_acid(self) -> Expr {
        self.clone().carbon().gt_eq(6).and(self.carbon().lt_eq(12))
    }

    /// [`Self::sum_of_long_chain_fatty_acids`]
    pub fn is_long_chain_fatty_acid(self) -> Expr {
        self.clone().carbon().gt_eq(13).and(self.carbon().lt_eq(21))
    }

    /// [`Self::sum_of_very_long_chain_fatty_acids`]
    pub fn is_very_long_chain_fatty_acid(self) -> Expr {
        self.carbon().gt_eq(22)
    }
}

impl FattyAcidExpr {
    /// [`Self::sum_of_saturated_fatty_acids`]
    pub fn is_saturated_fatty_acid(self) -> Expr {
        self.indices().list().len().eq(0)
    }

    /// [`Self::sum_of_monounsaturated_fatty_acids`]
    pub fn is_monounsaturated_fatty_acid(self) -> Expr {
        self.indices().list().len().eq(1)
    }

    /// [`Self::sum_of_n_unsaturated_fatty_acids`]
    pub fn is_n_unsaturated_fatty_acid(self, n: impl RangeBounds<u8>) -> Expr {
        let mut predicate = lit(true);
        let len = self.indices().list().len();
        match n.start_bound() {
            Bound::Included(start) => predicate = predicate.and(len.clone().gt_eq(*start)),
            Bound::Excluded(start) => predicate = predicate.and(len.clone().gt(*start)),
            Bound::Unbounded => {}
        };
        match n.end_bound() {
            Bound::Included(end) => predicate = predicate.and(len.clone().lt_eq(*end)),
            Bound::Excluded(end) => predicate = predicate.and(len.clone().lt(*end)),
            Bound::Unbounded => {}
        };
        predicate
    }

    /// [`Self::sum_of_polyunsaturated_fatty_acids`]
    pub fn is_polyunsaturated_fatty_acid(self) -> Expr {
        self.indices().list().len().gt(1)
    }

    /// [`Self::sum_of_unsaturated_fatty_acids`]
    pub fn is_unsaturated_fatty_acid(self) -> Expr {
        self.indices().list().len().gt(0)
    }
}

impl FattyAcidExpr {
    /// [`Self::sum_of_offset_fatty_acids`]
    pub fn is_offset_fatty_acid(self, offset: Option<NonZeroI8>) -> Expr {
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

    /// [`Self::sum_of_delta9_fatty_acids`]
    pub fn is_delta9_fatty_acid() -> Expr {
        self.is_offset_fatty_acid(NonZeroI8::new(9))
    }

    /// [`Self::sum_of_delta12_fatty_acids`]
    pub fn is_delta12_fatty_acid() -> Expr {
        self.is_offset_fatty_acid(NonZeroI8::new(12))
    }

    /// [`Self::sum_of_omega9_fatty_acids`]
    pub fn is_omega9_fatty_acid() -> Expr {
        self.is_offset_fatty_acid(NonZeroI8::new(-9))
    }

    /// [`Self::sum_of_omega6_fatty_acids`]
    pub fn is_omega6_fatty_acid() -> Expr {
        self.is_offset_fatty_acid(NonZeroI8::new(-6))
    }

    /// [`Self::sum_of_omega3_fatty_acids`]
    pub fn is_omega3_fatty_acid() -> Expr {
        self.is_offset_fatty_acid(NonZeroI8::new(-3))
    }
}

impl FattyAcidExpr {
    /// [`Self::sum_of_cis_fatty_acids`]
    pub fn is_cis_fatty_acid(self) -> Expr {
        self.clone().indices().list().len().gt(0).and(
            self.indices()
                .list()
                .agg(element().struct_().field_by_name(PARITY).any(false).not()),
        )
    }

    /// [`Self::sum_of_trans_fatty_acids`]
    pub fn is_trans_fatty_acid(self) -> Expr {
        self.indices()
            .list()
            .agg(element().struct_().field_by_name(PARITY).any(false))
    }
}

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

// impl FattyAcidMaskByParity for FattyAcidExpr {
//     fn is_cis(self) -> Expr {
//         self.clone().indices().list().len().gt(0).and(
//             self.indices()
//                 .list()
//                 .agg(element().struct_().field_by_name(PARITY).any(false).not()),
//         )
//     }

//     fn is_trans(self) -> Expr {
//         self.indices()
//             .list()
//             .agg(element().struct_().field_by_name(PARITY).any(false))
//     }
// }
