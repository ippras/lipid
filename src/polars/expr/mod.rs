pub use self::{fatty_acid::FattyAcidExpr, triacylglycerol::TriacylglycerolExpr};

use polars::prelude::*;

/// Extension methods for [`Expr`]
pub trait ExprExt: Sized {
    fn fatty_acid(self) -> FattyAcidExpr;

    fn triacylglycerol(self) -> TriacylglycerolExpr;
}

impl ExprExt for Expr {
    fn fatty_acid(self) -> FattyAcidExpr {
        FattyAcidExpr(self)
    }

    fn triacylglycerol(self) -> TriacylglycerolExpr {
        TriacylglycerolExpr(self)
    }
}

pub mod fatty_acid;
pub mod triacylglycerol;
