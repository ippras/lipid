use crate::fatty_acid::polars::{ExprExt as _, expr::FattyAcidExpr};
use polars::prelude::*;

// Stereospecific number [`Expr`]
#[derive(Clone, Debug)]
pub struct StereospecificNumberExpr(pub Expr);

impl StereospecificNumberExpr {
    pub fn index(self) -> Expr {
        self.0.struct_().field_by_name("Index")
    }

    // Fatty acid
    pub fn fatty_acid(self) -> FattyAcidExpr {
        self.0.struct_().field_by_name("FattyAcid").fa()
    }

    // pub fn substitute(self, kind: Kind) -> Expr {
    //     match kind {
    //         Kind::Ecn => self.fa().ecn(),
    //         Kind::Mass => self.fa().mass(),
    //         Kind::Species => as_struct(vec![
    //             self.clone().fa().c(),
    //             self.clone().fa().d(),
    //             self.fa().t(),
    //         ]),
    //         Kind::Type => self.fa().saturated(),
    //         Kind::Unsaturation => self.fa().unsaturation(),
    //     }
    // }
}

impl From<StereospecificNumberExpr> for Expr {
    fn from(value: StereospecificNumberExpr) -> Self {
        value.0
    }
}
