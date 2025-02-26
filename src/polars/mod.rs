pub use self::{
    column::ColumnExt,
    data_frame::DataFrameExt,
    // expr::{ExprExt, filter::Filter, find::FindByName},
    expr::ExprExt,
    series::SeriesExt,
};

/// Fatty acid column name
pub const FATTY_ACID_COLUMN: &str = "FattyAcid";

pub mod bound;
pub mod column;
pub mod data_frame;
pub mod expr;
pub mod fatty_acid;
pub mod schema;
pub mod series;
pub mod r#type;
