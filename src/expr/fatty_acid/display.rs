use crate::{prelude::*, r#struct::fatty_acid::display::Kind};
use polars::prelude::*;

impl FattyAcidExpr {
    #[inline]
    pub fn display(self, kind: Kind) -> Expr {
        // let indices = self.indices();
        // let carbon = self.carbon();
        // let unsaturated = indices.list().len();
        // format_str("{}:{}-{}", [carbon, unsaturated, indices])
        self.0.map(
            |column| {
                let fatty_acid = column.try_fatty_acid()?;
                Ok(match kind {
                    Kind::Delta => fatty_acid.delta(),
                    Kind::Id => fatty_acid.id(),
                }?
                .into_column())
            },
            |_, field| Ok(Field::new(field.name().clone(), DataType::String)),
        )
    }

    #[inline]
    pub fn delta(self) -> Expr {
        self.0.map(
            |column| Ok(column.try_fatty_acid()?.delta()?.into_column()),
            |_, field| Ok(Field::new(field.name().clone(), DataType::String)),
        )
    }

    #[inline]
    pub fn id(self) -> Expr {
        self.0.map(
            |column| Ok(column.try_fatty_acid()?.id()?.into_column()),
            |_, field| Ok(Field::new(field.name().clone(), DataType::String)),
        )
    }
}
