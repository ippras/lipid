use super::*;

macro_rules! check {
    ($identifier:ident, $expected:expr) => {{
        let data_frame = fatty_acid($identifier.clone())?
            .lazy()
            .select([col(FATTY_ACID)
                .fatty_acid()
                .rcoo()
                .relative_atomic_mass(None)
                .round(DECIMALS, RoundMode::HalfToEven)])
            .collect()?;
        let mass = data_frame["Mass"].f64()?.get(0).unwrap();
        assert_epsilon!(mass, $expected);
    }};
}

#[test]
fn c4() -> PolarsResult<()> {
    check!(C4, 87.0446);
    Ok(())
}

#[test]
fn c5() -> PolarsResult<()> {
    check!(C5, 101.0603);
    Ok(())
}

#[test]
fn c6() -> PolarsResult<()> {
    check!(C6, 115.0759);
    Ok(())
}

#[test]
fn c7() -> PolarsResult<()> {
    check!(C7, 129.0916);
    Ok(())
}

#[test]
fn c8() -> PolarsResult<()> {
    check!(C8, 143.1072);
    Ok(())
}

#[test]
fn c9() -> PolarsResult<()> {
    check!(C9, 157.1229);
    Ok(())
}

#[test]
fn c10() -> PolarsResult<()> {
    check!(C10, 171.1385);
    Ok(())
}

#[test]
fn c11() -> PolarsResult<()> {
    check!(C11, 185.1542);
    Ok(())
}

#[test]
fn c12() -> PolarsResult<()> {
    check!(C12, 199.1698);
    Ok(())
}

#[test]
fn c13() -> PolarsResult<()> {
    check!(C13, 213.1855);
    Ok(())
}

#[test]
fn c14() -> PolarsResult<()> {
    check!(C14, 227.2011);
    Ok(())
}

#[test]
fn c15() -> PolarsResult<()> {
    check!(C15, 241.2168);
    Ok(())
}

#[test]
fn c16() -> PolarsResult<()> {
    check!(C16, 255.2324);
    Ok(())
}

#[test]
fn c16dc9() -> PolarsResult<()> {
    check!(C16DC9, 253.2168);
    Ok(())
}

#[test]
fn c16dt9() -> PolarsResult<()> {
    check!(C16DT9, 253.2168);
    Ok(())
}

#[test]
fn c17() -> PolarsResult<()> {
    check!(C17, 269.2481);
    Ok(())
}

#[test]
fn c18() -> PolarsResult<()> {
    check!(C18, 283.2637);
    Ok(())
}

#[test]
fn c18dc9() -> PolarsResult<()> {
    check!(C18DC9, 281.2481);
    Ok(())
}

#[test]
fn c18dt9() -> PolarsResult<()> {
    check!(C18DT9, 281.2481);
    Ok(())
}

#[test]
fn c18dc9dc12() -> PolarsResult<()> {
    check!(C18DC9DC12, 279.2324);
    Ok(())
}

#[test]
fn c18dc6dc9dc12() -> PolarsResult<()> {
    check!(C18DC6DC9DC12, 277.2168);
    Ok(())
}

#[test]
fn c18dc8dt10dc12() -> PolarsResult<()> {
    check!(C18DC8DT10DC12, 277.2168);
    Ok(())
}

#[test]
fn c18dc9dc12dc15() -> PolarsResult<()> {
    check!(C18DC9DC12DC15, 277.2168);
    Ok(())
}

#[test]
fn c18dc9dt11dt13() -> PolarsResult<()> {
    check!(C18DC9DT11DT13, 277.2168);
    Ok(())
}

#[test]
fn c18dt9dt11dc13() -> PolarsResult<()> {
    check!(C18DT9DT11DC13, 277.2168);
    Ok(())
}

#[test]
fn c18dt9dt11dt13() -> PolarsResult<()> {
    check!(C18DT9DT11DT13, 277.2168);
    Ok(())
}

#[test]
fn c18dc6dc9dc12dc15() -> PolarsResult<()> {
    check!(C18DC6DC9DC12DC15, 275.2011);
    Ok(())
}

#[test]
fn c19() -> PolarsResult<()> {
    check!(C19, 297.2794);
    Ok(())
}

#[test]
fn c20() -> PolarsResult<()> {
    check!(C20, 311.2950);
    Ok(())
}

#[test]
fn c20dc9() -> PolarsResult<()> {
    check!(C20DC9, 309.2794);
    Ok(())
}

#[test]
fn c20dc11() -> PolarsResult<()> {
    check!(C20DC11, 309.2794);
    Ok(())
}

#[test]
fn c20dc11dc14() -> PolarsResult<()> {
    check!(C20DC11DC14, 307.2637);
    Ok(())
}

#[test]
fn c20dc5dc8dc11() -> PolarsResult<()> {
    check!(C20DC5DC8DC11, 305.2481);
    Ok(())
}

#[test]
fn c20dc8dc11dc14() -> PolarsResult<()> {
    check!(C20DC8DC11DC14, 305.2481);
    Ok(())
}

#[test]
fn c20dc11dc14dc17() -> PolarsResult<()> {
    check!(C20DC11DC14DC17, 305.2481);
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14() -> PolarsResult<()> {
    check!(C20DC5DC8DC11DC14, 303.2324);
    Ok(())
}

#[test]
fn c20dc8dc11dc14dc17() -> PolarsResult<()> {
    check!(C20DC8DC11DC14DC17, 303.2324);
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14dc17() -> PolarsResult<()> {
    check!(C20DC5DC8DC11DC14DC17, 301.2168);
    Ok(())
}

#[test]
fn c21() -> PolarsResult<()> {
    check!(C21, 325.3107);
    Ok(())
}

#[test]
fn c22() -> PolarsResult<()> {
    check!(C22, 339.3263);
    Ok(())
}

#[test]
fn c22dc13() -> PolarsResult<()> {
    check!(C22DC13, 337.3107);
    Ok(())
}

#[test]
fn c22dc13dc16() -> PolarsResult<()> {
    check!(C22DC13DC16, 335.2950);
    Ok(())
}

#[test]
fn c22dc5dc13dc16() -> PolarsResult<()> {
    check!(C22DC5DC13DC16, 333.2794);
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16() -> PolarsResult<()> {
    check!(C22DC7DC10DC13DC16, 331.2637);
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check!(C22DC7DC10DC13DC16DC19, 329.2481);
    Ok(())
}

#[test]
fn c22dc4dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check!(C22DC4DC7DC10DC13DC16DC19, 327.2324);
    Ok(())
}

#[test]
fn c23() -> PolarsResult<()> {
    check!(C23, 353.3420);
    Ok(())
}

#[test]
fn c24() -> PolarsResult<()> {
    check!(C24, 367.3576);
    Ok(())
}

#[test]
fn c24dc15() -> PolarsResult<()> {
    check!(C24DC15, 365.3420);
    Ok(())
}

#[test]
fn c24dc15dc18() -> PolarsResult<()> {
    check!(C24DC15DC18, 363.3263);
    Ok(())
}

#[test]
fn c24dc12dc15dc18() -> PolarsResult<()> {
    check!(C24DC12DC15DC18, 361.3107);
    Ok(())
}

#[test]
fn c24dc9dc12dc15dc18() -> PolarsResult<()> {
    check!(C24DC9DC12DC15DC18, 359.2950);
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18() -> PolarsResult<()> {
    check!(C24DC6DC9DC12DC15DC18, 357.2794);
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18dc21() -> PolarsResult<()> {
    check!(C24DC6DC9DC12DC15DC18DC21, 355.2637);
    Ok(())
}

#[test]
fn c25() -> PolarsResult<()> {
    check!(C25, 381.3733);
    Ok(())
}

#[test]
fn c26() -> PolarsResult<()> {
    check!(C26, 395.3889);
    Ok(())
}

#[test]
fn c26dc17() -> PolarsResult<()> {
    check!(C26DC17, 393.3733);
    Ok(())
}

#[test]
fn c27() -> PolarsResult<()> {
    check!(C27, 409.4046);
    Ok(())
}

#[test]
fn c28() -> PolarsResult<()> {
    check!(C28, 423.4202);
    Ok(())
}

#[test]
fn c29() -> PolarsResult<()> {
    check!(C29, 437.4359);
    Ok(())
}

#[test]
fn c30() -> PolarsResult<()> {
    check!(C30, 451.4515);
    Ok(())
}

#[test]
fn c30dc21() -> PolarsResult<()> {
    check!(C30DC21, 449.4359);
    Ok(())
}

#[test]
fn c31() -> PolarsResult<()> {
    check!(C31, 465.4672);
    Ok(())
}

#[test]
fn c32() -> PolarsResult<()> {
    check!(C32, 479.4828);
    Ok(())
}

#[test]
fn c33() -> PolarsResult<()> {
    check!(C33, 493.4985);
    Ok(())
}

#[test]
fn c34() -> PolarsResult<()> {
    check!(C34, 507.5141);
    Ok(())
}

#[test]
fn c35() -> PolarsResult<()> {
    check!(C35, 521.5298);
    Ok(())
}

#[test]
fn c36() -> PolarsResult<()> {
    check!(C36, 535.5454);
    Ok(())
}
