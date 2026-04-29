use crate::prelude::*;
use polars::prelude::*;
use std::num::NonZeroI8;

impl FattyAcidSumByBounds for FattyAcidExpr {
    fn sum_conjugated(self, expr: Self::Output) -> Self::Output {
        expr.filter(self.is_conjugated(true)).sum()
    }

    fn sum_monounsaturated(self, expr: Self::Output) -> Self::Output {
        expr.filter(self.is_monounsaturated()).sum()
    }

    fn sum_polyunsaturated(self, expr: Self::Output) -> Self::Output {
        expr.filter(self.is_polyunsaturated()).sum()
    }

    fn sum_saturated(self, expr: Self::Output) -> Self::Output {
        expr.filter(self.is_saturated()).sum()
    }

    fn sum_trans(self, expr: Self::Output) -> Self::Output {
        expr.filter(self.is_trans()).sum()
    }

    fn sum_unsaturated(self, expr: Expr, offset: Option<NonZeroI8>) -> Expr {
        expr.filter(self.is_unsaturated(offset)).sum()
    }
}

impl FattyAcidSumByDoubleBounds for FattyAcidExpr {
    fn sum_dienoics(self, expr: Expr) -> Expr {
        expr.filter(self.clone().is_dienoic()).sum()
    }

    fn sum_hexaenoics(self, expr: Expr) -> Expr {
        expr.filter(self.clone().is_hexaenoic()).sum()
    }

    fn sum_monoenoics(self, expr: Expr) -> Expr {
        expr.filter(self.clone().is_monoenoic()).sum()
    }

    fn sum_pentaenoics(self, expr: Expr) -> Expr {
        expr.filter(self.clone().is_pentaenoic()).sum()
    }

    fn sum_tetraenoics(self, expr: Expr) -> Expr {
        expr.filter(self.clone().is_tetraenoic()).sum()
    }

    fn sum_trienoic(self, expr: Expr) -> Expr {
        expr.filter(self.clone().is_trienoic()).sum()
    }
}
