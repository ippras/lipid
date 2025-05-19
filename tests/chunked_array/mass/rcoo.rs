use super::*;

macro_rules! check {
    ($identifier:ident) => {{
        let fatty_acid = FattyAcidChunked::try_from($identifier)?;
        let mass = round_to_decimals(fatty_acid.rcoo().mass(None), DECIMALS);
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
    pub(super) const C4: f64 = 87.0446;
    pub(super) const C5: f64 = 101.0603;
    pub(super) const C6: f64 = 115.0759;
    pub(super) const C7: f64 = 129.0916;
    pub(super) const C8: f64 = 143.1072;
    pub(super) const C9: f64 = 157.1229;
    pub(super) const C10: f64 = 171.1385;
    pub(super) const C11: f64 = 185.1542;
    pub(super) const C12: f64 = 199.1698;
    pub(super) const C13: f64 = 213.1855;
    pub(super) const C14: f64 = 227.2011;
    pub(super) const C15: f64 = 241.2168;
    pub(super) const C16: f64 = 255.2324;
    pub(super) const C16DC9: f64 = 253.2168;
    pub(super) const C16DT9: f64 = 253.2168;
    pub(super) const C17: f64 = 269.2481;
    pub(super) const C18: f64 = 283.2637;
    pub(super) const C18DC9: f64 = 281.2481;
    pub(super) const C18DT9: f64 = 281.2481;
    pub(super) const C18DC9DC12: f64 = 279.2324;
    pub(super) const C18DC6DC9DC12: f64 = 277.2168;
    pub(super) const C18DC8DT10DC12: f64 = 277.2168;
    pub(super) const C18DC9DC12DC15: f64 = 277.2168;
    pub(super) const C18DC9DT11DT13: f64 = 277.2168;
    pub(super) const C18DT9DT11DC13: f64 = 277.2168;
    pub(super) const C18DT9DT11DT13: f64 = 277.2168;
    pub(super) const C18DC6DC9DC12DC15: f64 = 275.2011;
    pub(super) const C19: f64 = 297.2794;
    pub(super) const C20: f64 = 311.2950;
    pub(super) const C20DC9: f64 = 309.2794;
    pub(super) const C20DC11: f64 = 309.2794;
    pub(super) const C20DC11DC14: f64 = 307.2637;
    pub(super) const C20DC5DC8DC11: f64 = 305.2481;
    pub(super) const C20DC8DC11DC14: f64 = 305.2481;
    pub(super) const C20DC11DC14DC17: f64 = 305.2481;
    pub(super) const C20DC5DC8DC11DC14: f64 = 303.2324;
    pub(super) const C20DC8DC11DC14DC17: f64 = 303.2324;
    pub(super) const C20DC5DC8DC11DC14DC17: f64 = 301.2168;
    pub(super) const C21: f64 = 325.3107;
    pub(super) const C22: f64 = 339.3263;
    pub(super) const C22DC13: f64 = 337.3107;
    pub(super) const C22DC13DC16: f64 = 335.2950;
    pub(super) const C22DC5DC13DC16: f64 = 333.2794;
    pub(super) const C22DC7DC10DC13DC16: f64 = 331.2637;
    pub(super) const C22DC7DC10DC13DC16DC19: f64 = 329.2481;
    pub(super) const C22DC4DC7DC10DC13DC16DC19: f64 = 327.2324;
    pub(super) const C23: f64 = 353.3420;
    pub(super) const C24: f64 = 367.3576;
    pub(super) const C24DC15: f64 = 365.3420;
    pub(super) const C24DC15DC18: f64 = 363.3263;
    pub(super) const C24DC12DC15DC18: f64 = 361.3107;
    pub(super) const C24DC9DC12DC15DC18: f64 = 359.2950;
    pub(super) const C24DC6DC9DC12DC15DC18: f64 = 357.2794;
    pub(super) const C24DC6DC9DC12DC15DC18DC21: f64 = 355.2637;
    pub(super) const C25: f64 = 381.3733;
    pub(super) const C26: f64 = 395.3889;
    pub(super) const C26DC17: f64 = 393.3733;
    pub(super) const C27: f64 = 409.4046;
    pub(super) const C28: f64 = 423.4202;
    pub(super) const C29: f64 = 437.4359;
    pub(super) const C30: f64 = 451.4515;
    pub(super) const C30DC21: f64 = 449.4359;
    pub(super) const C31: f64 = 465.4672;
    pub(super) const C32: f64 = 479.4828;
    pub(super) const C33: f64 = 493.4985;
    pub(super) const C34: f64 = 507.5141;
    pub(super) const C35: f64 = 521.5298;
    pub(super) const C36: f64 = 535.5454;
}
