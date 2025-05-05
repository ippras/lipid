use super::*;

fn check_mass(fatty_acid: FattyAcidChunked, expected: f64) {
    let mass = round_to_decimals(fatty_acid.rcoo().mass(None), DECIMALS);
    assert!((mass - expected).abs() < f64::EPSILON);
}

#[test]
fn c4() -> PolarsResult<()> {
    check_mass(fatty_acid(&C4)?, expected::C4);
    Ok(())
}

#[test]
fn c5() -> PolarsResult<()> {
    check_mass(fatty_acid(&C5)?, expected::C5);
    Ok(())
}

#[test]
fn c6() -> PolarsResult<()> {
    check_mass(fatty_acid(&C6)?, expected::C6);
    Ok(())
}

#[test]
fn c7() -> PolarsResult<()> {
    check_mass(fatty_acid(&C7)?, expected::C7);
    Ok(())
}

#[test]
fn c8() -> PolarsResult<()> {
    check_mass(fatty_acid(&C8)?, expected::C8);
    Ok(())
}

#[test]
fn c9() -> PolarsResult<()> {
    check_mass(fatty_acid(&C9)?, expected::C9);
    Ok(())
}

#[test]
fn c10() -> PolarsResult<()> {
    check_mass(fatty_acid(&C10)?, expected::C10);
    Ok(())
}

#[test]
fn c11() -> PolarsResult<()> {
    check_mass(fatty_acid(&C11)?, expected::C11);
    Ok(())
}

#[test]
fn c12() -> PolarsResult<()> {
    check_mass(fatty_acid(&C12)?, expected::C12);
    Ok(())
}

#[test]
fn c13() -> PolarsResult<()> {
    check_mass(fatty_acid(&C13)?, expected::C13);
    Ok(())
}

#[test]
fn c14() -> PolarsResult<()> {
    check_mass(fatty_acid(&C14)?, expected::C14);
    Ok(())
}

#[test]
fn c15() -> PolarsResult<()> {
    check_mass(fatty_acid(&C15)?, expected::C15);
    Ok(())
}

#[test]
fn c16() -> PolarsResult<()> {
    check_mass(fatty_acid(&C16)?, expected::C16);
    Ok(())
}

#[test]
fn c16dc9() -> PolarsResult<()> {
    check_mass(fatty_acid(&C16DC9)?, expected::C16DC9);
    Ok(())
}

#[test]
fn c16dt9() -> PolarsResult<()> {
    check_mass(fatty_acid(&C16DT9)?, expected::C16DT9);
    Ok(())
}

#[test]
fn c17() -> PolarsResult<()> {
    check_mass(fatty_acid(&C17)?, expected::C17);
    Ok(())
}

#[test]
fn c18() -> PolarsResult<()> {
    check_mass(fatty_acid(&C18)?, expected::C18);
    Ok(())
}

#[test]
fn c18dc9() -> PolarsResult<()> {
    check_mass(fatty_acid(&C18DC9)?, expected::C18DC9);
    Ok(())
}

#[test]
fn c18dt9() -> PolarsResult<()> {
    check_mass(fatty_acid(&C18DT9)?, expected::C18DT9);
    Ok(())
}

#[test]
fn c18dc9dc12() -> PolarsResult<()> {
    check_mass(fatty_acid(&C18DC9DC12)?, expected::C18DC9DC12);
    Ok(())
}

#[test]
fn c18dc6dc9dc12() -> PolarsResult<()> {
    check_mass(fatty_acid(&C18DC6DC9DC12)?, expected::C18DC6DC9DC12);
    Ok(())
}

#[test]
fn c18dc8dt10dc12() -> PolarsResult<()> {
    check_mass(fatty_acid(&C18DC8DT10DC12)?, expected::C18DC8DT10DC12);
    Ok(())
}

#[test]
fn c18dc9dc12dc15() -> PolarsResult<()> {
    check_mass(fatty_acid(&C18DC9DC12DC15)?, expected::C18DC9DC12DC15);
    Ok(())
}

#[test]
fn c18dc9dt11dt13() -> PolarsResult<()> {
    check_mass(fatty_acid(&C18DC9DT11DT13)?, expected::C18DC9DT11DT13);
    Ok(())
}

#[test]
fn c18dt9dt11dc13() -> PolarsResult<()> {
    check_mass(fatty_acid(&C18DT9DT11DC13)?, expected::C18DT9DT11DC13);
    Ok(())
}

#[test]
fn c18dt9dt11dt13() -> PolarsResult<()> {
    check_mass(fatty_acid(&C18DT9DT11DT13)?, expected::C18DT9DT11DT13);
    Ok(())
}

#[test]
fn c18dc6dc9dc12dc15() -> PolarsResult<()> {
    check_mass(fatty_acid(&C18DC6DC9DC12DC15)?, expected::C18DC6DC9DC12DC15);
    Ok(())
}

#[test]
fn c19() -> PolarsResult<()> {
    check_mass(fatty_acid(&C19)?, expected::C19);
    Ok(())
}

#[test]
fn c20() -> PolarsResult<()> {
    check_mass(fatty_acid(&C20)?, expected::C20);
    Ok(())
}

#[test]
fn c20dc9() -> PolarsResult<()> {
    check_mass(fatty_acid(&C20DC9)?, expected::C20DC9);
    Ok(())
}

#[test]
fn c20dc11() -> PolarsResult<()> {
    check_mass(fatty_acid(&C20DC11)?, expected::C20DC11);
    Ok(())
}

#[test]
fn c20dc11dc14() -> PolarsResult<()> {
    check_mass(fatty_acid(&C20DC11DC14)?, expected::C20DC11DC14);
    Ok(())
}

#[test]
fn c20dc5dc8dc11() -> PolarsResult<()> {
    check_mass(fatty_acid(&C20DC5DC8DC11)?, expected::C20DC5DC8DC11);
    Ok(())
}

#[test]
fn c20dc8dc11dc14() -> PolarsResult<()> {
    check_mass(fatty_acid(&C20DC8DC11DC14)?, expected::C20DC8DC11DC14);
    Ok(())
}

#[test]
fn c20dc11dc14dc17() -> PolarsResult<()> {
    check_mass(fatty_acid(&C20DC11DC14DC17)?, expected::C20DC11DC14DC17);
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14() -> PolarsResult<()> {
    check_mass(fatty_acid(&C20DC5DC8DC11DC14)?, expected::C20DC5DC8DC11DC14);
    Ok(())
}

#[test]
fn c20dc8dc11dc14dc17() -> PolarsResult<()> {
    check_mass(
        fatty_acid(&C20DC8DC11DC14DC17)?,
        expected::C20DC8DC11DC14DC17,
    );
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14dc17() -> PolarsResult<()> {
    check_mass(
        fatty_acid(&C20DC5DC8DC11DC14DC17)?,
        expected::C20DC5DC8DC11DC14DC17,
    );
    Ok(())
}

#[test]
fn c21() -> PolarsResult<()> {
    check_mass(fatty_acid(&C21)?, expected::C21);
    Ok(())
}

#[test]
fn c22() -> PolarsResult<()> {
    check_mass(fatty_acid(&C22)?, expected::C22);
    Ok(())
}

#[test]
fn c22dc13() -> PolarsResult<()> {
    check_mass(fatty_acid(&C22DC13)?, expected::C22DC13);
    Ok(())
}

#[test]
fn c22dc13dc16() -> PolarsResult<()> {
    check_mass(fatty_acid(&C22DC13DC16)?, expected::C22DC13DC16);
    Ok(())
}

#[test]
fn c22dc5dc13dc16() -> PolarsResult<()> {
    check_mass(fatty_acid(&C22DC5DC13DC16)?, expected::C22DC5DC13DC16);
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16() -> PolarsResult<()> {
    check_mass(
        fatty_acid(&C22DC7DC10DC13DC16)?,
        expected::C22DC7DC10DC13DC16,
    );
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check_mass(
        fatty_acid(&C22DC7DC10DC13DC16DC19)?,
        expected::C22DC7DC10DC13DC16DC19,
    );
    Ok(())
}

#[test]
fn c22dc4dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check_mass(
        fatty_acid(&C22DC4DC7DC10DC13DC16DC19)?,
        expected::C22DC4DC7DC10DC13DC16DC19,
    );
    Ok(())
}

#[test]
fn c23() -> PolarsResult<()> {
    check_mass(fatty_acid(&C23)?, expected::C23);
    Ok(())
}

#[test]
fn c24() -> PolarsResult<()> {
    check_mass(fatty_acid(&C24)?, expected::C24);
    Ok(())
}

#[test]
fn c24dc15() -> PolarsResult<()> {
    check_mass(fatty_acid(&C24DC15)?, expected::C24DC15);
    Ok(())
}

#[test]
fn c24dc15dc18() -> PolarsResult<()> {
    check_mass(fatty_acid(&C24DC15DC18)?, expected::C24DC15DC18);
    Ok(())
}

#[test]
fn c24dc12dc15dc18() -> PolarsResult<()> {
    check_mass(fatty_acid(&C24DC12DC15DC18)?, expected::C24DC12DC15DC18);
    Ok(())
}

#[test]
fn c24dc9dc12dc15dc18() -> PolarsResult<()> {
    check_mass(
        fatty_acid(&C24DC9DC12DC15DC18)?,
        expected::C24DC9DC12DC15DC18,
    );
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18() -> PolarsResult<()> {
    check_mass(
        fatty_acid(&C24DC6DC9DC12DC15DC18)?,
        expected::C24DC6DC9DC12DC15DC18,
    );
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18dc21() -> PolarsResult<()> {
    check_mass(
        fatty_acid(&C24DC6DC9DC12DC15DC18DC21)?,
        expected::C24DC6DC9DC12DC15DC18DC21,
    );
    Ok(())
}

#[test]
fn c25() -> PolarsResult<()> {
    check_mass(fatty_acid(&C25)?, expected::C25);
    Ok(())
}

#[test]
fn c26() -> PolarsResult<()> {
    check_mass(fatty_acid(&C26)?, expected::C26);
    Ok(())
}

#[test]
fn c26dc17() -> PolarsResult<()> {
    check_mass(fatty_acid(&C26DC17)?, expected::C26DC17);
    Ok(())
}

#[test]
fn c27() -> PolarsResult<()> {
    check_mass(fatty_acid(&C27)?, expected::C27);
    Ok(())
}

#[test]
fn c28() -> PolarsResult<()> {
    check_mass(fatty_acid(&C28)?, expected::C28);
    Ok(())
}

#[test]
fn c29() -> PolarsResult<()> {
    check_mass(fatty_acid(&C29)?, expected::C29);
    Ok(())
}

#[test]
fn c30() -> PolarsResult<()> {
    check_mass(fatty_acid(&C30)?, expected::C30);
    Ok(())
}

#[test]
fn c30dc21() -> PolarsResult<()> {
    check_mass(fatty_acid(&C30DC21)?, expected::C30DC21);
    Ok(())
}

#[test]
fn c31() -> PolarsResult<()> {
    check_mass(fatty_acid(&C31)?, expected::C31);
    Ok(())
}

#[test]
fn c32() -> PolarsResult<()> {
    check_mass(fatty_acid(&C32)?, expected::C32);
    Ok(())
}

#[test]
fn c33() -> PolarsResult<()> {
    check_mass(fatty_acid(&C33)?, expected::C33);
    Ok(())
}

#[test]
fn c34() -> PolarsResult<()> {
    check_mass(fatty_acid(&C34)?, expected::C34);
    Ok(())
}

#[test]
fn c35() -> PolarsResult<()> {
    check_mass(fatty_acid(&C35)?, expected::C35);
    Ok(())
}

#[test]
fn c36() -> PolarsResult<()> {
    check_mass(fatty_acid(&C36)?, expected::C36);
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
