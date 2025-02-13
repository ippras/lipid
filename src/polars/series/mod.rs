pub use self::fatty_acid::FattyAcidSeries;

use polars::prelude::*;

/// Extension methods for [`Series`]
pub trait SeriesExt {
    fn fatty_acid(&self) -> FattyAcidSeries;

    fn fa(&self) -> FattyAcidSeries {
        self.fatty_acid()
    }
}

impl SeriesExt for Series {
    fn fatty_acid(&self) -> FattyAcidSeries {
        FattyAcidSeries::new(self).expect(r#"Expected "FattyAcid" series"#)
    }
}

pub mod fatty_acid;
