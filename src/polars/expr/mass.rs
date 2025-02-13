use polars::prelude::*;

/// Mass
pub trait Mass {
    /// Mass
    fn mass(self, adduct: Option<Expr>) -> Expr;
}
