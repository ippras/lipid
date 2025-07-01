use super::*;

macro_rules! check {
    ($identifier:ident, $expected:expr) => {{
        let data_frame = fatty_acid($identifier.clone())?
            .lazy()
            .select([col(FATTY_ACID)
                .fatty_acid()
                .is_unsaturated(NonZero::new(-6))])
            .collect()?;
        let is_unsaturated_6 = data_frame["IsUnsaturated-6"].bool()?.get(0).unwrap();
        assert!(is_unsaturated_6 == $expected);
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
    check!(C16DC9, false);
    Ok(())
}

#[test]
fn c16dt9() -> PolarsResult<()> {
    check!(C16DT9, false);
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
    check!(C18DC9, false);
    Ok(())
}

#[test]
fn c18dt9() -> PolarsResult<()> {
    check!(C18DT9, false);
    Ok(())
}

#[test]
fn c18dc9dc12() -> PolarsResult<()> {
    check!(C18DC9DC12, true);
    Ok(())
}

#[test]
fn c18dc6dc9dc12() -> PolarsResult<()> {
    check!(C18DC6DC9DC12, true);
    Ok(())
}

#[test]
fn c18dc8dt10dc12() -> PolarsResult<()> {
    check!(C18DC8DT10DC12, true);
    Ok(())
}

#[test]
fn c18dc9dc12dc15() -> PolarsResult<()> {
    check!(C18DC9DC12DC15, false);
    Ok(())
}

#[test]
fn c18dc9dt11dt13() -> PolarsResult<()> {
    check!(C18DC9DT11DT13, false);
    Ok(())
}

#[test]
fn c18dt9dt11dc13() -> PolarsResult<()> {
    check!(C18DT9DT11DC13, false);
    Ok(())
}

#[test]
fn c18dt9dt11dt13() -> PolarsResult<()> {
    check!(C18DT9DT11DT13, false);
    Ok(())
}

#[test]
fn c18dc6dc9dc12dc15() -> PolarsResult<()> {
    check!(C18DC6DC9DC12DC15, false);
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
    check!(C20DC9, false);
    Ok(())
}

#[test]
fn c20dc11() -> PolarsResult<()> {
    check!(C20DC11, false);
    Ok(())
}

#[test]
fn c20dc11dc14() -> PolarsResult<()> {
    check!(C20DC11DC14, true);
    Ok(())
}

#[test]
fn c20dc5dc8dc11() -> PolarsResult<()> {
    check!(C20DC5DC8DC11, false);
    Ok(())
}

#[test]
fn c20dc8dc11dc14() -> PolarsResult<()> {
    check!(C20DC8DC11DC14, true);
    Ok(())
}

#[test]
fn c20dc11dc14dc17() -> PolarsResult<()> {
    check!(C20DC11DC14DC17, false);
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14() -> PolarsResult<()> {
    check!(C20DC5DC8DC11DC14, true);
    Ok(())
}

#[test]
fn c20dc8dc11dc14dc17() -> PolarsResult<()> {
    check!(C20DC8DC11DC14DC17, false);
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14dc17() -> PolarsResult<()> {
    check!(C20DC5DC8DC11DC14DC17, false);
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
    check!(C22DC13, false);
    Ok(())
}

#[test]
fn c22dc13dc16() -> PolarsResult<()> {
    check!(C22DC13DC16, true);
    Ok(())
}

#[test]
fn c22dc5dc13dc16() -> PolarsResult<()> {
    check!(C22DC5DC13DC16, true);
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16() -> PolarsResult<()> {
    check!(C22DC7DC10DC13DC16, true);
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check!(C22DC7DC10DC13DC16DC19, false);
    Ok(())
}

#[test]
fn c22dc4dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check!(C22DC4DC7DC10DC13DC16DC19, false);
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
    check!(C24DC15, false);
    Ok(())
}

#[test]
fn c24dc15dc18() -> PolarsResult<()> {
    check!(C24DC15DC18, true);
    Ok(())
}

#[test]
fn c24dc12dc15dc18() -> PolarsResult<()> {
    check!(C24DC12DC15DC18, true);
    Ok(())
}

#[test]
fn c24dc9dc12dc15dc18() -> PolarsResult<()> {
    check!(C24DC9DC12DC15DC18, true);
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18() -> PolarsResult<()> {
    check!(C24DC6DC9DC12DC15DC18, true);
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18dc21() -> PolarsResult<()> {
    check!(C24DC6DC9DC12DC15DC18DC21, false);
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
    check!(C26DC17, false);
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
    check!(C30DC21, false);
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
