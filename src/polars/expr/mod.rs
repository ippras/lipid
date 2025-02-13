pub use self::{fatty_acid::FattyAcidExpr, triacylglycerol::TriacylglycerolExpr};

use polars::prelude::*;

/// Extension methods for [`Expr`]
pub trait ExprExt: Sized {
    fn fatty_acid(self) -> FattyAcidExpr;

    fn fa(self) -> FattyAcidExpr {
        self.fatty_acid()
    }

    fn triacylglycerol(self) -> TriacylglycerolExpr;

    fn tag(self) -> TriacylglycerolExpr {
        self.triacylglycerol()
    }
}

impl ExprExt for Expr {
    fn fatty_acid(self) -> FattyAcidExpr {
        FattyAcidExpr(self)
    }

    fn triacylglycerol(self) -> TriacylglycerolExpr {
        TriacylglycerolExpr(self)
    }
}

pub mod chain_length;
pub mod fatty_acid;
pub mod mass;
pub mod triacylglycerol;
