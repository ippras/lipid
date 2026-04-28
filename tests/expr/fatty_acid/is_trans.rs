use super::*;

macro_rules! check {
    ($identifier:ident, $expected:expr) => {{
        let data_frame = fatty_acid($identifier.clone())?
            .lazy()
            .select([col(FATTY_ACID).fatty_acid().is_trans().alias("IsTrans")])
            .collect()?;
        let is_trans = data_frame["IsTrans"].bool()?.get(0).unwrap();
        assert!(is_trans == $expected);
    }};
}

#[test]
fn c4() -> PolarsResult<()> {
    check!(C4, false);
    Ok(())
}

#[test]
fn c5() -> PolarsResult<()> {
    check!(C5, false);
    Ok(())
}

#[test]
fn c6() -> PolarsResult<()> {
    check!(C6, false);
    Ok(())
}

#[test]
fn c7() -> PolarsResult<()> {
    check!(C7, false);
    Ok(())
}

#[test]
fn c8() -> PolarsResult<()> {
    check!(C8, false);
    Ok(())
}

#[test]
fn c9() -> PolarsResult<()> {
    check!(C9, false);
    Ok(())
}

#[test]
fn c10() -> PolarsResult<()> {
    check!(C10, false);
    Ok(())
}

#[test]
fn c11() -> PolarsResult<()> {
    check!(C11, false);
    Ok(())
}

#[test]
fn c12() -> PolarsResult<()> {
    check!(C12, false);
    Ok(())
}

#[test]
fn c13() -> PolarsResult<()> {
    check!(C13, false);
    Ok(())
}

#[test]
fn c14() -> PolarsResult<()> {
    check!(C14, false);
    Ok(())
}

#[test]
fn c15() -> PolarsResult<()> {
    check!(C15, false);
    Ok(())
}

#[test]
fn c16() -> PolarsResult<()> {
    check!(C16, false);
    Ok(())
}

#[test]
fn c16dc9() -> PolarsResult<()> {
    check!(C16C9, false);
    Ok(())
}

#[test]
fn c16dt9() -> PolarsResult<()> {
    check!(C16T9, true);
    Ok(())
}

#[test]
fn c17() -> PolarsResult<()> {
    check!(C17, false);
    Ok(())
}

#[test]
fn c18() -> PolarsResult<()> {
    check!(C18, false);
    Ok(())
}

#[test]
fn c18dc9() -> PolarsResult<()> {
    check!(C18C9, false);
    Ok(())
}

#[test]
fn c18dt9() -> PolarsResult<()> {
    check!(C18T9, true);
    Ok(())
}

#[test]
fn c18dc9dc12() -> PolarsResult<()> {
    check!(C18C9C12, false);
    Ok(())
}

#[test]
fn c18dc6dc9dc12() -> PolarsResult<()> {
    check!(C18C6C9C12, false);
    Ok(())
}

#[test]
fn c18dc8dt10dc12() -> PolarsResult<()> {
    check!(C18C8T10C12, true);
    Ok(())
}

#[test]
fn c18dc9dc12dc15() -> PolarsResult<()> {
    check!(C18C9C12C15, false);
    Ok(())
}

#[test]
fn c18dc9dt11dt13() -> PolarsResult<()> {
    check!(C18C9T11T13, true);
    Ok(())
}

#[test]
fn c18dt9dt11dc13() -> PolarsResult<()> {
    check!(C18T9T11C13, true);
    Ok(())
}

#[test]
fn c18dt9dt11dt13() -> PolarsResult<()> {
    check!(C18T9T11T13, true);
    Ok(())
}

#[test]
fn c18dc6dc9dc12dc15() -> PolarsResult<()> {
    check!(C18C6C9C12C15, false);
    Ok(())
}

#[test]
fn c19() -> PolarsResult<()> {
    check!(C19, false);
    Ok(())
}

#[test]
fn c20() -> PolarsResult<()> {
    check!(C20, false);
    Ok(())
}

#[test]
fn c20dc9() -> PolarsResult<()> {
    check!(C20C9, false);
    Ok(())
}

#[test]
fn c20dc11() -> PolarsResult<()> {
    check!(C20C11, false);
    Ok(())
}

#[test]
fn c20dc11dc14() -> PolarsResult<()> {
    check!(C20C11C14, false);
    Ok(())
}

#[test]
fn c20dc5dc8dc11() -> PolarsResult<()> {
    check!(C20C5C8C11, false);
    Ok(())
}

#[test]
fn c20dc8dc11dc14() -> PolarsResult<()> {
    check!(C20C8C11C14, false);
    Ok(())
}

#[test]
fn c20dc11dc14dc17() -> PolarsResult<()> {
    check!(C20C11C14C17, false);
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14() -> PolarsResult<()> {
    check!(C20C5C8C11C14, false);
    Ok(())
}

#[test]
fn c20dc8dc11dc14dc17() -> PolarsResult<()> {
    check!(C20C8C11C14C17, false);
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14dc17() -> PolarsResult<()> {
    check!(C20C5C8C11C14C17, false);
    Ok(())
}

#[test]
fn c21() -> PolarsResult<()> {
    check!(C21, false);
    Ok(())
}

#[test]
fn c22() -> PolarsResult<()> {
    check!(C22, false);
    Ok(())
}

#[test]
fn c22dc13() -> PolarsResult<()> {
    check!(C22C13, false);
    Ok(())
}

#[test]
fn c22dc13dc16() -> PolarsResult<()> {
    check!(C22C13C16, false);
    Ok(())
}

#[test]
fn c22dc5dc13dc16() -> PolarsResult<()> {
    check!(C22C5C13C16, false);
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16() -> PolarsResult<()> {
    check!(C22C7C10C13C16, false);
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check!(C22C7C10C13C16C19, false);
    Ok(())
}

#[test]
fn c22dc4dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check!(C22C4C7C10C13C16C19, false);
    Ok(())
}

#[test]
fn c23() -> PolarsResult<()> {
    check!(C23, false);
    Ok(())
}

#[test]
fn c24() -> PolarsResult<()> {
    check!(C24, false);
    Ok(())
}

#[test]
fn c24dc15() -> PolarsResult<()> {
    check!(C24C15, false);
    Ok(())
}

#[test]
fn c24dc15dc18() -> PolarsResult<()> {
    check!(C24C15C18, false);
    Ok(())
}

#[test]
fn c24dc12dc15dc18() -> PolarsResult<()> {
    check!(C24C12C15C18, false);
    Ok(())
}

#[test]
fn c24dc9dc12dc15dc18() -> PolarsResult<()> {
    check!(C24C9C12C15C18, false);
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18() -> PolarsResult<()> {
    check!(C24C6C9C12C15C18, false);
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18dc21() -> PolarsResult<()> {
    check!(C24C6C9C12C15C18C21, false);
    Ok(())
}

#[test]
fn c25() -> PolarsResult<()> {
    check!(C25, false);
    Ok(())
}

#[test]
fn c26() -> PolarsResult<()> {
    check!(C26, false);
    Ok(())
}

#[test]
fn c26dc17() -> PolarsResult<()> {
    check!(C26C17, false);
    Ok(())
}

#[test]
fn c27() -> PolarsResult<()> {
    check!(C27, false);
    Ok(())
}

#[test]
fn c28() -> PolarsResult<()> {
    check!(C28, false);
    Ok(())
}

#[test]
fn c29() -> PolarsResult<()> {
    check!(C29, false);
    Ok(())
}

#[test]
fn c30() -> PolarsResult<()> {
    check!(C30, false);
    Ok(())
}

#[test]
fn c30dc21() -> PolarsResult<()> {
    check!(C30C21, false);
    Ok(())
}

#[test]
fn c31() -> PolarsResult<()> {
    check!(C31, false);
    Ok(())
}

#[test]
fn c32() -> PolarsResult<()> {
    check!(C32, false);
    Ok(())
}

#[test]
fn c33() -> PolarsResult<()> {
    check!(C33, false);
    Ok(())
}

#[test]
fn c34() -> PolarsResult<()> {
    check!(C34, false);
    Ok(())
}

#[test]
fn c35() -> PolarsResult<()> {
    check!(C35, false);
    Ok(())
}

#[test]
fn c36() -> PolarsResult<()> {
    check!(C36, false);
    Ok(())
}
