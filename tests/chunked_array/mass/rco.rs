use super::*;

macro_rules! check {
    ($identifier:ident) => {{
        let fatty_acid = FattyAcidChunked::try_from($identifier)?;
        let mass = round_to_decimals(fatty_acid.rco().mass(None), DECIMALS);
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
fn c16u1dc9() -> PolarsResult<()> {
    check!(C16DC9);
    Ok(())
}

#[test]
fn c16u1dt9() -> PolarsResult<()> {
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
fn c18u1dc9() -> PolarsResult<()> {
    check!(C18DC9);
    Ok(())
}

#[test]
fn c18u1dt9() -> PolarsResult<()> {
    check!(C18DT9);
    Ok(())
}

#[test]
fn c18u2dc9dc12() -> PolarsResult<()> {
    check!(C18DC9DC12);
    Ok(())
}

#[test]
fn c18u3dc6dc9dc12() -> PolarsResult<()> {
    check!(C18DC6DC9DC12);
    Ok(())
}

#[test]
fn c18u3dc8dt10dc12() -> PolarsResult<()> {
    check!(C18DC8DT10DC12);
    Ok(())
}

#[test]
fn c18u3dc9dc12dc15() -> PolarsResult<()> {
    check!(C18DC9DC12DC15);
    Ok(())
}

#[test]
fn c18u3dc9dt11dt13() -> PolarsResult<()> {
    check!(C18DC9DT11DT13);
    Ok(())
}

#[test]
fn c18u3dt9dt11dc13() -> PolarsResult<()> {
    check!(C18DT9DT11DC13);
    Ok(())
}

#[test]
fn c18u3dt9dt11dt13() -> PolarsResult<()> {
    check!(C18DT9DT11DT13);
    Ok(())
}

#[test]
fn c18u4dc6dc9dc12dc15() -> PolarsResult<()> {
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
fn c20u1dc9() -> PolarsResult<()> {
    check!(C20DC9);
    Ok(())
}

#[test]
fn c20u1dc11() -> PolarsResult<()> {
    check!(C20DC11);
    Ok(())
}

#[test]
fn c20u2dc11dc14() -> PolarsResult<()> {
    check!(C20DC11DC14);
    Ok(())
}

#[test]
fn c20u3dc5dc8dc11() -> PolarsResult<()> {
    check!(C20DC5DC8DC11);
    Ok(())
}

#[test]
fn c20u3dc8dc11dc14() -> PolarsResult<()> {
    check!(C20DC8DC11DC14);
    Ok(())
}

#[test]
fn c20u3dc11dc14dc17() -> PolarsResult<()> {
    check!(C20DC11DC14DC17);
    Ok(())
}

#[test]
fn c20u4dc5dc8dc11dc14() -> PolarsResult<()> {
    check!(C20DC5DC8DC11DC14);
    Ok(())
}

#[test]
fn c20u4dc8dc11dc14dc17() -> PolarsResult<()> {
    check!(C20DC8DC11DC14DC17);
    Ok(())
}

#[test]
fn c20u5dc5dc8dc11dc14dc17() -> PolarsResult<()> {
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
fn c22u1dc13() -> PolarsResult<()> {
    check!(C22DC13);
    Ok(())
}

#[test]
fn c22u2dc13dc16() -> PolarsResult<()> {
    check!(C22DC13DC16);
    Ok(())
}

#[test]
fn c22u3dc5dc13dc16() -> PolarsResult<()> {
    check!(C22DC5DC13DC16);
    Ok(())
}

#[test]
fn c22u4dc7dc10dc13dc16() -> PolarsResult<()> {
    check!(C22DC7DC10DC13DC16);
    Ok(())
}

#[test]
fn c22u5dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check!(C22DC7DC10DC13DC16DC19);
    Ok(())
}

#[test]
fn c22u6dc4dc7dc10dc13dc16dc19() -> PolarsResult<()> {
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
fn c24u1dc15() -> PolarsResult<()> {
    check!(C24DC15);
    Ok(())
}

#[test]
fn c24u2dc15dc18() -> PolarsResult<()> {
    check!(C24DC15DC18);
    Ok(())
}

#[test]
fn c24u3dc12dc15dc18() -> PolarsResult<()> {
    check!(C24DC12DC15DC18);
    Ok(())
}

#[test]
fn c24u4dc9dc12dc15dc18() -> PolarsResult<()> {
    check!(C24DC9DC12DC15DC18);
    Ok(())
}

#[test]
fn c24u5dc6dc9dc12dc15dc18() -> PolarsResult<()> {
    check!(C24DC6DC9DC12DC15DC18);
    Ok(())
}

#[test]
fn c24u6dc6dc9dc12dc15dc18dc21() -> PolarsResult<()> {
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
fn c26u1dc17() -> PolarsResult<()> {
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
fn c30u1dc21() -> PolarsResult<()> {
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
    pub(super) const C4: f64 = 71.0497;
    pub(super) const C5: f64 = 85.0653;
    pub(super) const C6: f64 = 99.0810;
    pub(super) const C7: f64 = 113.0966;
    pub(super) const C8: f64 = 127.1123;
    pub(super) const C9: f64 = 141.1279;
    pub(super) const C10: f64 = 155.1436;
    pub(super) const C11: f64 = 169.1592;
    pub(super) const C12: f64 = 183.1749;
    pub(super) const C13: f64 = 197.1905;
    pub(super) const C14: f64 = 211.2062;
    pub(super) const C15: f64 = 225.2218;
    pub(super) const C16: f64 = 239.2375;
    pub(super) const C16DC9: f64 = 237.2218;
    pub(super) const C16DT9: f64 = 237.2218;
    pub(super) const C17: f64 = 253.2531;
    pub(super) const C18: f64 = 267.2688;
    pub(super) const C18DC9: f64 = 265.2531;
    pub(super) const C18DT9: f64 = 265.2531;
    pub(super) const C18DC9DC12: f64 = 263.2375;
    pub(super) const C18DC6DC9DC12: f64 = 261.2218;
    pub(super) const C18DC8DT10DC12: f64 = 261.2218;
    pub(super) const C18DC9DC12DC15: f64 = 261.2218;
    pub(super) const C18DC9DT11DT13: f64 = 261.2218;
    pub(super) const C18DT9DT11DC13: f64 = 261.2218;
    pub(super) const C18DT9DT11DT13: f64 = 261.2218;
    pub(super) const C18DC6DC9DC12DC15: f64 = 259.2062;
    pub(super) const C19: f64 = 281.2844;
    pub(super) const C20: f64 = 295.3001;
    pub(super) const C20DC9: f64 = 293.2844;
    pub(super) const C20DC11: f64 = 293.2844;
    pub(super) const C20DC11DC14: f64 = 291.2688;
    pub(super) const C20DC5DC8DC11: f64 = 289.2531;
    pub(super) const C20DC8DC11DC14: f64 = 289.2531;
    pub(super) const C20DC11DC14DC17: f64 = 289.2531;
    pub(super) const C20DC5DC8DC11DC14: f64 = 287.2375;
    pub(super) const C20DC8DC11DC14DC17: f64 = 287.2375;
    pub(super) const C20DC5DC8DC11DC14DC17: f64 = 285.2218;
    pub(super) const C21: f64 = 309.3157;
    pub(super) const C22: f64 = 323.3314;
    pub(super) const C22DC13: f64 = 321.3157;
    pub(super) const C22DC13DC16: f64 = 319.3001;
    pub(super) const C22DC5DC13DC16: f64 = 317.2844;
    pub(super) const C22DC7DC10DC13DC16: f64 = 315.2688;
    pub(super) const C22DC7DC10DC13DC16DC19: f64 = 313.2531;
    pub(super) const C22DC4DC7DC10DC13DC16DC19: f64 = 311.2375;
    pub(super) const C23: f64 = 337.3470;
    pub(super) const C24: f64 = 351.3627;
    pub(super) const C24DC15: f64 = 349.3470;
    pub(super) const C24DC15DC18: f64 = 347.3314;
    pub(super) const C24DC12DC15DC18: f64 = 345.3157;
    pub(super) const C24DC9DC12DC15DC18: f64 = 343.3001;
    pub(super) const C24DC6DC9DC12DC15DC18: f64 = 341.2844;
    pub(super) const C24DC6DC9DC12DC15DC18DC21: f64 = 339.2688;
    pub(super) const C25: f64 = 365.3783;
    pub(super) const C26: f64 = 379.3940;
    pub(super) const C26DC17: f64 = 377.3783;
    pub(super) const C27: f64 = 393.4096;
    pub(super) const C28: f64 = 407.4253;
    pub(super) const C29: f64 = 421.4409;
    pub(super) const C30: f64 = 435.4566;
    pub(super) const C30DC21: f64 = 433.4409;
    pub(super) const C31: f64 = 449.4722;
    pub(super) const C32: f64 = 463.4879;
    pub(super) const C33: f64 = 477.5035;
    pub(super) const C34: f64 = 491.5192;
    pub(super) const C35: f64 = 505.5348;
    pub(super) const C36: f64 = 519.5505;
}
