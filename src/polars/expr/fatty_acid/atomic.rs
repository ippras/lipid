use super::FattyAcidExpr;
use crate::prelude::*;
use polars::prelude::*;

/// Atomic methods
impl FattyAcidExpr {
    /// Returns the number of carbons for each fatty acid in the given series.
    #[inline]
    pub fn carbons(self) -> Expr {
        self.0.map(
            |column| Ok(Some(column.fatty_acid()?.carbons()?.into_column())),
            GetOutput::from_type(DataType::UInt8),
        )
    }

    /// Returns the number of hydrogens for each fatty acid in the given series.
    #[inline]
    pub fn hydrogens(self) -> Expr {
        self.0.map(
            |column| Ok(Some(column.fatty_acid()?.hydrogens()?.into_column())),
            GetOutput::from_type(DataType::UInt8),
        )
    }
}
