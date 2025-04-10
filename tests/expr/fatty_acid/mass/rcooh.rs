use super::*;

fn mass(data_frame: DataFrame) -> PolarsResult<f64> {
    let data_frame = data_frame
        .lazy()
        .select([col("FattyAcid").fatty_acid().rcooh().mass(None).round(4)])
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
    pub(super) const C4U0: f64 = 88.0524;
    pub(super) const C5U0: f64 = 102.0681;
    pub(super) const C6U0: f64 = 116.0837;
    pub(super) const C7U0: f64 = 130.0994;
    pub(super) const C8U0: f64 = 144.1150;
    pub(super) const C9U0: f64 = 158.1307;
    pub(super) const C10U0: f64 = 172.1463;
    pub(super) const C11U0: f64 = 186.1620;
    pub(super) const C12U0: f64 = 200.1776;
    pub(super) const C13U0: f64 = 214.1933;
    pub(super) const C14U0: f64 = 228.2089;
    pub(super) const C15U0: f64 = 242.2246;
    pub(super) const C16U0: f64 = 256.2402;
    pub(super) const C16U1DC9: f64 = 254.2246;
    pub(super) const C16U1DT9: f64 = 254.2246;
    pub(super) const C17U0: f64 = 270.2559;
    pub(super) const C18U0: f64 = 284.2715;
    pub(super) const C18U1DC9: f64 = 282.2559;
    pub(super) const C18U1DT9: f64 = 282.2559;
    pub(super) const C18U2DC9DC12: f64 = 280.2402;
    pub(super) const C18U3DC6DC9DC12: f64 = 278.2246;
    pub(super) const C18U3DC8DT10DC12: f64 = 278.2246;
    pub(super) const C18U3DC9DC12DC15: f64 = 278.2246;
    pub(super) const C18U3DC9DT11DT13: f64 = 278.2246;
    pub(super) const C18U3DT9DT11DC13: f64 = 278.2246;
    pub(super) const C18U3DT9DT11DT13: f64 = 278.2246;
    pub(super) const C18U4DC6DC9DC12DC15: f64 = 276.2089;
    pub(super) const C19U0: f64 = 298.2872;
    pub(super) const C20U0: f64 = 312.3028;
    pub(super) const C20U1DC9: f64 = 310.2872;
    pub(super) const C20U1DC11: f64 = 310.2872;
    pub(super) const C20U2DC11DC14: f64 = 308.2715;
    pub(super) const C20U3DC5DC8DC11: f64 = 306.2559;
    pub(super) const C20U3DC8DC11DC14: f64 = 306.2559;
    pub(super) const C20U3DC11DC14DC17: f64 = 306.2559;
    pub(super) const C20U4DC5DC8DC11DC14: f64 = 304.2402;
    pub(super) const C20U4DC8DC11DC14DC17: f64 = 304.2402;
    pub(super) const C20U5DC5DC8DC11DC14DC17: f64 = 302.2246;
    pub(super) const C21U0: f64 = 326.3185;
    pub(super) const C22U0: f64 = 340.3341;
    pub(super) const C22U1DC13: f64 = 338.3185;
    pub(super) const C22U2DC13DC16: f64 = 336.3028;
    pub(super) const C22U3DC5DC13DC16: f64 = 334.2872;
    pub(super) const C22U4DC7DC10DC13DC16: f64 = 332.2715;
    pub(super) const C22U5DC7DC10DC13DC16DC19: f64 = 330.2559;
    pub(super) const C22U6DC4DC7DC10DC13DC16DC19: f64 = 328.2402;
    pub(super) const C23U0: f64 = 354.3498;
    pub(super) const C24U0: f64 = 368.3654;
    pub(super) const C24U1DC15: f64 = 366.3498;
    pub(super) const C24U2DC15DC18: f64 = 364.3341;
    pub(super) const C24U3DC12DC15DC18: f64 = 362.3185;
    pub(super) const C24U4DC9DC12DC15DC18: f64 = 360.3028;
    pub(super) const C24U5DC6DC9DC12DC15DC18: f64 = 358.2872;
    pub(super) const C24U6DC6DC9DC12DC15DC18DC21: f64 = 356.2715;
    pub(super) const C25U0: f64 = 382.3811;
    pub(super) const C26U0: f64 = 396.3967;
    pub(super) const C26U1DC17: f64 = 394.3811;
    pub(super) const C27U0: f64 = 410.4124;
    pub(super) const C28U0: f64 = 424.4280;
    pub(super) const C29U0: f64 = 438.4437;
    pub(super) const C30U0: f64 = 452.4593;
    pub(super) const C30U1DC21: f64 = 450.4437;
    pub(super) const C31U0: f64 = 466.4750;
    pub(super) const C32U0: f64 = 480.4906;
    pub(super) const C33U0: f64 = 494.5063;
    pub(super) const C34U0: f64 = 508.5219;
    pub(super) const C35U0: f64 = 522.5376;
    pub(super) const C36U0: f64 = 536.5532;
}
