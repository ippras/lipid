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
    check!(C4U0, 87.0446);
    Ok(())
}

#[test]
fn c5() -> PolarsResult<()> {
    check!(C5U0, 101.0603);
    Ok(())
}

#[test]
fn c6() -> PolarsResult<()> {
    check!(C6U0, 115.0759);
    Ok(())
}

#[test]
fn c7() -> PolarsResult<()> {
    check!(C7U0, 129.0916);
    Ok(())
}

#[test]
fn c8() -> PolarsResult<()> {
    check!(C8U0, 143.1072);
    Ok(())
}

#[test]
fn c9() -> PolarsResult<()> {
    check!(C9U0, 157.1229);
    Ok(())
}

#[test]
fn c10() -> PolarsResult<()> {
    check!(C10U0, 171.1385);
    Ok(())
}

#[test]
fn c11() -> PolarsResult<()> {
    check!(C11U0, 185.1542);
    Ok(())
}

#[test]
fn c12() -> PolarsResult<()> {
    check!(C12U0, 199.1698);
    Ok(())
}

#[test]
fn c13() -> PolarsResult<()> {
    check!(C13U0, 213.1855);
    Ok(())
}

#[test]
fn c14() -> PolarsResult<()> {
    check!(C14U0, 227.2011);
    Ok(())
}

#[test]
fn c15() -> PolarsResult<()> {
    check!(C15U0, 241.2168);
    Ok(())
}

#[test]
fn c16() -> PolarsResult<()> {
    check!(C16U0, 255.2324);
    Ok(())
}

#[test]
fn c16dc9() -> PolarsResult<()> {
    check!(C16U1C9, 253.2168);
    Ok(())
}

#[test]
fn c16dt9() -> PolarsResult<()> {
    check!(C16U1T9, 253.2168);
    Ok(())
}

#[test]
fn c17() -> PolarsResult<()> {
    check!(C17U0, 269.2481);
    Ok(())
}

#[test]
fn c18() -> PolarsResult<()> {
    check!(C18U0, 283.2637);
    Ok(())
}

#[test]
fn c18dc9() -> PolarsResult<()> {
    check!(C18U1C9, 281.2481);
    Ok(())
}

#[test]
fn c18dt9() -> PolarsResult<()> {
    check!(C18U1T9, 281.2481);
    Ok(())
}

#[test]
fn c18dc9dc12() -> PolarsResult<()> {
    check!(C18U2C9C12, 279.2324);
    Ok(())
}

#[test]
fn c18dc6dc9dc12() -> PolarsResult<()> {
    check!(C18U3C6C9C12, 277.2168);
    Ok(())
}

#[test]
fn c18dc8dt10dc12() -> PolarsResult<()> {
    check!(C18U3C8T10C12, 277.2168);
    Ok(())
}

#[test]
fn c18dc9dc12dc15() -> PolarsResult<()> {
    check!(C18U3C9C12C15, 277.2168);
    Ok(())
}

#[test]
fn c18dc9dt11dt13() -> PolarsResult<()> {
    check!(C18U3C9T11T13, 277.2168);
    Ok(())
}

#[test]
fn c18dt9dt11dc13() -> PolarsResult<()> {
    check!(C18U3T9T11C13, 277.2168);
    Ok(())
}

#[test]
fn c18dt9dt11dt13() -> PolarsResult<()> {
    check!(C18U3T9T11T13, 277.2168);
    Ok(())
}

#[test]
fn c18dc6dc9dc12dc15() -> PolarsResult<()> {
    check!(C18U4C6C9C12C15, 275.2011);
    Ok(())
}

#[test]
fn c19() -> PolarsResult<()> {
    check!(C19U0, 297.2794);
    Ok(())
}

#[test]
fn c20() -> PolarsResult<()> {
    check!(C20U0, 311.2950);
    Ok(())
}

#[test]
fn c20dc9() -> PolarsResult<()> {
    check!(C20U1C9, 309.2794);
    Ok(())
}

#[test]
fn c20dc11() -> PolarsResult<()> {
    check!(C20U1C11, 309.2794);
    Ok(())
}

#[test]
fn c20dc11dc14() -> PolarsResult<()> {
    check!(C20U2C11C14, 307.2637);
    Ok(())
}

#[test]
fn c20dc5dc8dc11() -> PolarsResult<()> {
    check!(C20U3C5C8C11, 305.2481);
    Ok(())
}

#[test]
fn c20dc8dc11dc14() -> PolarsResult<()> {
    check!(C20U3C8C11C14, 305.2481);
    Ok(())
}

#[test]
fn c20dc11dc14dc17() -> PolarsResult<()> {
    check!(C20U3C11C14C17, 305.2481);
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14() -> PolarsResult<()> {
    check!(C20U4C5C8C11C14, 303.2324);
    Ok(())
}

#[test]
fn c20dc8dc11dc14dc17() -> PolarsResult<()> {
    check!(C20U4C8C11C14C17, 303.2324);
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14dc17() -> PolarsResult<()> {
    check!(C20U5C5C8C11C14C17, 301.2168);
    Ok(())
}

#[test]
fn c21() -> PolarsResult<()> {
    check!(C21U0, 325.3107);
    Ok(())
}

#[test]
fn c22() -> PolarsResult<()> {
    check!(C22U0, 339.3263);
    Ok(())
}

#[test]
fn c22dc13() -> PolarsResult<()> {
    check!(C22U1C13, 337.3107);
    Ok(())
}

#[test]
fn c22dc13dc16() -> PolarsResult<()> {
    check!(C22U2C13C16, 335.2950);
    Ok(())
}

#[test]
fn c22dc5dc13dc16() -> PolarsResult<()> {
    check!(C22U3C5C13C16, 333.2794);
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16() -> PolarsResult<()> {
    check!(C22U4C7C10C13C16, 331.2637);
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check!(C22U5C7C10C13C16C19, 329.2481);
    Ok(())
}

#[test]
fn c22dc4dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check!(C22U6C4C7C10C13C16C19, 327.2324);
    Ok(())
}

#[test]
fn c23() -> PolarsResult<()> {
    check!(C23U0, 353.3420);
    Ok(())
}

#[test]
fn c24() -> PolarsResult<()> {
    check!(C24U0, 367.3576);
    Ok(())
}

#[test]
fn c24dc15() -> PolarsResult<()> {
    check!(C24U1C15, 365.3420);
    Ok(())
}

#[test]
fn c24dc15dc18() -> PolarsResult<()> {
    check!(C24U2C15C18, 363.3263);
    Ok(())
}

#[test]
fn c24dc12dc15dc18() -> PolarsResult<()> {
    check!(C24U3C12C15C18, 361.3107);
    Ok(())
}

#[test]
fn c24dc9dc12dc15dc18() -> PolarsResult<()> {
    check!(C24U4C9C12C15C18, 359.2950);
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18() -> PolarsResult<()> {
    check!(C24U5C6C9C12C15C18, 357.2794);
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18dc21() -> PolarsResult<()> {
    check!(C24U6C6C9C12C15C18C21, 355.2637);
    Ok(())
}

#[test]
fn c25() -> PolarsResult<()> {
    check!(C25U0, 381.3733);
    Ok(())
}

#[test]
fn c26() -> PolarsResult<()> {
    check!(C26U0, 395.3889);
    Ok(())
}

#[test]
fn c26dc17() -> PolarsResult<()> {
    check!(C26U1C17, 393.3733);
    Ok(())
}

#[test]
fn c27() -> PolarsResult<()> {
    check!(C27U0, 409.4046);
    Ok(())
}

#[test]
fn c28() -> PolarsResult<()> {
    check!(C28U0, 423.4202);
    Ok(())
}

#[test]
fn c29() -> PolarsResult<()> {
    check!(C29U0, 437.4359);
    Ok(())
}

#[test]
fn c30() -> PolarsResult<()> {
    check!(C30U0, 451.4515);
    Ok(())
}

#[test]
fn c30dc21() -> PolarsResult<()> {
    check!(C30U1C21, 449.4359);
    Ok(())
}

#[test]
fn c31() -> PolarsResult<()> {
    check!(C31U0, 465.4672);
    Ok(())
}

#[test]
fn c32() -> PolarsResult<()> {
    check!(C32U0, 479.4828);
    Ok(())
}

#[test]
fn c33() -> PolarsResult<()> {
    check!(C33U0, 493.4985);
    Ok(())
}

#[test]
fn c34() -> PolarsResult<()> {
    check!(C34U0, 507.5141);
    Ok(())
}

#[test]
fn c35() -> PolarsResult<()> {
    check!(C35U0, 521.5298);
    Ok(())
}

#[test]
fn c36() -> PolarsResult<()> {
    check!(C36U0, 535.5454);
    Ok(())
}
