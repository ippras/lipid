pub use self::fatty_acid::FattyAcidSeries;

use polars::prelude::*;

/// Extension methods for [`Series`]
pub trait SeriesExt {
    fn fatty_acid(&self) -> FattyAcidSeries;
}

impl SeriesExt for Series {
    fn fatty_acid(&self) -> FattyAcidSeries {
        FattyAcidSeries::new(self).expect(r#"Expected "FattyAcids" series"#)
    }
}

pub mod fatty_acid;
