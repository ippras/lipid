use super::FattyAcidExpr;
use crate::polars::{ColumnExt as _, ExprExt};
use polars::prelude::*;

impl FattyAcidExpr {
    #[inline]
    pub fn equivalent_carbon_number(self) -> Expr {
        self.0.map(
            |column| {
                Ok(Some(
                    column
                        .fatty_acid()?
                        .equivalent_carbon_number()?
                        .into_column(),
                ))
            },
            GetOutput::from_type(DataType::UInt8),
        )
    }

    #[inline]
    pub fn equivalent_chain_lengths(self, retention_time: Expr, options: Options) -> Expr {
        self.clone()
            .saturated(true)
            .fatty_acid()
            .carbons()
            .forward_fill(None)
            + self.fractional_chain_length(retention_time, options)
    }

    #[inline]
    pub fn fractional_chain_length(self, retention_time: Expr, options: Options) -> Expr {
        let options = |mut expr: Expr| {
            if options.logarithmic {
                expr = expr.log(10.0)
            }
            expr
        };
        let unsaturated_time = || options(retention_time.clone());
        let saturated_time = || {
            options(ternary_expr(
                self.clone().is_saturated(),
                retention_time.clone(),
                lit(NULL),
            ))
        };
        let saturated_carbons = || self.clone().saturated(false).fatty_acid().carbons();
        ternary_expr(
            self.clone().is_saturated(),
            lit(0),
            (saturated_carbons().backward_fill(None) - saturated_carbons().forward_fill(None))
                * (unsaturated_time() - saturated_time().forward_fill(None))
                / (saturated_time().backward_fill(None) - saturated_time().forward_fill(None)),
        )
    }
    // #[inline]
    // pub fn fractional_chain_length(self, retention_time: Expr, options: Options) -> Expr {
    //     let options = |mut expr: Expr| {
    //         if options.logarithmic {
    //             expr = expr.log(10.0)
    //         }
    //         expr
    //     };
    //     let unsaturated_time = || options(retention_time.clone());
    //     // let saturated_time = || options(self.clone().saturated_or_null(retention_time.clone()));
    //     // let saturated_carbons = || self.clone().saturated_or_null(self.clone().carbons());
    //     // retention_time.clone()
    //     let saturated_time = || options(self.clone().saturated(true).fatty_acid());
    //     let saturated_carbons = || self.clone().saturated_or_null(self.clone().carbons());
    //     ternary_expr(
    //         self.clone().is_saturated(),
    //         lit(0),
    //         (saturated_carbons().backward_fill(None) - saturated_carbons().forward_fill(None))
    //             * (unsaturated_time() - saturated_time().forward_fill(None))
    //             / (saturated_time().backward_fill(None) - saturated_time().forward_fill(None)),
    //     )
    // }
}

/// Chain length options
#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub logarithmic: bool,
}

impl Options {
    pub const fn new() -> Self {
        Self { logarithmic: false }
    }

    pub fn logarithmic(self, logarithmic: bool) -> Self {
        Self { logarithmic }
    }
}
