use lipid::prelude::*;
use polars::prelude::*;
use std::num::NonZeroI8;

fn fatty_acid<const N: usize>(
    bounds: &[(Option<Option<NonZeroI8>>, Option<&str>); N],
) -> PolarsResult<FattyAcidChunked> {
    FattyAcidChunked::try_from(bounds)
}

mod common {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn delta() -> PolarsResult<()> {
        let c4 = fatty_acid(&C4)?;
        let display = c4.display(Default::default());
        assert_eq!("4:0", display.to_string());
        assert_eq!("4:0", format!("{display:#}"));
        assert_eq!("04:00", format!("{display:02}"));

        let c16dc9 = fatty_acid(&C16DC9)?;
        let display = c16dc9.display(Default::default());
        assert_eq!("16:1", display.to_string());
        assert_eq!("16:1Δ9", format!("{display:#}"));
        assert_eq!("16:01Δ09", format!("{display:#02}"));

        let c18dc9dc12 = fatty_acid(&C18DC9DC12)?;
        let display = c18dc9dc12.display(Default::default());
        assert_eq!("18:2", display.to_string());
        assert_eq!("18:2Δ9,12", format!("{display:#}"));
        assert_eq!("18:02Δ09,12", format!("{display:#02}"));

        let c20dc8dc11dc14dc17 = fatty_acid(&C20DC8DC11DC14DC17)?;
        let display = c20dc8dc11dc14dc17.display(Default::default());
        assert_eq!("20:4", display.to_string());
        assert_eq!("20:4Δ8,11,14,17", format!("{display:#}"));
        assert_eq!("20:04Δ08,11,14,17", format!("{display:#02}"));

        let c22dc4dc7dc10dc13dc16dc19 = fatty_acid(&C22DC4DC7DC10DC13DC16DC19)?;
        let display = c22dc4dc7dc10dc13dc16dc19.display(Default::default());
        assert_eq!("22:6", display.to_string());
        assert_eq!("22:6Δ4,7,10,13,16,19", format!("{display:#}"));
        assert_eq!("22:06Δ04,07,10,13,16,19", format!("{display:#02}"));

        Ok(())
    }
}

// mod system {
//     use super::*;

//     fn fatty_acid(bounds: impl IntoIterator<Item = &'static str>) -> PolarsResult<FattyAcid> {
//         super::fatty_acid(
//             bounds,
//             Options {
//                 kind: System,
//                 ..Default::default()
//             },
//         )
//     }

//     #[test]
//     #[rustfmt::skip]
//     fn system() -> PolarsResult<()> {
//         let c2u0 = fatty_acid(C2U0)?;
//         assert_eq!("c2u0", c2u0.to_string());
//         assert_eq!("c2u0", format!("{c2u0:#}"));
//         assert_eq!("c02u00", format!("{c2u0:02}"));

//         let c16u1dc9 = fatty_acid(C16U1DC9)?;
//         assert_eq!("c16u1dc9", c16u1dc9.to_string());
//         assert_eq!("c16u1dc9", format!("{c16u1dc9:#}"));
//         assert_eq!("c16u01dc09", format!("{c16u1dc9:02}"));

//         let c18u2dc9dc12 = fatty_acid(C18U2DC9DC12)?;
//         assert_eq!("c18u2dc9dc12", c18u2dc9dc12.to_string());
//         assert_eq!("c18u2dc9dc12", format!("{c18u2dc9dc12:#}"));
//         assert_eq!("c18u02dc09dc12", format!("{c18u2dc9dc12:02}"));

//         let c20u4dc8dc11dc14dc17 = fatty_acid(C20U4DC8DC11DC14DC17)?;
//         assert_eq!("c20u4dc8dc11dc14dc17", c20u4dc8dc11dc14dc17.to_string());
//         assert_eq!("c20u4dc8dc11dc14dc17", format!("{c20u4dc8dc11dc14dc17:#}"));
//         assert_eq!("c20u04dc08dc11dc14dc17", format!("{c20u4dc8dc11dc14dc17:02}"));

//         let c22u6dc4dc7dc10dc13dc16dc19 = fatty_acid(C22U6DC4DC7DC10DC13DC16DC19)?;
//         assert_eq!("c22u6dc4dc7dc10dc13dc16dc19", c22u6dc4dc7dc10dc13dc16dc19.to_string());
//         assert_eq!("c22u6dc4dc7dc10dc13dc16dc19", format!("{c22u6dc4dc7dc10dc13dc16dc19:#}"));
//         assert_eq!("c22u06dc04dc07dc10dc13dc16dc19", format!("{c22u6dc4dc7dc10dc13dc16dc19:02}"));

//         Ok(())
//     }
// }
