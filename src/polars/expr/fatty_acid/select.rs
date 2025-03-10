use super::FattyAcidExpr;
use crate::prelude::*;
use polars::prelude::*;

impl FattyAcidExpr {
    pub fn filter(self, mask: Expr) -> Expr {
        self.0.filter(mask)
    }

    pub fn nullify(self, mask: Expr) -> Expr {
        ternary_expr(mask, self.0, lit(NULL))
        // as_struct(vec![self.0, mask.name().prefix("_")]).map(
        //     move |column| {
        //         let fields = column.struct_()?.fields_as_series();
        //         Ok(Some(
        //             fields[0]
        //                 .fatty_acid()?
        //                 .nullify(fields[1].bool()?)?
        //                 .into_column(),
        //         ))
        //     },
        //     GetOutput::first(),
        // )
    }

    /// Saturated fatty acids (SFA).
    ///
    /// All saturated fatty acids
    pub fn saturated(self, filter: bool) -> Expr {
        self.0.map(
            move |column| Ok(Some(column.fatty_acid()?.saturated(filter)?.into_column())),
            GetOutput::same_type(),
        )
    }

    /// Unsaturated fatty acids (UFA).
    ///
    /// All unsaturated fatty acids
    pub fn unsaturated(self, filter: bool) -> Expr {
        self.0.map(
            move |column| {
                Ok(Some(
                    column.fatty_acid()?.unsaturated(filter)?.into_column(),
                ))
            },
            GetOutput::same_type(),
        )
    }

    /// Monounsaturated fatty acids (MUFA).
    ///
    /// All unsaturated fatty acids having only one unsaturated bond.
    pub fn monounsaturated(self, filter: bool) -> Expr {
        self.0.map(
            move |column| {
                Ok(Some(
                    column
                        .fatty_acid()?
                        .select(&column.fatty_acid()?.is_monounsaturated()?, filter)?
                        .into_column(),
                ))
            },
            GetOutput::same_type(),
        )
    }

    /// Polyunsaturated fatty acids (PUFA).
    ///
    /// All unsaturated fatty acids having more than one unsaturated bond.
    pub fn polyunsaturated(self, filter: bool) -> Expr {
        self.0.map(
            move |column| {
                Ok(Some(
                    column
                        .fatty_acid()?
                        .select(&column.fatty_acid()?.is_polyunsaturated()?, filter)?
                        .into_column(),
                ))
            },
            GetOutput::same_type(),
        )
    }
}
