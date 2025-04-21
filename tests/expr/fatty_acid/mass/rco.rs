use super::fatty_acid_data_frame;
use lipid::prelude::*;
use polars::prelude::*;

fn check_mass(data_frame: DataFrame, expected: f64) -> PolarsResult<()> {
    let data_frame = data_frame
        .lazy()
        .select([col("FattyAcid").fatty_acid().rco().mass(None).round(4)])
        .collect()?;
    let mass = data_frame["Mass"].f64()?.get(0).unwrap();
    assert!((mass - expected).abs() < f64::EPSILON);
    Ok(())
}

#[test]
fn c4() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C4)?, expected::C4)?;
    Ok(())
}

#[test]
fn c5() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C5)?, expected::C5)?;
    Ok(())
}

#[test]
fn c6() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C6)?, expected::C6)?;
    Ok(())
}

#[test]
fn c7() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C7)?, expected::C7)?;
    Ok(())
}

#[test]
fn c8() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C8)?, expected::C8)?;
    Ok(())
}

#[test]
fn c9() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C9)?, expected::C9)?;
    Ok(())
}

#[test]
fn c10() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C10)?, expected::C10)?;
    Ok(())
}

#[test]
fn c11() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C11)?, expected::C11)?;
    Ok(())
}

#[test]
fn c12() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C12)?, expected::C12)?;
    Ok(())
}

#[test]
fn c13() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C13)?, expected::C13)?;
    Ok(())
}

#[test]
fn c14() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C14)?, expected::C14)?;
    Ok(())
}

#[test]
fn c15() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C15)?, expected::C15)?;
    Ok(())
}

#[test]
fn c16() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C16)?, expected::C16)?;
    Ok(())
}

#[test]
fn c16dc9() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C16DC9)?, expected::C16DC9)?;
    Ok(())
}

#[test]
fn c16dt9() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C16DT9)?, expected::C16DT9)?;
    Ok(())
}

#[test]
fn c17() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C17)?, expected::C17)?;
    Ok(())
}

#[test]
fn c18() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C18)?, expected::C18)?;
    Ok(())
}

#[test]
fn c18dc9() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C18DC9)?, expected::C18DC9)?;
    Ok(())
}

#[test]
fn c18dt9() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C18DT9)?, expected::C18DT9)?;
    Ok(())
}

#[test]
fn c18dc9dc12() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C18DC9DC12)?, expected::C18DC9DC12)?;
    Ok(())
}

#[test]
fn c18dc6dc9dc12() -> PolarsResult<()> {
    check_mass(
        fatty_acid_data_frame(&C18DC6DC9DC12)?,
        expected::C18DC6DC9DC12,
    )?;
    Ok(())
}

#[test]
fn c18dc8dt10dc12() -> PolarsResult<()> {
    check_mass(
        fatty_acid_data_frame(&C18DC8DT10DC12)?,
        expected::C18DC8DT10DC12,
    )?;
    Ok(())
}

#[test]
fn c18dc9dc12dc15() -> PolarsResult<()> {
    check_mass(
        fatty_acid_data_frame(&C18DC9DC12DC15)?,
        expected::C18DC9DC12DC15,
    )?;
    Ok(())
}

#[test]
fn c18dc9dt11dt13() -> PolarsResult<()> {
    check_mass(
        fatty_acid_data_frame(&C18DC9DT11DT13)?,
        expected::C18DC9DT11DT13,
    )?;
    Ok(())
}

#[test]
fn c18dt9dt11dc13() -> PolarsResult<()> {
    check_mass(
        fatty_acid_data_frame(&C18DT9DT11DC13)?,
        expected::C18DT9DT11DC13,
    )?;
    Ok(())
}

#[test]
fn c18dt9dt11dt13() -> PolarsResult<()> {
    check_mass(
        fatty_acid_data_frame(&C18DT9DT11DT13)?,
        expected::C18DT9DT11DT13,
    )?;
    Ok(())
}

#[test]
fn c18dc6dc9dc12dc15() -> PolarsResult<()> {
    check_mass(
        fatty_acid_data_frame(&C18DC6DC9DC12DC15)?,
        expected::C18DC6DC9DC12DC15,
    )?;
    Ok(())
}

#[test]
fn c19() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C19)?, expected::C19)?;
    Ok(())
}

#[test]
fn c20() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C20)?, expected::C20)?;
    Ok(())
}

#[test]
fn c20dc9() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C20DC9)?, expected::C20DC9)?;
    Ok(())
}

#[test]
fn c20dc11() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C20DC11)?, expected::C20DC11)?;
    Ok(())
}

#[test]
fn c20dc11dc14() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C20DC11DC14)?, expected::C20DC11DC14)?;
    Ok(())
}

#[test]
fn c20dc5dc8dc11() -> PolarsResult<()> {
    check_mass(
        fatty_acid_data_frame(&C20DC5DC8DC11)?,
        expected::C20DC5DC8DC11,
    )?;
    Ok(())
}

#[test]
fn c20dc8dc11dc14() -> PolarsResult<()> {
    check_mass(
        fatty_acid_data_frame(&C20DC8DC11DC14)?,
        expected::C20DC8DC11DC14,
    )?;
    Ok(())
}

#[test]
fn c20dc11dc14dc17() -> PolarsResult<()> {
    check_mass(
        fatty_acid_data_frame(&C20DC11DC14DC17)?,
        expected::C20DC11DC14DC17,
    )?;
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14() -> PolarsResult<()> {
    check_mass(
        fatty_acid_data_frame(&C20DC5DC8DC11DC14)?,
        expected::C20DC5DC8DC11DC14,
    )?;
    Ok(())
}

#[test]
fn c20dc8dc11dc14dc17() -> PolarsResult<()> {
    check_mass(
        fatty_acid_data_frame(&C20DC8DC11DC14DC17)?,
        expected::C20DC8DC11DC14DC17,
    )?;
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14dc17() -> PolarsResult<()> {
    check_mass(
        fatty_acid_data_frame(&C20DC5DC8DC11DC14DC17)?,
        expected::C20DC5DC8DC11DC14DC17,
    )?;
    Ok(())
}

#[test]
fn c21() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C21)?, expected::C21)?;
    Ok(())
}

#[test]
fn c22() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C22)?, expected::C22)?;
    Ok(())
}

#[test]
fn c22dc13() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C22DC13)?, expected::C22DC13)?;
    Ok(())
}

#[test]
fn c22dc13dc16() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C22DC13DC16)?, expected::C22DC13DC16)?;
    Ok(())
}

#[test]
fn c22dc5dc13dc16() -> PolarsResult<()> {
    check_mass(
        fatty_acid_data_frame(&C22DC5DC13DC16)?,
        expected::C22DC5DC13DC16,
    )?;
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16() -> PolarsResult<()> {
    check_mass(
        fatty_acid_data_frame(&C22DC7DC10DC13DC16)?,
        expected::C22DC7DC10DC13DC16,
    )?;
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check_mass(
        fatty_acid_data_frame(&C22DC7DC10DC13DC16DC19)?,
        expected::C22DC7DC10DC13DC16DC19,
    )?;
    Ok(())
}

#[test]
fn c22dc4dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check_mass(
        fatty_acid_data_frame(&C22DC4DC7DC10DC13DC16DC19)?,
        expected::C22DC4DC7DC10DC13DC16DC19,
    )?;
    Ok(())
}

#[test]
fn c23() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C23)?, expected::C23)?;
    Ok(())
}

#[test]
fn c24() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C24)?, expected::C24)?;
    Ok(())
}

#[test]
fn c24dc15() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C24DC15)?, expected::C24DC15)?;
    Ok(())
}

#[test]
fn c24dc15dc18() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C24DC15DC18)?, expected::C24DC15DC18)?;
    Ok(())
}

#[test]
fn c24dc12dc15dc18() -> PolarsResult<()> {
    check_mass(
        fatty_acid_data_frame(&C24DC12DC15DC18)?,
        expected::C24DC12DC15DC18,
    )?;
    Ok(())
}

#[test]
fn c24dc9dc12dc15dc18() -> PolarsResult<()> {
    check_mass(
        fatty_acid_data_frame(&C24DC9DC12DC15DC18)?,
        expected::C24DC9DC12DC15DC18,
    )?;
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18() -> PolarsResult<()> {
    check_mass(
        fatty_acid_data_frame(&C24DC6DC9DC12DC15DC18)?,
        expected::C24DC6DC9DC12DC15DC18,
    )?;
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18dc21() -> PolarsResult<()> {
    check_mass(
        fatty_acid_data_frame(&C24DC6DC9DC12DC15DC18DC21)?,
        expected::C24DC6DC9DC12DC15DC18DC21,
    )?;
    Ok(())
}

#[test]
fn c25() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C25)?, expected::C25)?;
    Ok(())
}

#[test]
fn c26() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C26)?, expected::C26)?;
    Ok(())
}

#[test]
fn c26dc17() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C26DC17)?, expected::C26DC17)?;
    Ok(())
}

#[test]
fn c27() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C27)?, expected::C27)?;
    Ok(())
}

#[test]
fn c28() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C28)?, expected::C28)?;
    Ok(())
}

#[test]
fn c29() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C29)?, expected::C29)?;
    Ok(())
}

#[test]
fn c30() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C30)?, expected::C30)?;
    Ok(())
}

#[test]
fn c30dc21() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C30DC21)?, expected::C30DC21)?;
    Ok(())
}

#[test]
fn c31() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C31)?, expected::C31)?;
    Ok(())
}

#[test]
fn c32() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C32)?, expected::C32)?;
    Ok(())
}

#[test]
fn c33() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C33)?, expected::C33)?;
    Ok(())
}

#[test]
fn c34() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C34)?, expected::C34)?;
    Ok(())
}

#[test]
fn c35() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C35)?, expected::C35)?;
    Ok(())
}

#[test]
fn c36() -> PolarsResult<()> {
    check_mass(fatty_acid_data_frame(&C36)?, expected::C36)?;
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
