use super::*;

macro_rules! check {
    ($identifier:ident, $expected:expr) => {{
        let data_frame = fatty_acid($identifier.clone())?
            .lazy()
            .select([col(FATTY_ACID)
                .fatty_acid()
                .rco()
                .relative_atomic_mass(None)
                .round(DECIMALS, RoundMode::HalfToEven)])
            .collect()?;
        let mass = data_frame["Mass"].f64()?.get(0).unwrap();
        assert_epsilon!(mass, $expected);
    }};
}

#[test]
fn c4() -> PolarsResult<()> {
    check!(C4, 71.0497);
    Ok(())
}

#[test]
fn c5() -> PolarsResult<()> {
    check!(C5, 85.0653);
    Ok(())
}

#[test]
fn c6() -> PolarsResult<()> {
    check!(C6, 99.0810);
    Ok(())
}

#[test]
fn c7() -> PolarsResult<()> {
    check!(C7, 113.0966);
    Ok(())
}

#[test]
fn c8() -> PolarsResult<()> {
    check!(C8, 127.1123);
    Ok(())
}

#[test]
fn c9() -> PolarsResult<()> {
    check!(C9, 141.1279);
    Ok(())
}

#[test]
fn c10() -> PolarsResult<()> {
    check!(C10, 155.1436);
    Ok(())
}

#[test]
fn c11() -> PolarsResult<()> {
    check!(C11, 169.1592);
    Ok(())
}

#[test]
fn c12() -> PolarsResult<()> {
    check!(C12, 183.1749);
    Ok(())
}

#[test]
fn c13() -> PolarsResult<()> {
    check!(C13, 197.1905);
    Ok(())
}

#[test]
fn c14() -> PolarsResult<()> {
    check!(C14, 211.2062);
    Ok(())
}

#[test]
fn c15() -> PolarsResult<()> {
    check!(C15, 225.2218);
    Ok(())
}

#[test]
fn c16() -> PolarsResult<()> {
    check!(C16, 239.2375);
    Ok(())
}

#[test]
fn c16dc9() -> PolarsResult<()> {
    check!(C16DC9, 237.2218);
    Ok(())
}

#[test]
fn c16dt9() -> PolarsResult<()> {
    check!(C16DT9, 237.2218);
    Ok(())
}

#[test]
fn c17() -> PolarsResult<()> {
    check!(C17, 253.2531);
    Ok(())
}

#[test]
fn c18() -> PolarsResult<()> {
    check!(C18, 267.2688);
    Ok(())
}

#[test]
fn c18dc9() -> PolarsResult<()> {
    check!(C18DC9, 265.2531);
    Ok(())
}

#[test]
fn c18dt9() -> PolarsResult<()> {
    check!(C18DT9, 265.2531);
    Ok(())
}

#[test]
fn c18dc9dc12() -> PolarsResult<()> {
    check!(C18DC9DC12, 263.2375);
    Ok(())
}

#[test]
fn c18dc6dc9dc12() -> PolarsResult<()> {
    check!(C18DC6DC9DC12, 261.2218);
    Ok(())
}

#[test]
fn c18dc8dt10dc12() -> PolarsResult<()> {
    check!(C18DC8DT10DC12, 261.2218);
    Ok(())
}

#[test]
fn c18dc9dc12dc15() -> PolarsResult<()> {
    check!(C18DC9DC12DC15, 261.2218);
    Ok(())
}

#[test]
fn c18dc9dt11dt13() -> PolarsResult<()> {
    check!(C18DC9DT11DT13, 261.2218);
    Ok(())
}

#[test]
fn c18dt9dt11dc13() -> PolarsResult<()> {
    check!(C18DT9DT11DC13, 261.2218);
    Ok(())
}

#[test]
fn c18dt9dt11dt13() -> PolarsResult<()> {
    check!(C18DT9DT11DT13, 261.2218);
    Ok(())
}

#[test]
fn c18dc6dc9dc12dc15() -> PolarsResult<()> {
    check!(C18DC6DC9DC12DC15, 259.2062);
    Ok(())
}

#[test]
fn c19() -> PolarsResult<()> {
    check!(C19, 281.2844);
    Ok(())
}

#[test]
fn c20() -> PolarsResult<()> {
    check!(C20, 295.3001);
    Ok(())
}

#[test]
fn c20dc9() -> PolarsResult<()> {
    check!(C20DC9, 293.2844);
    Ok(())
}

#[test]
fn c20dc11() -> PolarsResult<()> {
    check!(C20DC11, 293.2844);
    Ok(())
}

#[test]
fn c20dc11dc14() -> PolarsResult<()> {
    check!(C20DC11DC14, 291.2688);
    Ok(())
}

#[test]
fn c20dc5dc8dc11() -> PolarsResult<()> {
    check!(C20DC5DC8DC11, 289.2531);
    Ok(())
}

#[test]
fn c20dc8dc11dc14() -> PolarsResult<()> {
    check!(C20DC8DC11DC14, 289.2531);
    Ok(())
}

#[test]
fn c20dc11dc14dc17() -> PolarsResult<()> {
    check!(C20DC11DC14DC17, 289.2531);
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14() -> PolarsResult<()> {
    check!(C20DC5DC8DC11DC14, 287.2375);
    Ok(())
}

#[test]
fn c20dc8dc11dc14dc17() -> PolarsResult<()> {
    check!(C20DC8DC11DC14DC17, 287.2375);
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14dc17() -> PolarsResult<()> {
    check!(C20DC5DC8DC11DC14DC17, 285.2218);
    Ok(())
}

#[test]
fn c21() -> PolarsResult<()> {
    check!(C21, 309.3157);
    Ok(())
}

#[test]
fn c22() -> PolarsResult<()> {
    check!(C22, 323.3314);
    Ok(())
}

#[test]
fn c22dc13() -> PolarsResult<()> {
    check!(C22DC13, 321.3157);
    Ok(())
}

#[test]
fn c22dc13dc16() -> PolarsResult<()> {
    check!(C22DC13DC16, 319.3001);
    Ok(())
}

#[test]
fn c22dc5dc13dc16() -> PolarsResult<()> {
    check!(C22DC5DC13DC16, 317.2844);
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16() -> PolarsResult<()> {
    check!(C22DC7DC10DC13DC16, 315.2688);
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check!(C22DC7DC10DC13DC16DC19, 313.2531);
    Ok(())
}

#[test]
fn c22dc4dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check!(C22DC4DC7DC10DC13DC16DC19, 311.2375);
    Ok(())
}

#[test]
fn c23() -> PolarsResult<()> {
    check!(C23, 337.3470);
    Ok(())
}

#[test]
fn c24() -> PolarsResult<()> {
    check!(C24, 351.3627);
    Ok(())
}

#[test]
fn c24dc15() -> PolarsResult<()> {
    check!(C24DC15, 349.3470);
    Ok(())
}

#[test]
fn c24dc15dc18() -> PolarsResult<()> {
    check!(C24DC15DC18, 347.3314);
    Ok(())
}

#[test]
fn c24dc12dc15dc18() -> PolarsResult<()> {
    check!(C24DC12DC15DC18, 345.3157);
    Ok(())
}

#[test]
fn c24dc9dc12dc15dc18() -> PolarsResult<()> {
    check!(C24DC9DC12DC15DC18, 343.3001);
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18() -> PolarsResult<()> {
    check!(C24DC6DC9DC12DC15DC18, 341.2844);
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18dc21() -> PolarsResult<()> {
    check!(C24DC6DC9DC12DC15DC18DC21, 339.2688);
    Ok(())
}

#[test]
fn c25() -> PolarsResult<()> {
    check!(C25, 365.3783);
    Ok(())
}

#[test]
fn c26() -> PolarsResult<()> {
    check!(C26, 379.3940);
    Ok(())
}

#[test]
fn c26dc17() -> PolarsResult<()> {
    check!(C26DC17, 377.3783);
    Ok(())
}

#[test]
fn c27() -> PolarsResult<()> {
    check!(C27, 393.4096);
    Ok(())
}

#[test]
fn c28() -> PolarsResult<()> {
    check!(C28, 407.4253);
    Ok(())
}

#[test]
fn c29() -> PolarsResult<()> {
    check!(C29, 421.4409);
    Ok(())
}

#[test]
fn c30() -> PolarsResult<()> {
    check!(C30, 435.4566);
    Ok(())
}

#[test]
fn c30dc21() -> PolarsResult<()> {
    check!(C30DC21, 433.4409);
    Ok(())
}

#[test]
fn c31() -> PolarsResult<()> {
    check!(C31, 449.4722);
    Ok(())
}

#[test]
fn c32() -> PolarsResult<()> {
    check!(C32, 463.4879);
    Ok(())
}

#[test]
fn c33() -> PolarsResult<()> {
    check!(C33, 477.5035);
    Ok(())
}

#[test]
fn c34() -> PolarsResult<()> {
    check!(C34, 491.5192);
    Ok(())
}

#[test]
fn c35() -> PolarsResult<()> {
    check!(C35, 505.5348);
    Ok(())
}

#[test]
fn c36() -> PolarsResult<()> {
    check!(C36, 519.5505);
    Ok(())
}
