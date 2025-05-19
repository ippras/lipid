use super::{DECIMALS, round_to_decimals};
use lipid::prelude::*;
use polars::prelude::*;

macro_rules! check {
    ($identifier:ident) => {{
        let fatty_acid = FattyAcidChunked::try_from($identifier)?;
        let mass = round_to_decimals(fatty_acid.mass(None), DECIMALS);
        assert!((mass - expected::$identifier).abs() < f64::EPSILON);
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

mod expected {
    pub(super) const C4: f64 = 88.0524;
    pub(super) const C5: f64 = 102.0681;
    pub(super) const C6: f64 = 116.0837;
    pub(super) const C7: f64 = 130.0994;
    pub(super) const C8: f64 = 144.1150;
    pub(super) const C9: f64 = 158.1307;
    pub(super) const C10: f64 = 172.1463;
    pub(super) const C11: f64 = 186.1620;
    pub(super) const C12: f64 = 200.1776;
    pub(super) const C13: f64 = 214.1933;
    pub(super) const C14: f64 = 228.2089;
    pub(super) const C15: f64 = 242.2246;
    pub(super) const C16: f64 = 256.2402;
    pub(super) const C16DC9: f64 = 254.2246;
    pub(super) const C16DT9: f64 = 254.2246;
    pub(super) const C17: f64 = 270.2559;
    pub(super) const C18: f64 = 284.2715;
    pub(super) const C18DC9: f64 = 282.2559;
    pub(super) const C18DT9: f64 = 282.2559;
    pub(super) const C18DC9DC12: f64 = 280.2402;
    pub(super) const C18DC6DC9DC12: f64 = 278.2246;
    pub(super) const C18DC8DT10DC12: f64 = 278.2246;
    pub(super) const C18DC9DC12DC15: f64 = 278.2246;
    pub(super) const C18DC9DT11DT13: f64 = 278.2246;
    pub(super) const C18DT9DT11DC13: f64 = 278.2246;
    pub(super) const C18DT9DT11DT13: f64 = 278.2246;
    pub(super) const C18DC6DC9DC12DC15: f64 = 276.2089;
    pub(super) const C19: f64 = 298.2872;
    pub(super) const C20: f64 = 312.3028;
    pub(super) const C20DC9: f64 = 310.2872;
    pub(super) const C20DC11: f64 = 310.2872;
    pub(super) const C20DC11DC14: f64 = 308.2715;
    pub(super) const C20DC5DC8DC11: f64 = 306.2559;
    pub(super) const C20DC8DC11DC14: f64 = 306.2559;
    pub(super) const C20DC11DC14DC17: f64 = 306.2559;
    pub(super) const C20DC5DC8DC11DC14: f64 = 304.2402;
    pub(super) const C20DC8DC11DC14DC17: f64 = 304.2402;
    pub(super) const C20DC5DC8DC11DC14DC17: f64 = 302.2246;
    pub(super) const C21: f64 = 326.3185;
    pub(super) const C22: f64 = 340.3341;
    pub(super) const C22DC13: f64 = 338.3185;
    pub(super) const C22DC13DC16: f64 = 336.3028;
    pub(super) const C22DC5DC13DC16: f64 = 334.2872;
    pub(super) const C22DC7DC10DC13DC16: f64 = 332.2715;
    pub(super) const C22DC7DC10DC13DC16DC19: f64 = 330.2559;
    pub(super) const C22DC4DC7DC10DC13DC16DC19: f64 = 328.2402;
    pub(super) const C23: f64 = 354.3498;
    pub(super) const C24: f64 = 368.3654;
    pub(super) const C24DC15: f64 = 366.3498;
    pub(super) const C24DC15DC18: f64 = 364.3341;
    pub(super) const C24DC12DC15DC18: f64 = 362.3185;
    pub(super) const C24DC9DC12DC15DC18: f64 = 360.3028;
    pub(super) const C24DC6DC9DC12DC15DC18: f64 = 358.2872;
    pub(super) const C24DC6DC9DC12DC15DC18DC21: f64 = 356.2715;
    pub(super) const C25: f64 = 382.3811;
    pub(super) const C26: f64 = 396.3967;
    pub(super) const C26DC17: f64 = 394.3811;
    pub(super) const C27: f64 = 410.4124;
    pub(super) const C28: f64 = 424.4280;
    pub(super) const C29: f64 = 438.4437;
    pub(super) const C30: f64 = 452.4593;
    pub(super) const C30DC21: f64 = 450.4437;
    pub(super) const C31: f64 = 466.4750;
    pub(super) const C32: f64 = 480.4906;
    pub(super) const C33: f64 = 494.5063;
    pub(super) const C34: f64 = 508.5219;
    pub(super) const C35: f64 = 522.5376;
    pub(super) const C36: f64 = 536.5532;
}

mod rco;
mod rcoo;
mod rcooch3;
mod rcooh;
