pub use self::{
    column::ColumnExt,
    data_frame::DataFrameExt,
    // expr::{ExprExt, filter::Filter, find::FindByName},
    expr::ExprExt,
    series::SeriesExt,
};

/// Fatty acid column name
pub const FATTY_ACID_COLUMN: &str = "FattyAcid";

pub mod prelude {
    pub use super::{
        column::ColumnExt,
        data_frame::DataFrameExt,
        // expr::{ExprExt, r#const::*, filter::Filter, find::FindByName},
        series::SeriesExt,
    };
}

pub mod column;
pub mod data_frame;
pub mod expr;
pub mod schema;
pub mod series;
