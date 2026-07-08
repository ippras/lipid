use super::*;

macro_rules! check {
    ($identifier:ident, $expected:expr) => {{
        let data_frame = fatty_acid($identifier.clone())?
            .lazy()
            .select([col(FATTY_ACID).fatty_acid().carbon().alias("Carbon")])
            .collect()?;
        let carbon = data_frame["Carbon"].u8()?.get(0).unwrap();
        assert_eq!(carbon, $expected);
    }};
}

#[test]
fn c4() -> PolarsResult<()> {
    check!(C4U0, 4);
    Ok(())
}

#[test]
fn c5() -> PolarsResult<()> {
    check!(C5U0, 5);
    Ok(())
}

#[test]
fn c6() -> PolarsResult<()> {
    check!(C6U0, 6);
    Ok(())
}

#[test]
fn c7() -> PolarsResult<()> {
    check!(C7U0, 7);
    Ok(())
}

#[test]
fn c8() -> PolarsResult<()> {
    check!(C8U0, 8);
    Ok(())
}

#[test]
fn c9() -> PolarsResult<()> {
    check!(C9U0, 9);
    Ok(())
}

#[test]
fn c10() -> PolarsResult<()> {
    check!(C10U0, 10);
    Ok(())
}

#[test]
fn c11() -> PolarsResult<()> {
    check!(C11U0, 11);
    Ok(())
}

#[test]
fn c12() -> PolarsResult<()> {
    check!(C12U0, 12);
    Ok(())
}

#[test]
fn c13() -> PolarsResult<()> {
    check!(C13U0, 13);
    Ok(())
}

#[test]
fn c14() -> PolarsResult<()> {
    check!(C14U0, 14);
    Ok(())
}

#[test]
fn c15() -> PolarsResult<()> {
    check!(C15U0, 15);
    Ok(())
}

#[test]
fn c16() -> PolarsResult<()> {
    check!(C16U0, 16);
    Ok(())
}

#[test]
fn c16dc9() -> PolarsResult<()> {
    check!(C16U1C9, 16);
    Ok(())
}

#[test]
fn c16dt9() -> PolarsResult<()> {
    check!(C16U1T9, 16);
    Ok(())
}

#[test]
fn c17() -> PolarsResult<()> {
    check!(C17U0, 17);
    Ok(())
}

#[test]
fn c18() -> PolarsResult<()> {
    check!(C18U0, 18);
    Ok(())
}

#[test]
fn c18dc9() -> PolarsResult<()> {
    check!(C18U1C9, 18);
    Ok(())
}

#[test]
fn c18dt9() -> PolarsResult<()> {
    check!(C18U1T9, 18);
    Ok(())
}

#[test]
fn c18dc9dc12() -> PolarsResult<()> {
    check!(C18U2C9C12, 18);
    Ok(())
}

#[test]
fn c18dc6dc9dc12() -> PolarsResult<()> {
    check!(C18U3C6C9C12, 18);
    Ok(())
}

#[test]
fn c18dc8dt10dc12() -> PolarsResult<()> {
    check!(C18U3C8T10C12, 18);
    Ok(())
}

#[test]
fn c18dc9dc12dc15() -> PolarsResult<()> {
    check!(C18U3C9C12C15, 18);
    Ok(())
}

#[test]
fn c18dc9dt11dt13() -> PolarsResult<()> {
    check!(C18U3C9T11T13, 18);
    Ok(())
}

#[test]
fn c18dt9dt11dc13() -> PolarsResult<()> {
    check!(C18U3T9T11C13, 18);
    Ok(())
}

#[test]
fn c18dt9dt11dt13() -> PolarsResult<()> {
    check!(C18U3T9T11T13, 18);
    Ok(())
}

#[test]
fn c18dc6dc9dc12dc15() -> PolarsResult<()> {
    check!(C18U4C6C9C12C15, 18);
    Ok(())
}

#[test]
fn c19() -> PolarsResult<()> {
    check!(C19U0, 19);
    Ok(())
}

#[test]
fn c20() -> PolarsResult<()> {
    check!(C20U0, 20);
    Ok(())
}

#[test]
fn c20dc9() -> PolarsResult<()> {
    check!(C20U1C9, 20);
    Ok(())
}

#[test]
fn c20dc11() -> PolarsResult<()> {
    check!(C20U1C11, 20);
    Ok(())
}

#[test]
fn c20dc11dc14() -> PolarsResult<()> {
    check!(C20U2C11C14, 20);
    Ok(())
}

#[test]
fn c20dc5dc8dc11() -> PolarsResult<()> {
    check!(C20U3C5C8C11, 20);
    Ok(())
}

#[test]
fn c20dc8dc11dc14() -> PolarsResult<()> {
    check!(C20U3C8C11C14, 20);
    Ok(())
}

#[test]
fn c20dc11dc14dc17() -> PolarsResult<()> {
    check!(C20U3C11C14C17, 20);
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14() -> PolarsResult<()> {
    check!(C20U4C5C8C11C14, 20);
    Ok(())
}

#[test]
fn c20dc8dc11dc14dc17() -> PolarsResult<()> {
    check!(C20U4C8C11C14C17, 20);
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14dc17() -> PolarsResult<()> {
    check!(C20U5C5C8C11C14C17, 20);
    Ok(())
}

#[test]
fn c21() -> PolarsResult<()> {
    check!(C21U0, 21);
    Ok(())
}

#[test]
fn c22() -> PolarsResult<()> {
    check!(C22U0, 22);
    Ok(())
}

#[test]
fn c22dc13() -> PolarsResult<()> {
    check!(C22U1C13, 22);
    Ok(())
}

#[test]
fn c22dc13dc16() -> PolarsResult<()> {
    check!(C22U2C13C16, 22);
    Ok(())
}

#[test]
fn c22dc5dc13dc16() -> PolarsResult<()> {
    check!(C22U3C5C13C16, 22);
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16() -> PolarsResult<()> {
    check!(C22U4C7C10C13C16, 22);
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check!(C22U5C7C10C13C16C19, 22);
    Ok(())
}

#[test]
fn c22dc4dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check!(C22U6C4C7C10C13C16C19, 22);
    Ok(())
}

#[test]
fn c23() -> PolarsResult<()> {
    check!(C23U0, 23);
    Ok(())
}

#[test]
fn c24() -> PolarsResult<()> {
    check!(C24U0, 24);
    Ok(())
}

#[test]
fn c24dc15() -> PolarsResult<()> {
    check!(C24U1C15, 24);
    Ok(())
}

#[test]
fn c24dc15dc18() -> PolarsResult<()> {
    check!(C24U2C15C18, 24);
    Ok(())
}

#[test]
fn c24dc12dc15dc18() -> PolarsResult<()> {
    check!(C24U3C12C15C18, 24);
    Ok(())
}

#[test]
fn c24dc9dc12dc15dc18() -> PolarsResult<()> {
    check!(C24U4C9C12C15C18, 24);
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18() -> PolarsResult<()> {
    check!(C24U5C6C9C12C15C18, 24);
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18dc21() -> PolarsResult<()> {
    check!(C24U6C6C9C12C15C18C21, 24);
    Ok(())
}

#[test]
fn c25() -> PolarsResult<()> {
    check!(C25U0, 25);
    Ok(())
}

#[test]
fn c26() -> PolarsResult<()> {
    check!(C26U0, 26);
    Ok(())
}

#[test]
fn c26dc17() -> PolarsResult<()> {
    check!(C26U1C17, 26);
    Ok(())
}

#[test]
fn c27() -> PolarsResult<()> {
    check!(C27U0, 27);
    Ok(())
}

#[test]
fn c28() -> PolarsResult<()> {
    check!(C28U0, 28);
    Ok(())
}

#[test]
fn c29() -> PolarsResult<()> {
    check!(C29U0, 29);
    Ok(())
}

#[test]
fn c30() -> PolarsResult<()> {
    check!(C30U0, 30);
    Ok(())
}

#[test]
fn c30dc21() -> PolarsResult<()> {
    check!(C30U1C21, 30);
    Ok(())
}

#[test]
fn c31() -> PolarsResult<()> {
    check!(C31U0, 31);
    Ok(())
}

#[test]
fn c32() -> PolarsResult<()> {
    check!(C32U0, 32);
    Ok(())
}

#[test]
fn c33() -> PolarsResult<()> {
    check!(C33U0, 33);
    Ok(())
}

#[test]
fn c34() -> PolarsResult<()> {
    check!(C34U0, 34);
    Ok(())
}

#[test]
fn c35() -> PolarsResult<()> {
    check!(C35U0, 35);
    Ok(())
}

#[test]
fn c36() -> PolarsResult<()> {
    check!(C36U0, 36);
    Ok(())
}
