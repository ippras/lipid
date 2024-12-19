pub use self::{
    column::ColumnExt,
    data_frame::DataFrameExt,
    expr::{
        ExprExt,
        filter::Filter,
        find::{Find, FindByName},
    },
    series::SeriesExt,
};
use polars::prelude::*;
use std::sync::LazyLock;

/// Fatty acid column name
pub const COLUMN: &str = "FattyAcid";

/// Fatty acid schema
pub static SCHEMA: LazyLock<Schema> = LazyLock::new(|| {
    Schema::from_iter([
        Field::new("Carbons".into(), DataType::UInt8),
        Field::new(
            "Unsaturated".into(),
            DataType::Struct(vec![
                Field::new("Index".into(), DataType::List(Box::new(DataType::UInt8))),
                Field::new("Isomerism".into(), DataType::List(Box::new(DataType::Int8))),
                Field::new(
                    "Unsaturation".into(),
                    DataType::List(Box::new(DataType::UInt8)),
                ),
            ]),
        ),
    ])
});

/// Extension methods for [`Schema`]
pub trait SchemaExt {
    fn names(&self) -> Vec<Expr>;
}

impl SchemaExt for Schema {
    fn names(&self) -> Vec<Expr> {
        self.iter_names_cloned().map(col).collect()
    }
}

mod column;
mod data_frame;
mod expr;
mod series;
