use crate::prelude::*;
use polars::prelude::*;

impl TriacylglycerolExpr {
    #[inline]
    pub fn display(self, stereospecificity: Stereospecificity) -> Expr {
        self.0.map(
            move |column| {
                Ok(column
                    .try_triacylglycerol()?
                    .display(stereospecificity)?
                    .into_column())
            },
            |_, field| Ok(Field::new(field.name().clone(), DataType::String)),
        )
    }
}
