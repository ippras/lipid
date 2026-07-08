use super::*;

macro_rules! check {
    ($identifier:ident, $expected:expr) => {{
        let data_frame = fatty_acid($identifier.clone())?
            .lazy()
            .select([col(FATTY_ACID)
                .fatty_acid()
                .rcooch3()
                .relative_atomic_mass(None)
                .round(DECIMALS, RoundMode::HalfToEven)])
            .collect()?;
        let mass = data_frame["Mass"].f64()?.get(0).unwrap();
        assert_epsilon!(mass, $expected);
    }};
}

#[test]
fn c4() -> PolarsResult<()> {
    check!(C4U0, 102.0681);
    Ok(())
}

#[test]
fn c5() -> PolarsResult<()> {
    check!(C5U0, 116.0837);
    Ok(())
}

#[test]
fn c6() -> PolarsResult<()> {
    check!(C6U0, 130.0994);
    Ok(())
}

#[test]
fn c7() -> PolarsResult<()> {
    check!(C7U0, 144.1150);
    Ok(())
}

#[test]
fn c8() -> PolarsResult<()> {
    check!(C8U0, 158.1307);
    Ok(())
}

#[test]
fn c9() -> PolarsResult<()> {
    check!(C9U0, 172.1463);
    Ok(())
}

#[test]
fn c10() -> PolarsResult<()> {
    check!(C10U0, 186.1620);
    Ok(())
}

#[test]
fn c11() -> PolarsResult<()> {
    check!(C11U0, 200.1776);
    Ok(())
}

#[test]
fn c12() -> PolarsResult<()> {
    check!(C12U0, 214.1933);
    Ok(())
}

#[test]
fn c13() -> PolarsResult<()> {
    check!(C13U0, 228.2089);
    Ok(())
}

#[test]
fn c14() -> PolarsResult<()> {
    check!(C14U0, 242.2246);
    Ok(())
}

#[test]
fn c15() -> PolarsResult<()> {
    check!(C15U0, 256.2402);
    Ok(())
}

#[test]
fn c16() -> PolarsResult<()> {
    check!(C16U0, 270.2559);
    Ok(())
}

#[test]
fn c16dc9() -> PolarsResult<()> {
    check!(C16C9, 268.2402);
    Ok(())
}

#[test]
fn c16dt9() -> PolarsResult<()> {
    check!(C16T9, 268.2402);
    Ok(())
}

#[test]
fn c17() -> PolarsResult<()> {
    check!(C17U0, 284.2715);
    Ok(())
}

#[test]
fn c18() -> PolarsResult<()> {
    check!(C18U0, 298.2872);
    Ok(())
}

#[test]
fn c18dc9() -> PolarsResult<()> {
    check!(C18C9, 296.2715);
    Ok(())
}

#[test]
fn c18dt9() -> PolarsResult<()> {
    check!(C18T9, 296.2715);
    Ok(())
}

#[test]
fn c18dc9dc12() -> PolarsResult<()> {
    check!(C18C9C12, 294.2559);
    Ok(())
}

#[test]
fn c18dc6dc9dc12() -> PolarsResult<()> {
    check!(C18C6C9C12, 292.2402);
    Ok(())
}

#[test]
fn c18dc8dt10dc12() -> PolarsResult<()> {
    check!(C18C8T10C12, 292.2402);
    Ok(())
}

#[test]
fn c18dc9dc12dc15() -> PolarsResult<()> {
    check!(C18C9C12C15, 292.2402);
    Ok(())
}

#[test]
fn c18dc9dt11dt13() -> PolarsResult<()> {
    check!(C18C9T11T13, 292.2402);
    Ok(())
}

#[test]
fn c18dt9dt11dc13() -> PolarsResult<()> {
    check!(C18T9T11C13, 292.2402);
    Ok(())
}

#[test]
fn c18dt9dt11dt13() -> PolarsResult<()> {
    check!(C18T9T11T13, 292.2402);
    Ok(())
}

#[test]
fn c18dc6dc9dc12dc15() -> PolarsResult<()> {
    check!(C18C6C9C12C15, 290.2246);
    Ok(())
}

#[test]
fn c19() -> PolarsResult<()> {
    check!(C19U0, 312.3028);
    Ok(())
}

#[test]
fn c20() -> PolarsResult<()> {
    check!(C20U0, 326.3185);
    Ok(())
}

#[test]
fn c20dc9() -> PolarsResult<()> {
    check!(C20C9, 324.3028);
    Ok(())
}

#[test]
fn c20dc11() -> PolarsResult<()> {
    check!(C20C11, 324.3028);
    Ok(())
}

#[test]
fn c20dc11dc14() -> PolarsResult<()> {
    check!(C20C11C14, 322.2872);
    Ok(())
}

#[test]
fn c20dc5dc8dc11() -> PolarsResult<()> {
    check!(C20C5C8C11, 320.2715);
    Ok(())
}

#[test]
fn c20dc8dc11dc14() -> PolarsResult<()> {
    check!(C20C8C11C14, 320.2715);
    Ok(())
}

#[test]
fn c20dc11dc14dc17() -> PolarsResult<()> {
    check!(C20C11C14C17, 320.2715);
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14() -> PolarsResult<()> {
    check!(C20C5C8C11C14, 318.2559);
    Ok(())
}

#[test]
fn c20dc8dc11dc14dc17() -> PolarsResult<()> {
    check!(C20C8C11C14C17, 318.2559);
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14dc17() -> PolarsResult<()> {
    check!(C20C5C8C11C14C17, 316.2402);
    Ok(())
}

#[test]
fn c21() -> PolarsResult<()> {
    check!(C21U0, 340.3341);
    Ok(())
}

#[test]
fn c22() -> PolarsResult<()> {
    check!(C22U0, 354.3498);
    Ok(())
}

#[test]
fn c22dc13() -> PolarsResult<()> {
    check!(C22C13, 352.3341);
    Ok(())
}

#[test]
fn c22dc13dc16() -> PolarsResult<()> {
    check!(C22C13C16, 350.3185);
    Ok(())
}

#[test]
fn c22dc5dc13dc16() -> PolarsResult<()> {
    check!(C22C5C13C16, 348.3028);
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16() -> PolarsResult<()> {
    check!(C22C7C10C13C16, 346.2872);
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check!(C22C7C10C13C16C19, 344.2715);
    Ok(())
}

#[test]
fn c22dc4dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check!(C22C4C7C10C13C16C19, 342.2559);
    Ok(())
}

#[test]
fn c23() -> PolarsResult<()> {
    check!(C23U0, 368.3654);
    Ok(())
}

#[test]
fn c24() -> PolarsResult<()> {
    check!(C24U0, 382.3811);
    Ok(())
}

#[test]
fn c24dc15() -> PolarsResult<()> {
    check!(C24C15, 380.3654);
    Ok(())
}

#[test]
fn c24dc15dc18() -> PolarsResult<()> {
    check!(C24C15C18, 378.3498);
    Ok(())
}

#[test]
fn c24dc12dc15dc18() -> PolarsResult<()> {
    check!(C24C12C15C18, 376.3341);
    Ok(())
}

#[test]
fn c24dc9dc12dc15dc18() -> PolarsResult<()> {
    check!(C24C9C12C15C18, 374.3185);
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18() -> PolarsResult<()> {
    check!(C24C6C9C12C15C18, 372.3028);
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18dc21() -> PolarsResult<()> {
    check!(C24C6C9C12C15C18C21, 370.2872);
    Ok(())
}

#[test]
fn c25() -> PolarsResult<()> {
    check!(C25U0, 396.3967);
    Ok(())
}

#[test]
fn c26() -> PolarsResult<()> {
    check!(C26U0, 410.4124);
    Ok(())
}

#[test]
fn c26dc17() -> PolarsResult<()> {
    check!(C26C17, 408.3967);
    Ok(())
}

#[test]
fn c27() -> PolarsResult<()> {
    check!(C27U0, 424.4280);
    Ok(())
}

#[test]
fn c28() -> PolarsResult<()> {
    check!(C28U0, 438.4437);
    Ok(())
}

#[test]
fn c29() -> PolarsResult<()> {
    check!(C29U0, 452.4593);
    Ok(())
}

#[test]
fn c30() -> PolarsResult<()> {
    check!(C30U0, 466.4750);
    Ok(())
}

#[test]
fn c30dc21() -> PolarsResult<()> {
    check!(C30C21, 464.4593);
    Ok(())
}

#[test]
fn c31() -> PolarsResult<()> {
    check!(C31U0, 480.4906);
    Ok(())
}

#[test]
fn c32() -> PolarsResult<()> {
    check!(C32U0, 494.5063);
    Ok(())
}

#[test]
fn c33() -> PolarsResult<()> {
    check!(C33U0, 508.5219);
    Ok(())
}

#[test]
fn c34() -> PolarsResult<()> {
    check!(C34U0, 522.5376);
    Ok(())
}

#[test]
fn c35() -> PolarsResult<()> {
    check!(C35U0, 536.5532);
    Ok(())
}

#[test]
fn c36() -> PolarsResult<()> {
    check!(C36U0, 550.5689);
    Ok(())
}
