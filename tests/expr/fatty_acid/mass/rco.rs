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
    check!(C4U0, 71.0497);
    Ok(())
}

#[test]
fn c5() -> PolarsResult<()> {
    check!(C5U0, 85.0653);
    Ok(())
}

#[test]
fn c6() -> PolarsResult<()> {
    check!(C6U0, 99.0810);
    Ok(())
}

#[test]
fn c7() -> PolarsResult<()> {
    check!(C7U0, 113.0966);
    Ok(())
}

#[test]
fn c8() -> PolarsResult<()> {
    check!(C8U0, 127.1123);
    Ok(())
}

#[test]
fn c9() -> PolarsResult<()> {
    check!(C9U0, 141.1279);
    Ok(())
}

#[test]
fn c10() -> PolarsResult<()> {
    check!(C10U0, 155.1436);
    Ok(())
}

#[test]
fn c11() -> PolarsResult<()> {
    check!(C11U0, 169.1592);
    Ok(())
}

#[test]
fn c12() -> PolarsResult<()> {
    check!(C12U0, 183.1749);
    Ok(())
}

#[test]
fn c13() -> PolarsResult<()> {
    check!(C13U0, 197.1905);
    Ok(())
}

#[test]
fn c14() -> PolarsResult<()> {
    check!(C14U0, 211.2062);
    Ok(())
}

#[test]
fn c15() -> PolarsResult<()> {
    check!(C15U0, 225.2218);
    Ok(())
}

#[test]
fn c16() -> PolarsResult<()> {
    check!(C16U0, 239.2375);
    Ok(())
}

#[test]
fn c16dc9() -> PolarsResult<()> {
    check!(C16C9, 237.2218);
    Ok(())
}

#[test]
fn c16dt9() -> PolarsResult<()> {
    check!(C16T9, 237.2218);
    Ok(())
}

#[test]
fn c17() -> PolarsResult<()> {
    check!(C17U0, 253.2531);
    Ok(())
}

#[test]
fn c18() -> PolarsResult<()> {
    check!(C18U0, 267.2688);
    Ok(())
}

#[test]
fn c18dc9() -> PolarsResult<()> {
    check!(C18C9, 265.2531);
    Ok(())
}

#[test]
fn c18dt9() -> PolarsResult<()> {
    check!(C18T9, 265.2531);
    Ok(())
}

#[test]
fn c18dc9dc12() -> PolarsResult<()> {
    check!(C18C9C12, 263.2375);
    Ok(())
}

#[test]
fn c18dc6dc9dc12() -> PolarsResult<()> {
    check!(C18C6C9C12, 261.2218);
    Ok(())
}

#[test]
fn c18dc8dt10dc12() -> PolarsResult<()> {
    check!(C18C8T10C12, 261.2218);
    Ok(())
}

#[test]
fn c18dc9dc12dc15() -> PolarsResult<()> {
    check!(C18C9C12C15, 261.2218);
    Ok(())
}

#[test]
fn c18dc9dt11dt13() -> PolarsResult<()> {
    check!(C18C9T11T13, 261.2218);
    Ok(())
}

#[test]
fn c18dt9dt11dc13() -> PolarsResult<()> {
    check!(C18T9T11C13, 261.2218);
    Ok(())
}

#[test]
fn c18dt9dt11dt13() -> PolarsResult<()> {
    check!(C18T9T11T13, 261.2218);
    Ok(())
}

#[test]
fn c18dc6dc9dc12dc15() -> PolarsResult<()> {
    check!(C18C6C9C12C15, 259.2062);
    Ok(())
}

#[test]
fn c19() -> PolarsResult<()> {
    check!(C19U0, 281.2844);
    Ok(())
}

#[test]
fn c20() -> PolarsResult<()> {
    check!(C20U0, 295.3001);
    Ok(())
}

#[test]
fn c20dc9() -> PolarsResult<()> {
    check!(C20C9, 293.2844);
    Ok(())
}

#[test]
fn c20dc11() -> PolarsResult<()> {
    check!(C20C11, 293.2844);
    Ok(())
}

#[test]
fn c20dc11dc14() -> PolarsResult<()> {
    check!(C20C11C14, 291.2688);
    Ok(())
}

#[test]
fn c20dc5dc8dc11() -> PolarsResult<()> {
    check!(C20C5C8C11, 289.2531);
    Ok(())
}

#[test]
fn c20dc8dc11dc14() -> PolarsResult<()> {
    check!(C20C8C11C14, 289.2531);
    Ok(())
}

#[test]
fn c20dc11dc14dc17() -> PolarsResult<()> {
    check!(C20C11C14C17, 289.2531);
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14() -> PolarsResult<()> {
    check!(C20C5C8C11C14, 287.2375);
    Ok(())
}

#[test]
fn c20dc8dc11dc14dc17() -> PolarsResult<()> {
    check!(C20C8C11C14C17, 287.2375);
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14dc17() -> PolarsResult<()> {
    check!(C20C5C8C11C14C17, 285.2218);
    Ok(())
}

#[test]
fn c21() -> PolarsResult<()> {
    check!(C21U0, 309.3157);
    Ok(())
}

#[test]
fn c22() -> PolarsResult<()> {
    check!(C22U0, 323.3314);
    Ok(())
}

#[test]
fn c22dc13() -> PolarsResult<()> {
    check!(C22C13, 321.3157);
    Ok(())
}

#[test]
fn c22dc13dc16() -> PolarsResult<()> {
    check!(C22C13C16, 319.3001);
    Ok(())
}

#[test]
fn c22dc5dc13dc16() -> PolarsResult<()> {
    check!(C22C5C13C16, 317.2844);
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16() -> PolarsResult<()> {
    check!(C22C7C10C13C16, 315.2688);
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check!(C22C7C10C13C16C19, 313.2531);
    Ok(())
}

#[test]
fn c22dc4dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check!(C22C4C7C10C13C16C19, 311.2375);
    Ok(())
}

#[test]
fn c23() -> PolarsResult<()> {
    check!(C23U0, 337.3470);
    Ok(())
}

#[test]
fn c24() -> PolarsResult<()> {
    check!(C24U0, 351.3627);
    Ok(())
}

#[test]
fn c24dc15() -> PolarsResult<()> {
    check!(C24C15, 349.3470);
    Ok(())
}

#[test]
fn c24dc15dc18() -> PolarsResult<()> {
    check!(C24C15C18, 347.3314);
    Ok(())
}

#[test]
fn c24dc12dc15dc18() -> PolarsResult<()> {
    check!(C24C12C15C18, 345.3157);
    Ok(())
}

#[test]
fn c24dc9dc12dc15dc18() -> PolarsResult<()> {
    check!(C24C9C12C15C18, 343.3001);
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18() -> PolarsResult<()> {
    check!(C24C6C9C12C15C18, 341.2844);
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18dc21() -> PolarsResult<()> {
    check!(C24C6C9C12C15C18C21, 339.2688);
    Ok(())
}

#[test]
fn c25() -> PolarsResult<()> {
    check!(C25U0, 365.3783);
    Ok(())
}

#[test]
fn c26() -> PolarsResult<()> {
    check!(C26U0, 379.3940);
    Ok(())
}

#[test]
fn c26dc17() -> PolarsResult<()> {
    check!(C26C17, 377.3783);
    Ok(())
}

#[test]
fn c27() -> PolarsResult<()> {
    check!(C27U0, 393.4096);
    Ok(())
}

#[test]
fn c28() -> PolarsResult<()> {
    check!(C28U0, 407.4253);
    Ok(())
}

#[test]
fn c29() -> PolarsResult<()> {
    check!(C29U0, 421.4409);
    Ok(())
}

#[test]
fn c30() -> PolarsResult<()> {
    check!(C30U0, 435.4566);
    Ok(())
}

#[test]
fn c30dc21() -> PolarsResult<()> {
    check!(C30C21, 433.4409);
    Ok(())
}

#[test]
fn c31() -> PolarsResult<()> {
    check!(C31U0, 449.4722);
    Ok(())
}

#[test]
fn c32() -> PolarsResult<()> {
    check!(C32U0, 463.4879);
    Ok(())
}

#[test]
fn c33() -> PolarsResult<()> {
    check!(C33U0, 477.5035);
    Ok(())
}

#[test]
fn c34() -> PolarsResult<()> {
    check!(C34U0, 491.5192);
    Ok(())
}

#[test]
fn c35() -> PolarsResult<()> {
    check!(C35U0, 505.5348);
    Ok(())
}

#[test]
fn c36() -> PolarsResult<()> {
    check!(C36U0, 519.5505);
    Ok(())
}
