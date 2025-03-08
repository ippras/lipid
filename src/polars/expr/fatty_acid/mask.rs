use super::FattyAcidExpr;
use crate::prelude::*;
use polars::prelude::*;

impl FattyAcidExpr {
    /// Is saturated
    #[inline]
    pub fn is_saturated(self) -> Expr {
        self.0.map(
            |column| Ok(Some(column.fatty_acid()?.is_saturated()?.into_column())),
            GetOutput::from_type(DataType::Boolean),
        )
    }

    /// Is unsaturated
    #[inline]
    pub fn is_unsaturated(self) -> Expr {
        self.0.map(
            |column| Ok(Some(column.fatty_acid()?.is_unsaturated()?.into_column())),
            GetOutput::from_type(DataType::Boolean),
        )
    }

    /// Is monounsaturated
    #[inline]
    pub fn is_monounsaturated(self) -> Expr {
        self.0.map(
            |column| {
                Ok(Some(
                    column.fatty_acid()?.is_monounsaturated()?.into_column(),
                ))
            },
            GetOutput::from_type(DataType::Boolean),
        )
    }

    /// Is polyunsaturated
    #[inline]
    pub fn is_polyunsaturated(self) -> Expr {
        self.0.map(
            |column| {
                Ok(Some(
                    column.fatty_acid()?.is_polyunsaturated()?.into_column(),
                ))
            },
            GetOutput::from_type(DataType::Boolean),
        )
    }
}
