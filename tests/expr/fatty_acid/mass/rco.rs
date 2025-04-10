use super::*;

fn mass(data_frame: DataFrame) -> PolarsResult<f64> {
    let data_frame = data_frame
        .lazy()
        .select([col("FattyAcid").fatty_acid().rco().mass(None).round(4)])
        .collect()?;
    Ok(data_frame["Mass"].f64()?.get(0).unwrap())
}

#[test]
fn c4u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C4U0)?)?, r#const::C4U0);
    Ok(())
}

#[test]
fn c5u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C5U0)?)?, r#const::C5U0);
    Ok(())
}

#[test]
fn c6u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C6U0)?)?, r#const::C6U0);
    Ok(())
}

#[test]
fn c7u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C7U0)?)?, r#const::C7U0);
    Ok(())
}

#[test]
fn c8u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C8U0)?)?, r#const::C8U0);
    Ok(())
}

#[test]
fn c9u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C9U0)?)?, r#const::C9U0);
    Ok(())
}

#[test]
fn c10u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C10U0)?)?, r#const::C10U0);
    Ok(())
}

#[test]
fn c11u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C11U0)?)?, r#const::C11U0);
    Ok(())
}

#[test]
fn c12u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C12U0)?)?, r#const::C12U0);
    Ok(())
}

#[test]
fn c13u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C13U0)?)?, r#const::C13U0);
    Ok(())
}

#[test]
fn c14u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C14U0)?)?, r#const::C14U0);
    Ok(())
}

#[test]
fn c15u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C15U0)?)?, r#const::C15U0);
    Ok(())
}

#[test]
fn c16u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C16U0)?)?, r#const::C16U0);
    Ok(())
}

#[test]
fn c16u1dc9() -> PolarsResult<()> {
    assert_eq!(mass(fa(C16U1DC9)?)?, r#const::C16U1DC9);
    Ok(())
}

#[test]
fn c16u1dt9() -> PolarsResult<()> {
    assert_eq!(mass(fa(C16U1DT9)?)?, r#const::C16U1DT9);
    Ok(())
}

#[test]
fn c17u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C17U0)?)?, r#const::C17U0);
    Ok(())
}

#[test]
fn c18u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C18U0)?)?, r#const::C18U0);
    Ok(())
}

#[test]
fn c18u1dc9() -> PolarsResult<()> {
    assert_eq!(mass(fa(C18U1DC9)?)?, r#const::C18U1DC9);
    Ok(())
}

#[test]
fn c18u1dt9() -> PolarsResult<()> {
    assert_eq!(mass(fa(C18U1DT9)?)?, r#const::C18U1DT9);
    Ok(())
}

#[test]
fn c18u2dc9dc12() -> PolarsResult<()> {
    assert_eq!(mass(fa(C18U2DC9DC12)?)?, r#const::C18U2DC9DC12);
    Ok(())
}

#[test]
fn c18u3dc6dc9dc12() -> PolarsResult<()> {
    assert_eq!(mass(fa(C18U3DC6DC9DC12)?)?, r#const::C18U3DC6DC9DC12);
    Ok(())
}

#[test]
fn c18u3dc8dt10dc12() -> PolarsResult<()> {
    assert_eq!(mass(fa(C18U3DC8DT10DC12)?)?, r#const::C18U3DC8DT10DC12);
    Ok(())
}

#[test]
fn c18u3dc9dc12dc15() -> PolarsResult<()> {
    assert_eq!(mass(fa(C18U3DC9DC12DC15)?)?, r#const::C18U3DC9DC12DC15);
    Ok(())
}

#[test]
fn c18u3dc9dt11dt13() -> PolarsResult<()> {
    assert_eq!(mass(fa(C18U3DC9DT11DT13)?)?, r#const::C18U3DC9DT11DT13);
    Ok(())
}

#[test]
fn c18u3dt9dt11dc13() -> PolarsResult<()> {
    assert_eq!(mass(fa(C18U3DT9DT11DC13)?)?, r#const::C18U3DT9DT11DC13);
    Ok(())
}

#[test]
fn c18u3dt9dt11dt13() -> PolarsResult<()> {
    assert_eq!(mass(fa(C18U3DT9DT11DT13)?)?, r#const::C18U3DT9DT11DT13);
    Ok(())
}

#[test]
fn c18u4dc6dc9dc12dc15() -> PolarsResult<()> {
    assert_eq!(
        mass(fa(C18U4DC6DC9DC12DC15)?)?,
        r#const::C18U4DC6DC9DC12DC15
    );
    Ok(())
}

#[test]
fn c19u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C19U0)?)?, r#const::C19U0);
    Ok(())
}

#[test]
fn c20u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C20U0)?)?, r#const::C20U0);
    Ok(())
}

#[test]
fn c20u1dc9() -> PolarsResult<()> {
    assert_eq!(mass(fa(C20U1DC9)?)?, r#const::C20U1DC9);
    Ok(())
}

#[test]
fn c20u1dc11() -> PolarsResult<()> {
    assert_eq!(mass(fa(C20U1DC11)?)?, r#const::C20U1DC11);
    Ok(())
}

#[test]
fn c20u2dc11dc14() -> PolarsResult<()> {
    assert_eq!(mass(fa(C20U2DC11DC14)?)?, r#const::C20U2DC11DC14);
    Ok(())
}

#[test]
fn c20u3dc5dc8dc11() -> PolarsResult<()> {
    assert_eq!(mass(fa(C20U3DC5DC8DC11)?)?, r#const::C20U3DC5DC8DC11);
    Ok(())
}

#[test]
fn c20u3dc8dc11dc14() -> PolarsResult<()> {
    assert_eq!(mass(fa(C20U3DC8DC11DC14)?)?, r#const::C20U3DC8DC11DC14);
    Ok(())
}

#[test]
fn c20u3dc11dc14dc17() -> PolarsResult<()> {
    assert_eq!(mass(fa(C20U3DC11DC14DC17)?)?, r#const::C20U3DC11DC14DC17);
    Ok(())
}

#[test]
fn c20u4dc5dc8dc11dc14() -> PolarsResult<()> {
    assert_eq!(
        mass(fa(C20U4DC5DC8DC11DC14)?)?,
        r#const::C20U4DC5DC8DC11DC14
    );
    Ok(())
}

#[test]
fn c20u4dc8dc11dc14dc17() -> PolarsResult<()> {
    assert_eq!(
        mass(fa(C20U4DC8DC11DC14DC17)?)?,
        r#const::C20U4DC8DC11DC14DC17
    );
    Ok(())
}

#[test]
fn c20u5dc5dc8dc11dc14dc17() -> PolarsResult<()> {
    assert_eq!(
        mass(fa(C20U5DC5DC8DC11DC14DC17)?)?,
        r#const::C20U5DC5DC8DC11DC14DC17
    );
    Ok(())
}

#[test]
fn c21u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C21U0)?)?, r#const::C21U0);
    Ok(())
}

#[test]
fn c22u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C22U0)?)?, r#const::C22U0);
    Ok(())
}

#[test]
fn c22u1dc13() -> PolarsResult<()> {
    assert_eq!(mass(fa(C22U1DC13)?)?, r#const::C22U1DC13);
    Ok(())
}

#[test]
fn c22u2dc13dc16() -> PolarsResult<()> {
    assert_eq!(mass(fa(C22U2DC13DC16)?)?, r#const::C22U2DC13DC16);
    Ok(())
}

#[test]
fn c22u3dc5dc13dc16() -> PolarsResult<()> {
    assert_eq!(mass(fa(C22U3DC5DC13DC16)?)?, r#const::C22U3DC5DC13DC16);
    Ok(())
}

#[test]
fn c22u4dc7dc10dc13dc16() -> PolarsResult<()> {
    assert_eq!(
        mass(fa(C22U4DC7DC10DC13DC16)?)?,
        r#const::C22U4DC7DC10DC13DC16
    );
    Ok(())
}

#[test]
fn c22u5dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    assert_eq!(
        mass(fa(C22U5DC7DC10DC13DC16DC19)?)?,
        r#const::C22U5DC7DC10DC13DC16DC19
    );
    Ok(())
}

#[test]
fn c22u6dc4dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    assert_eq!(
        mass(fa(C22U6DC4DC7DC10DC13DC16DC19)?)?,
        r#const::C22U6DC4DC7DC10DC13DC16DC19
    );
    Ok(())
}

#[test]
fn c23u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C23U0)?)?, r#const::C23U0);
    Ok(())
}

#[test]
fn c24u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C24U0)?)?, r#const::C24U0);
    Ok(())
}

#[test]
fn c24u1dc15() -> PolarsResult<()> {
    assert_eq!(mass(fa(C24U1DC15)?)?, r#const::C24U1DC15);
    Ok(())
}

#[test]
fn c24u2dc15dc18() -> PolarsResult<()> {
    assert_eq!(mass(fa(C24U2DC15DC18)?)?, r#const::C24U2DC15DC18);
    Ok(())
}

#[test]
fn c24u3dc12dc15dc18() -> PolarsResult<()> {
    assert_eq!(mass(fa(C24U3DC12DC15DC18)?)?, r#const::C24U3DC12DC15DC18);
    Ok(())
}

#[test]
fn c24u4dc9dc12dc15dc18() -> PolarsResult<()> {
    assert_eq!(
        mass(fa(C24U4DC9DC12DC15DC18)?)?,
        r#const::C24U4DC9DC12DC15DC18
    );
    Ok(())
}

#[test]
fn c24u5dc6dc9dc12dc15dc18() -> PolarsResult<()> {
    assert_eq!(
        mass(fa(C24U5DC6DC9DC12DC15DC18)?)?,
        r#const::C24U5DC6DC9DC12DC15DC18
    );
    Ok(())
}

#[test]
fn c24u6dc6dc9dc12dc15dc18dc21() -> PolarsResult<()> {
    assert_eq!(
        mass(fa(C24U6DC6DC9DC12DC15DC18DC21)?)?,
        r#const::C24U6DC6DC9DC12DC15DC18DC21
    );
    Ok(())
}

#[test]
fn c25u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C25U0)?)?, r#const::C25U0);
    Ok(())
}

#[test]
fn c26u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C26U0)?)?, r#const::C26U0);
    Ok(())
}

#[test]
fn c26u1dc17() -> PolarsResult<()> {
    assert_eq!(mass(fa(C26U1DC17)?)?, r#const::C26U1DC17);
    Ok(())
}

#[test]
fn c27u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C27U0)?)?, r#const::C27U0);
    Ok(())
}

#[test]
fn c28u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C28U0)?)?, r#const::C28U0);
    Ok(())
}

#[test]
fn c29u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C29U0)?)?, r#const::C29U0);
    Ok(())
}

#[test]
fn c30u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C30U0)?)?, r#const::C30U0);
    Ok(())
}

#[test]
fn c30u1dc21() -> PolarsResult<()> {
    assert_eq!(mass(fa(C30U1DC21)?)?, r#const::C30U1DC21);
    Ok(())
}

#[test]
fn c31u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C31U0)?)?, r#const::C31U0);
    Ok(())
}

#[test]
fn c32u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C32U0)?)?, r#const::C32U0);
    Ok(())
}

#[test]
fn c33u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C33U0)?)?, r#const::C33U0);
    Ok(())
}

#[test]
fn c34u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C34U0)?)?, r#const::C34U0);
    Ok(())
}

#[test]
fn c35u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C35U0)?)?, r#const::C35U0);
    Ok(())
}

#[test]
fn c36u0() -> PolarsResult<()> {
    assert_eq!(mass(fa(C36U0)?)?, r#const::C36U0);
    Ok(())
}

mod r#const {
    pub(super) const C4U0: f64 = 71.0497;
    pub(super) const C5U0: f64 = 85.0653;
    pub(super) const C6U0: f64 = 99.0810;
    pub(super) const C7U0: f64 = 113.0966;
    pub(super) const C8U0: f64 = 127.1123;
    pub(super) const C9U0: f64 = 141.1279;
    pub(super) const C10U0: f64 = 155.1436;
    pub(super) const C11U0: f64 = 169.1592;
    pub(super) const C12U0: f64 = 183.1749;
    pub(super) const C13U0: f64 = 197.1905;
    pub(super) const C14U0: f64 = 211.2062;
    pub(super) const C15U0: f64 = 225.2218;
    pub(super) const C16U0: f64 = 239.2375;
    pub(super) const C16U1DC9: f64 = 237.2218;
    pub(super) const C16U1DT9: f64 = 237.2218;
    pub(super) const C17U0: f64 = 253.2531;
    pub(super) const C18U0: f64 = 267.2688;
    pub(super) const C18U1DC9: f64 = 265.2531;
    pub(super) const C18U1DT9: f64 = 265.2531;
    pub(super) const C18U2DC9DC12: f64 = 263.2375;
    pub(super) const C18U3DC6DC9DC12: f64 = 261.2218;
    pub(super) const C18U3DC8DT10DC12: f64 = 261.2218;
    pub(super) const C18U3DC9DC12DC15: f64 = 261.2218;
    pub(super) const C18U3DC9DT11DT13: f64 = 261.2218;
    pub(super) const C18U3DT9DT11DC13: f64 = 261.2218;
    pub(super) const C18U3DT9DT11DT13: f64 = 261.2218;
    pub(super) const C18U4DC6DC9DC12DC15: f64 = 259.2062;
    pub(super) const C19U0: f64 = 281.2844;
    pub(super) const C20U0: f64 = 295.3001;
    pub(super) const C20U1DC9: f64 = 293.2844;
    pub(super) const C20U1DC11: f64 = 293.2844;
    pub(super) const C20U2DC11DC14: f64 = 291.2688;
    pub(super) const C20U3DC5DC8DC11: f64 = 289.2531;
    pub(super) const C20U3DC8DC11DC14: f64 = 289.2531;
    pub(super) const C20U3DC11DC14DC17: f64 = 289.2531;
    pub(super) const C20U4DC5DC8DC11DC14: f64 = 287.2375;
    pub(super) const C20U4DC8DC11DC14DC17: f64 = 287.2375;
    pub(super) const C20U5DC5DC8DC11DC14DC17: f64 = 285.2218;
    pub(super) const C21U0: f64 = 309.3157;
    pub(super) const C22U0: f64 = 323.3314;
    pub(super) const C22U1DC13: f64 = 321.3157;
    pub(super) const C22U2DC13DC16: f64 = 319.3001;
    pub(super) const C22U3DC5DC13DC16: f64 = 317.2844;
    pub(super) const C22U4DC7DC10DC13DC16: f64 = 315.2688;
    pub(super) const C22U5DC7DC10DC13DC16DC19: f64 = 313.2531;
    pub(super) const C22U6DC4DC7DC10DC13DC16DC19: f64 = 311.2375;
    pub(super) const C23U0: f64 = 337.3470;
    pub(super) const C24U0: f64 = 351.3627;
    pub(super) const C24U1DC15: f64 = 349.3470;
    pub(super) const C24U2DC15DC18: f64 = 347.3314;
    pub(super) const C24U3DC12DC15DC18: f64 = 345.3157;
    pub(super) const C24U4DC9DC12DC15DC18: f64 = 343.3001;
    pub(super) const C24U5DC6DC9DC12DC15DC18: f64 = 341.2844;
    pub(super) const C24U6DC6DC9DC12DC15DC18DC21: f64 = 339.2688;
    pub(super) const C25U0: f64 = 365.3783;
    pub(super) const C26U0: f64 = 379.3940;
    pub(super) const C26U1DC17: f64 = 377.3783;
    pub(super) const C27U0: f64 = 393.4096;
    pub(super) const C28U0: f64 = 407.4253;
    pub(super) const C29U0: f64 = 421.4409;
    pub(super) const C30U0: f64 = 435.4566;
    pub(super) const C30U1DC21: f64 = 433.4409;
    pub(super) const C31U0: f64 = 449.4722;
    pub(super) const C32U0: f64 = 463.4879;
    pub(super) const C33U0: f64 = 477.5035;
    pub(super) const C34U0: f64 = 491.5192;
    pub(super) const C35U0: f64 = 505.5348;
    pub(super) const C36U0: f64 = 519.5505;
}
