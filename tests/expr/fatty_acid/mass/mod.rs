use super::*;

const DECIMALS: u32 = 4;

macro_rules! check {
    ($identifier:ident, $expected:expr) => {{
        let data_frame = fatty_acid($identifier)?
            .lazy()
            .select([col(FATTY_ACID)
                .fatty_acid()
                .mass(None)
                .round(DECIMALS, RoundMode::HalfToEven)])
            .collect()?;
        let got = data_frame["Mass"].f64()?.get(0).unwrap();
        assert!((got - $expected).abs() < f64::EPSILON);
    }};
}

#[test]
fn c4() -> PolarsResult<()> {
    check!(C4, 88.0524);
    Ok(())
}

#[test]
fn c5() -> PolarsResult<()> {
    check!(C5, 102.0681);
    Ok(())
}

#[test]
fn c6() -> PolarsResult<()> {
    check!(C6, 116.0837);
    Ok(())
}

#[test]
fn c7() -> PolarsResult<()> {
    check!(C7, 130.0994);
    Ok(())
}

#[test]
fn c8() -> PolarsResult<()> {
    check!(C8, 144.1150);
    Ok(())
}

#[test]
fn c9() -> PolarsResult<()> {
    check!(C9, 158.1307);
    Ok(())
}

#[test]
fn c10() -> PolarsResult<()> {
    check!(C10, 172.1463);
    Ok(())
}

#[test]
fn c11() -> PolarsResult<()> {
    check!(C11, 186.1620);
    Ok(())
}

#[test]
fn c12() -> PolarsResult<()> {
    check!(C12, 200.1776);
    Ok(())
}

#[test]
fn c13() -> PolarsResult<()> {
    check!(C13, 214.1933);
    Ok(())
}

#[test]
fn c14() -> PolarsResult<()> {
    check!(C14, 228.2089);
    Ok(())
}

#[test]
fn c15() -> PolarsResult<()> {
    check!(C15, 242.2246);
    Ok(())
}

#[test]
fn c16() -> PolarsResult<()> {
    check!(C16, 256.2402);
    Ok(())
}

#[test]
fn c16dc9() -> PolarsResult<()> {
    check!(C16DC9, 254.2246);
    Ok(())
}

#[test]
fn c16dt9() -> PolarsResult<()> {
    check!(C16DT9, 254.2246);
    Ok(())
}

#[test]
fn c17() -> PolarsResult<()> {
    check!(C17, 270.2559);
    Ok(())
}

#[test]
fn c18() -> PolarsResult<()> {
    check!(C18, 284.2715);
    Ok(())
}

#[test]
fn c18dc9() -> PolarsResult<()> {
    check!(C18DC9, 282.2559);
    Ok(())
}

#[test]
fn c18dt9() -> PolarsResult<()> {
    check!(C18DT9, 282.2559);
    Ok(())
}

#[test]
fn c18dc9dc12() -> PolarsResult<()> {
    check!(C18DC9DC12, 280.2402);
    Ok(())
}

#[test]
fn c18dc6dc9dc12() -> PolarsResult<()> {
    check!(C18DC6DC9DC12, 278.2246);
    Ok(())
}

#[test]
fn c18dc8dt10dc12() -> PolarsResult<()> {
    check!(C18DC8DT10DC12, 278.2246);
    Ok(())
}

#[test]
fn c18dc9dc12dc15() -> PolarsResult<()> {
    check!(C18DC9DC12DC15, 278.2246);
    Ok(())
}

#[test]
fn c18dc9dt11dt13() -> PolarsResult<()> {
    check!(C18DC9DT11DT13, 278.2246);
    Ok(())
}

#[test]
fn c18dt9dt11dc13() -> PolarsResult<()> {
    check!(C18DT9DT11DC13, 278.2246);
    Ok(())
}

#[test]
fn c18dt9dt11dt13() -> PolarsResult<()> {
    check!(C18DT9DT11DT13, 278.2246);
    Ok(())
}

#[test]
fn c18dc6dc9dc12dc15() -> PolarsResult<()> {
    check!(C18DC6DC9DC12DC15, 276.2089);
    Ok(())
}

#[test]
fn c19() -> PolarsResult<()> {
    check!(C19, 298.2872);
    Ok(())
}

#[test]
fn c20() -> PolarsResult<()> {
    check!(C20, 312.3028);
    Ok(())
}

#[test]
fn c20dc9() -> PolarsResult<()> {
    check!(C20DC9, 310.2872);
    Ok(())
}

#[test]
fn c20dc11() -> PolarsResult<()> {
    check!(C20DC11, 310.2872);
    Ok(())
}

#[test]
fn c20dc11dc14() -> PolarsResult<()> {
    check!(C20DC11DC14, 308.2715);
    Ok(())
}

#[test]
fn c20dc5dc8dc11() -> PolarsResult<()> {
    check!(C20DC5DC8DC11, 306.2559);
    Ok(())
}

#[test]
fn c20dc8dc11dc14() -> PolarsResult<()> {
    check!(C20DC8DC11DC14, 306.2559);
    Ok(())
}

#[test]
fn c20dc11dc14dc17() -> PolarsResult<()> {
    check!(C20DC11DC14DC17, 306.2559);
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14() -> PolarsResult<()> {
    check!(C20DC5DC8DC11DC14, 304.2402);
    Ok(())
}

#[test]
fn c20dc8dc11dc14dc17() -> PolarsResult<()> {
    check!(C20DC8DC11DC14DC17, 304.2402);
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14dc17() -> PolarsResult<()> {
    check!(C20DC5DC8DC11DC14DC17, 302.2246);
    Ok(())
}

#[test]
fn c21() -> PolarsResult<()> {
    check!(C21, 326.3185);
    Ok(())
}

#[test]
fn c22() -> PolarsResult<()> {
    check!(C22, 340.3341);
    Ok(())
}

#[test]
fn c22dc13() -> PolarsResult<()> {
    check!(C22DC13, 338.3185);
    Ok(())
}

#[test]
fn c22dc13dc16() -> PolarsResult<()> {
    check!(C22DC13DC16, 336.3028);
    Ok(())
}

#[test]
fn c22dc5dc13dc16() -> PolarsResult<()> {
    check!(C22DC5DC13DC16, 334.2872);
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16() -> PolarsResult<()> {
    check!(C22DC7DC10DC13DC16, 332.2715);
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check!(C22DC7DC10DC13DC16DC19, 330.2559);
    Ok(())
}

#[test]
fn c22dc4dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check!(C22DC4DC7DC10DC13DC16DC19, 328.2402);
    Ok(())
}

#[test]
fn c23() -> PolarsResult<()> {
    check!(C23, 354.3498);
    Ok(())
}

#[test]
fn c24() -> PolarsResult<()> {
    check!(C24, 368.3654);
    Ok(())
}

#[test]
fn c24dc15() -> PolarsResult<()> {
    check!(C24DC15, 366.3498);
    Ok(())
}

#[test]
fn c24dc15dc18() -> PolarsResult<()> {
    check!(C24DC15DC18, 364.3341);
    Ok(())
}

#[test]
fn c24dc12dc15dc18() -> PolarsResult<()> {
    check!(C24DC12DC15DC18, 362.3185);
    Ok(())
}

#[test]
fn c24dc9dc12dc15dc18() -> PolarsResult<()> {
    check!(C24DC9DC12DC15DC18, 360.3028);
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18() -> PolarsResult<()> {
    check!(C24DC6DC9DC12DC15DC18, 358.2872);
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18dc21() -> PolarsResult<()> {
    check!(C24DC6DC9DC12DC15DC18DC21, 356.2715);
    Ok(())
}

#[test]
fn c25() -> PolarsResult<()> {
    check!(C25, 382.3811);
    Ok(())
}

#[test]
fn c26() -> PolarsResult<()> {
    check!(C26, 396.3967);
    Ok(())
}

#[test]
fn c26dc17() -> PolarsResult<()> {
    check!(C26DC17, 394.3811);
    Ok(())
}

#[test]
fn c27() -> PolarsResult<()> {
    check!(C27, 410.4124);
    Ok(())
}

#[test]
fn c28() -> PolarsResult<()> {
    check!(C28, 424.4280);
    Ok(())
}

#[test]
fn c29() -> PolarsResult<()> {
    check!(C29, 438.4437);
    Ok(())
}

#[test]
fn c30() -> PolarsResult<()> {
    check!(C30, 452.4593);
    Ok(())
}

#[test]
fn c30dc21() -> PolarsResult<()> {
    check!(C30DC21, 450.4437);
    Ok(())
}

#[test]
fn c31() -> PolarsResult<()> {
    check!(C31, 466.4750);
    Ok(())
}

#[test]
fn c32() -> PolarsResult<()> {
    check!(C32, 480.4906);
    Ok(())
}

#[test]
fn c33() -> PolarsResult<()> {
    check!(C33, 494.5063);
    Ok(())
}

#[test]
fn c34() -> PolarsResult<()> {
    check!(C34, 508.5219);
    Ok(())
}

#[test]
fn c35() -> PolarsResult<()> {
    check!(C35, 522.5376);
    Ok(())
}

#[test]
fn c36() -> PolarsResult<()> {
    check!(C36, 536.5532);
    Ok(())
}

mod rco;
mod rcoo;
mod rcooch3;
mod rcooh;
