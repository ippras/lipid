use lipid::{display::FattyAcid, fatty_acid::r#const::*, prelude::*};
use polars::prelude::*;

mod common {
    use super::*;

    fn fatty_acid(bounds: impl IntoIterator<Item = &'static str>) -> PolarsResult<FattyAcid> {
        Series::from_iter(bounds)
            .cast(&BOUND_DATA_TYPE)?
            .bound()?
            .display(Options::default())
    }

    #[test]
    fn delta() -> PolarsResult<()> {
        let c2u0 = fatty_acid(C2U0)?;
        assert_eq!("2:0", c2u0.to_string());
        assert_eq!("2:0", format!("{c2u0:#}"));
        assert_eq!("02:00", format!("{c2u0:02}"));
        let c18u2dc9dc12 = fatty_acid(C18U2DC9DC12)?;
        assert_eq!("18:2", c18u2dc9dc12.to_string());
        assert_eq!("18:2Δ9,12", format!("{c18u2dc9dc12:#}"));
        assert_eq!("18:02Δ09,12", format!("{c18u2dc9dc12:#02}"));
        Ok(())
    }
}
