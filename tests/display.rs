use lipid::{display::FattyAcid, fatty_acid::r#const::*, prelude::*};
use polars::prelude::*;

fn fatty_acid(
    bounds: impl IntoIterator<Item = &'static str>,
    options: Options,
) -> PolarsResult<FattyAcid> {
    Series::from_iter(bounds)
        .cast(&BOUND_DATA_TYPE)?
        .bound()?
        .display(options)
}

mod common {
    use super::*;

    fn fatty_acid(bounds: impl IntoIterator<Item = &'static str>) -> PolarsResult<FattyAcid> {
        super::fatty_acid(bounds, Default::default())
    }

    #[test]
    #[rustfmt::skip]
    fn delta() -> PolarsResult<()> {
        let c2u0 = fatty_acid(C2U0)?;
        assert_eq!("2:0", c2u0.to_string());
        assert_eq!("2:0", format!("{c2u0:#}"));
        assert_eq!("02:00", format!("{c2u0:02}"));

        let c16u1dc9 = fatty_acid(C16U1DC9)?;
        assert_eq!("16:1", c16u1dc9.to_string());
        assert_eq!("16:1Δ9", format!("{c16u1dc9:#}"));
        assert_eq!("16:01Δ09", format!("{c16u1dc9:#02}"));

        let c18u2dc9dc12 = fatty_acid(C18U2DC9DC12)?;
        assert_eq!("18:2", c18u2dc9dc12.to_string());
        assert_eq!("18:2Δ9,12", format!("{c18u2dc9dc12:#}"));
        assert_eq!("18:02Δ09,12", format!("{c18u2dc9dc12:#02}"));

        let c20u4dc8dc11dc14dc17 = fatty_acid(C20U4DC8DC11DC14DC17)?;
        assert_eq!("20:4", c20u4dc8dc11dc14dc17.to_string());
        assert_eq!("20:4Δ8,11,14,17", format!("{c20u4dc8dc11dc14dc17:#}"));
        assert_eq!("20:04Δ08,11,14,17", format!("{c20u4dc8dc11dc14dc17:#02}"));

        let c22u6dc4dc7dc10dc13dc16dc19 = fatty_acid(C22U6DC4DC7DC10DC13DC16DC19)?;
        assert_eq!("22:6", c22u6dc4dc7dc10dc13dc16dc19.to_string());
        assert_eq!("22:6Δ4,7,10,13,16,19", format!("{c22u6dc4dc7dc10dc13dc16dc19:#}"));
        assert_eq!("22:06Δ04,07,10,13,16,19", format!("{c22u6dc4dc7dc10dc13dc16dc19:#02}"));

        Ok(())
    }
}

mod system {
    use super::*;

    fn fatty_acid(bounds: impl IntoIterator<Item = &'static str>) -> PolarsResult<FattyAcid> {
        super::fatty_acid(
            bounds,
            Options {
                kind: System,
                ..Default::default()
            },
        )
    }

    #[test]
    #[rustfmt::skip]
    fn system() -> PolarsResult<()> {
        let c2u0 = fatty_acid(C2U0)?;
        assert_eq!("c2u0", c2u0.to_string());
        assert_eq!("c2u0", format!("{c2u0:#}"));
        assert_eq!("c02u00", format!("{c2u0:02}"));

        let c16u1dc9 = fatty_acid(C16U1DC9)?;
        assert_eq!("c16u1dc9", c16u1dc9.to_string());
        assert_eq!("c16u1dc9", format!("{c16u1dc9:#}"));
        assert_eq!("c16u01dc09", format!("{c16u1dc9:02}"));

        let c18u2dc9dc12 = fatty_acid(C18U2DC9DC12)?;
        assert_eq!("c18u2dc9dc12", c18u2dc9dc12.to_string());
        assert_eq!("c18u2dc9dc12", format!("{c18u2dc9dc12:#}"));
        assert_eq!("c18u02dc09dc12", format!("{c18u2dc9dc12:02}"));

        let c20u4dc8dc11dc14dc17 = fatty_acid(C20U4DC8DC11DC14DC17)?;
        assert_eq!("c20u4dc8dc11dc14dc17", c20u4dc8dc11dc14dc17.to_string());
        assert_eq!("c20u4dc8dc11dc14dc17", format!("{c20u4dc8dc11dc14dc17:#}"));
        assert_eq!("c20u04dc08dc11dc14dc17", format!("{c20u4dc8dc11dc14dc17:02}"));

        let c22u6dc4dc7dc10dc13dc16dc19 = fatty_acid(C22U6DC4DC7DC10DC13DC16DC19)?;
        assert_eq!("c22u6dc4dc7dc10dc13dc16dc19", c22u6dc4dc7dc10dc13dc16dc19.to_string());
        assert_eq!("c22u6dc4dc7dc10dc13dc16dc19", format!("{c22u6dc4dc7dc10dc13dc16dc19:#}"));
        assert_eq!("c22u06dc04dc07dc10dc13dc16dc19", format!("{c22u6dc4dc7dc10dc13dc16dc19:02}"));

        Ok(())
    }
}
