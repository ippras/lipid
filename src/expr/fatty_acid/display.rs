use crate::prelude::*;
use polars::prelude::*;

impl FattyAcidExpr {
    #[inline]
    pub fn display(self) -> Expr {
        // let indices = self.indices();
        // let carbon = self.carbon();
        // let unsaturated = indices.list().len();
        // format_str("{}:{}-{}", [carbon, unsaturated, indices])
        self.0.map(
            |column| Ok(column.try_fatty_acid()?.delta()?.into_column()),
            |_, field| Ok(Field::new(field.name().clone(), DataType::String)),
        )
    }
}
