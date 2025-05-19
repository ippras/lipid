use lipid::prelude::*;
use polars::prelude::*;

macro_rules! check {
    ($identifier:ident) => {{
        let fatty_acid = FattyAcidChunked::try_from($identifier)?;
        assert_eq!(fatty_acid.to_string(), expected::$identifier);
    }};
}

#[test]
fn c4() -> PolarsResult<()> {
    check!(C4);
    Ok(())
}

#[test]
fn c5() -> PolarsResult<()> {
    check!(C5);
    Ok(())
}

#[test]
fn c6() -> PolarsResult<()> {
    check!(C6);
    Ok(())
}

#[test]
fn c7() -> PolarsResult<()> {
    check!(C7);
    Ok(())
}

#[test]
fn c8() -> PolarsResult<()> {
    check!(C8);
    Ok(())
}

#[test]
fn c9() -> PolarsResult<()> {
    check!(C9);
    Ok(())
}

#[test]
fn c10() -> PolarsResult<()> {
    check!(C10);
    Ok(())
}

#[test]
fn c11() -> PolarsResult<()> {
    check!(C11);
    Ok(())
}

#[test]
fn c12() -> PolarsResult<()> {
    check!(C12);
    Ok(())
}

#[test]
fn c13() -> PolarsResult<()> {
    check!(C13);
    Ok(())
}

#[test]
fn c14() -> PolarsResult<()> {
    check!(C14);
    Ok(())
}

#[test]
fn c15() -> PolarsResult<()> {
    check!(C15);
    Ok(())
}

#[test]
fn c16() -> PolarsResult<()> {
    check!(C16);
    Ok(())
}

#[test]
fn c16dc9() -> PolarsResult<()> {
    check!(C16DC9);
    Ok(())
}

#[test]
fn c16dt9() -> PolarsResult<()> {
    check!(C16DT9);
    Ok(())
}

#[test]
fn c17() -> PolarsResult<()> {
    check!(C17);
    Ok(())
}

#[test]
fn c18() -> PolarsResult<()> {
    check!(C18);
    Ok(())
}

#[test]
fn c18dc9() -> PolarsResult<()> {
    check!(C18DC9);
    Ok(())
}

#[test]
fn c18dt9() -> PolarsResult<()> {
    check!(C18DT9);
    Ok(())
}

#[test]
fn c18dc9dc12() -> PolarsResult<()> {
    check!(C18DC9DC12);
    Ok(())
}

#[test]
fn c18dc6dc9dc12() -> PolarsResult<()> {
    check!(C18DC6DC9DC12);
    Ok(())
}

#[test]
fn c18dc8dt10dc12() -> PolarsResult<()> {
    check!(C18DC8DT10DC12);
    Ok(())
}

#[test]
fn c18dc9dc12dc15() -> PolarsResult<()> {
    check!(C18DC9DC12DC15);
    Ok(())
}

#[test]
fn c18dc9dt11dt13() -> PolarsResult<()> {
    check!(C18DC9DT11DT13);
    Ok(())
}

#[test]
fn c18dt9dt11dc13() -> PolarsResult<()> {
    check!(C18DT9DT11DC13);
    Ok(())
}

#[test]
fn c18dt9dt11dt13() -> PolarsResult<()> {
    check!(C18DT9DT11DT13);
    Ok(())
}

#[test]
fn c18dc6dc9dc12dc15() -> PolarsResult<()> {
    check!(C18DC6DC9DC12DC15);
    Ok(())
}

#[test]
fn c19() -> PolarsResult<()> {
    check!(C19);
    Ok(())
}

#[test]
fn c20() -> PolarsResult<()> {
    check!(C20);
    Ok(())
}

#[test]
fn c20dc9() -> PolarsResult<()> {
    check!(C20DC9);
    Ok(())
}

#[test]
fn c20dc11() -> PolarsResult<()> {
    check!(C20DC11);
    Ok(())
}

#[test]
fn c20dc11dc14() -> PolarsResult<()> {
    check!(C20DC11DC14);
    Ok(())
}

#[test]
fn c20dc5dc8dc11() -> PolarsResult<()> {
    check!(C20DC5DC8DC11);
    Ok(())
}

#[test]
fn c20dc8dc11dc14() -> PolarsResult<()> {
    check!(C20DC8DC11DC14);
    Ok(())
}

#[test]
fn c20dc11dc14dc17() -> PolarsResult<()> {
    check!(C20DC11DC14DC17);
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14() -> PolarsResult<()> {
    check!(C20DC5DC8DC11DC14);
    Ok(())
}

#[test]
fn c20dc8dc11dc14dc17() -> PolarsResult<()> {
    check!(C20DC8DC11DC14DC17);
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14dc17() -> PolarsResult<()> {
    check!(C20DC5DC8DC11DC14DC17);
    Ok(())
}

#[test]
fn c21() -> PolarsResult<()> {
    check!(C21);
    Ok(())
}

#[test]
fn c22() -> PolarsResult<()> {
    check!(C22);
    Ok(())
}

#[test]
fn c22dc13() -> PolarsResult<()> {
    check!(C22DC13);
    Ok(())
}

#[test]
fn c22dc13dc16() -> PolarsResult<()> {
    check!(C22DC13DC16);
    Ok(())
}

#[test]
fn c22dc5dc13dc16() -> PolarsResult<()> {
    check!(C22DC5DC13DC16);
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16() -> PolarsResult<()> {
    check!(C22DC7DC10DC13DC16);
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check!(C22DC7DC10DC13DC16DC19);
    Ok(())
}

#[test]
fn c22dc4dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check!(C22DC4DC7DC10DC13DC16DC19);
    Ok(())
}

#[test]
fn c23() -> PolarsResult<()> {
    check!(C23);
    Ok(())
}

#[test]
fn c24() -> PolarsResult<()> {
    check!(C24);
    Ok(())
}

#[test]
fn c24dc15() -> PolarsResult<()> {
    check!(C24DC15);
    Ok(())
}

#[test]
fn c24dc15dc18() -> PolarsResult<()> {
    check!(C24DC15DC18);
    Ok(())
}

#[test]
fn c24dc12dc15dc18() -> PolarsResult<()> {
    check!(C24DC12DC15DC18);
    Ok(())
}

#[test]
fn c24dc9dc12dc15dc18() -> PolarsResult<()> {
    check!(C24DC9DC12DC15DC18);
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18() -> PolarsResult<()> {
    check!(C24DC6DC9DC12DC15DC18);
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18dc21() -> PolarsResult<()> {
    check!(C24DC6DC9DC12DC15DC18DC21);
    Ok(())
}

#[test]
fn c25() -> PolarsResult<()> {
    check!(C25);
    Ok(())
}

#[test]
fn c26() -> PolarsResult<()> {
    check!(C26);
    Ok(())
}

#[test]
fn c26dc17() -> PolarsResult<()> {
    check!(C26DC17);
    Ok(())
}

#[test]
fn c27() -> PolarsResult<()> {
    check!(C27);
    Ok(())
}

#[test]
fn c28() -> PolarsResult<()> {
    check!(C28);
    Ok(())
}

#[test]
fn c29() -> PolarsResult<()> {
    check!(C29);
    Ok(())
}

#[test]
fn c30() -> PolarsResult<()> {
    check!(C30);
    Ok(())
}

#[test]
fn c30dc21() -> PolarsResult<()> {
    check!(C30DC21);
    Ok(())
}

#[test]
fn c31() -> PolarsResult<()> {
    check!(C31);
    Ok(())
}

#[test]
fn c32() -> PolarsResult<()> {
    check!(C32);
    Ok(())
}

#[test]
fn c33() -> PolarsResult<()> {
    check!(C33);
    Ok(())
}

#[test]
fn c34() -> PolarsResult<()> {
    check!(C34);
    Ok(())
}

#[test]
fn c35() -> PolarsResult<()> {
    check!(C35);
    Ok(())
}

#[test]
fn c36() -> PolarsResult<()> {
    check!(C36);
    Ok(())
}

#[rustfmt::skip]
mod expected {
    pub(super) const C4: &str = "s1s2s3";
    pub(super) const C5: &str = "s1s2s3s4";
    pub(super) const C6: &str = "s1s2s3s4s5";
    pub(super) const C7: &str = "s1s2s3s4s5s6";
    pub(super) const C8: &str = "s1s2s3s4s5s6s7";
    pub(super) const C9: &str = "s1s2s3s4s5s6s7s8";
    pub(super) const C10: &str = "s1s2s3s4s5s6s7s8s9";
    pub(super) const C11: &str = "s1s2s3s4s5s6s7s8s9s10";
    pub(super) const C12: &str = "s1s2s3s4s5s6s7s8s9s10s11";
    pub(super) const C13: &str = "s1s2s3s4s5s6s7s8s9s10s11s12";
    pub(super) const C14: &str = "s1s2s3s4s5s6s7s8s9s10s11s12s13";
    pub(super) const C15: &str = "s1s2s3s4s5s6s7s8s9s10s11s12s13s14";
    pub(super) const C16: &str = "s1s2s3s4s5s6s7s8s9s10s11s12s13s14s15";
    pub(super) const C16DC9: &str = "s1s2s3s4s5s6s7s8dc9s10s11s12s13s14s15";
    pub(super) const C16DT9: &str = "s1s2s3s4s5s6s7s8dt9s10s11s12s13s14s15";
    pub(super) const C17: &str = "s1s2s3s4s5s6s7s8s9s10s11s12s13s14s15s16";
    pub(super) const C18: &str = "s1s2s3s4s5s6s7s8s9s10s11s12s13s14s15s16s17";
    pub(super) const C18DC9: &str = "s1s2s3s4s5s6s7s8dc9s10s11s12s13s14s15s16s17";
    pub(super) const C18DT9: &str = "s1s2s3s4s5s6s7s8dt9s10s11s12s13s14s15s16s17";
    pub(super) const C18DC9DC12: &str = "s1s2s3s4s5s6s7s8dc9s10s11dc12s13s14s15s16s17";
    pub(super) const C18DC6DC9DC12: &str = "s1s2s3s4s5dc6s7s8dc9s10s11dc12s13s14s15s16s17";
    pub(super) const C18DC8DT10DC12: &str = "s1s2s3s4s5s6s7dc8s9dt10s11dc12s13s14s15s16s17";
    pub(super) const C18DC9DC12DC15: &str = "s1s2s3s4s5s6s7s8dc9s10s11dc12s13s14dc15s16s17";
    pub(super) const C18DC9DT11DT13: &str = "s1s2s3s4s5s6s7s8dc9s10dt11s12dt13s14s15s16s17";
    pub(super) const C18DT9DT11DC13: &str = "s1s2s3s4s5s6s7s8dt9s10dt11s12dc13s14s15s16s17";
    pub(super) const C18DT9DT11DT13: &str = "s1s2s3s4s5s6s7s8dt9s10dt11s12dt13s14s15s16s17";
    pub(super) const C18DC6DC9DC12DC15: &str = "s1s2s3s4s5dc6s7s8dc9s10s11dc12s13s14dc15s16s17";
    pub(super) const C19: &str = "s1s2s3s4s5s6s7s8s9s10s11s12s13s14s15s16s17s18";
    pub(super) const C20: &str = "s1s2s3s4s5s6s7s8s9s10s11s12s13s14s15s16s17s18s19";
    pub(super) const C20DC9: &str = "s1s2s3s4s5s6s7s8dc9s10s11s12s13s14s15s16s17s18s19";
    pub(super) const C20DC11: &str = "s1s2s3s4s5s6s7s8s9s10dc11s12s13s14s15s16s17s18s19";
    pub(super) const C20DC11DC14: &str = "s1s2s3s4s5s6s7s8s9s10dc11s12s13dc14s15s16s17s18s19";
    pub(super) const C20DC5DC8DC11: &str = "s1s2s3s4dc5s6s7dc8s9s10dc11s12s13s14s15s16s17s18s19";
    pub(super) const C20DC8DC11DC14: &str = "s1s2s3s4s5s6s7dc8s9s10dc11s12s13dc14s15s16s17s18s19";
    pub(super) const C20DC11DC14DC17: &str = "s1s2s3s4s5s6s7s8s9s10dc11s12s13dc14s15s16dc17s18s19";
    pub(super) const C20DC5DC8DC11DC14: &str = "s1s2s3s4dc5s6s7dc8s9s10dc11s12s13dc14s15s16s17s18s19";
    pub(super) const C20DC8DC11DC14DC17: &str = "s1s2s3s4s5s6s7dc8s9s10dc11s12s13dc14s15s16dc17s18s19";
    pub(super) const C20DC5DC8DC11DC14DC17: &str = "s1s2s3s4dc5s6s7dc8s9s10dc11s12s13dc14s15s16dc17s18s19";
    pub(super) const C21: &str = "s1s2s3s4s5s6s7s8s9s10s11s12s13s14s15s16s17s18s19s20";
    pub(super) const C22: &str = "s1s2s3s4s5s6s7s8s9s10s11s12s13s14s15s16s17s18s19s20s21";
    pub(super) const C22DC13: &str = "s1s2s3s4s5s6s7s8s9s10s11s12dc13s14s15s16s17s18s19s20s21";
    pub(super) const C22DC13DC16: &str = "s1s2s3s4s5s6s7s8s9s10s11s12dc13s14s15dc16s17s18s19s20s21";
    pub(super) const C22DC5DC13DC16: &str = "s1s2s3s4dc5s6s7s8s9s10s11s12dc13s14s15dc16s17s18s19s20s21";
    pub(super) const C22DC7DC10DC13DC16: &str = "s1s2s3s4s5s6dc7s8s9dc10s11s12dc13s14s15dc16s17s18s19s20s21";
    pub(super) const C22DC7DC10DC13DC16DC19: &str = "s1s2s3s4s5s6dc7s8s9dc10s11s12dc13s14s15dc16s17s18dc19s20s21";
    pub(super) const C22DC4DC7DC10DC13DC16DC19: &str = "s1s2s3dc4s5s6dc7s8s9dc10s11s12dc13s14s15dc16s17s18dc19s20s21";
    pub(super) const C23: &str = "s1s2s3s4s5s6s7s8s9s10s11s12s13s14s15s16s17s18s19s20s21s22";
    pub(super) const C24: &str = "s1s2s3s4s5s6s7s8s9s10s11s12s13s14s15s16s17s18s19s20s21s22s23";
    pub(super) const C24DC15: &str = "s1s2s3s4s5s6s7s8s9s10s11s12s13s14dc15s16s17s18s19s20s21s22s23";
    pub(super) const C24DC15DC18: &str = "s1s2s3s4s5s6s7s8s9s10s11s12s13s14dc15s16s17dc18s19s20s21s22s23";
    pub(super) const C24DC12DC15DC18: &str = "s1s2s3s4s5s6s7s8s9s10s11dc12s13s14dc15s16s17dc18s19s20s21s22s23";
    pub(super) const C24DC9DC12DC15DC18: &str = "s1s2s3s4s5s6s7s8dc9s10s11dc12s13s14dc15s16s17dc18s19s20s21s22s23";
    pub(super) const C24DC6DC9DC12DC15DC18: &str = "s1s2s3s4s5dc6s7s8dc9s10s11dc12s13s14dc15s16s17dc18s19s20s21s22s23";
    pub(super) const C24DC6DC9DC12DC15DC18DC21: &str = "s1s2s3s4s5dc6s7s8dc9s10s11dc12s13s14dc15s16s17dc18s19s20dc21s22s23";
    pub(super) const C25: &str = "s1s2s3s4s5s6s7s8s9s10s11s12s13s14s15s16s17s18s19s20s21s22s23s24";
    pub(super) const C26: &str = "s1s2s3s4s5s6s7s8s9s10s11s12s13s14s15s16s17s18s19s20s21s22s23s24s25";
    pub(super) const C26DC17: &str = "s1s2s3s4s5s6s7s8s9s10s11s12s13s14s15s16dc17s18s19s20s21s22s23s24s25";
    pub(super) const C27: &str = "s1s2s3s4s5s6s7s8s9s10s11s12s13s14s15s16s17s18s19s20s21s22s23s24s25s26";
    pub(super) const C28: &str = "s1s2s3s4s5s6s7s8s9s10s11s12s13s14s15s16s17s18s19s20s21s22s23s24s25s26s27";
    pub(super) const C29: &str = "s1s2s3s4s5s6s7s8s9s10s11s12s13s14s15s16s17s18s19s20s21s22s23s24s25s26s27s28";
    pub(super) const C30: &str = "s1s2s3s4s5s6s7s8s9s10s11s12s13s14s15s16s17s18s19s20s21s22s23s24s25s26s27s28s29";
    pub(super) const C30DC21: &str = "s1s2s3s4s5s6s7s8s9s10s11s12s13s14s15s16s17s18s19s20dc21s22s23s24s25s26s27s28s29";
    pub(super) const C31: &str = "s1s2s3s4s5s6s7s8s9s10s11s12s13s14s15s16s17s18s19s20s21s22s23s24s25s26s27s28s29s30";
    pub(super) const C32: &str = "s1s2s3s4s5s6s7s8s9s10s11s12s13s14s15s16s17s18s19s20s21s22s23s24s25s26s27s28s29s30s31";
    pub(super) const C33: &str = "s1s2s3s4s5s6s7s8s9s10s11s12s13s14s15s16s17s18s19s20s21s22s23s24s25s26s27s28s29s30s31s32";
    pub(super) const C34: &str = "s1s2s3s4s5s6s7s8s9s10s11s12s13s14s15s16s17s18s19s20s21s22s23s24s25s26s27s28s29s30s31s32s33";
    pub(super) const C35: &str = "s1s2s3s4s5s6s7s8s9s10s11s12s13s14s15s16s17s18s19s20s21s22s23s24s25s26s27s28s29s30s31s32s33s34";
    pub(super) const C36: &str = "s1s2s3s4s5s6s7s8s9s10s11s12s13s14s15s16s17s18s19s20s21s22s23s24s25s26s27s28s29s30s31s32s33s34s35";
}
