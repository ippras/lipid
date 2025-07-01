use lipid::prelude::*;
use polars::prelude::*;

macro_rules! check {
    ($identifier:ident, $expected:literal) => {{
        let series = Series::from_any_values(PlSmallStr::EMPTY, &[$identifier.clone()], true)?;
        let fatty_acid = series.fatty_acid();
        let hydrogen = fatty_acid.hydrogen()?;
        assert!(hydrogen.first() == Some($expected));
    }};
}

#[test]
fn c4() -> PolarsResult<()> {
    check!(C4, 8);
    Ok(())
}

#[test]
fn c5() -> PolarsResult<()> {
    check!(C5, 10);
    Ok(())
}

#[test]
fn c6() -> PolarsResult<()> {
    check!(C6, 12);
    Ok(())
}

#[test]
fn c7() -> PolarsResult<()> {
    check!(C7, 14);
    Ok(())
}

#[test]
fn c8() -> PolarsResult<()> {
    check!(C8, 16);
    Ok(())
}

#[test]
fn c9() -> PolarsResult<()> {
    check!(C9, 18);
    Ok(())
}

#[test]
fn c10() -> PolarsResult<()> {
    check!(C10, 20);
    Ok(())
}

#[test]
fn c11() -> PolarsResult<()> {
    check!(C11, 22);
    Ok(())
}

#[test]
fn c12() -> PolarsResult<()> {
    check!(C12, 24);
    Ok(())
}

#[test]
fn c13() -> PolarsResult<()> {
    check!(C13, 26);
    Ok(())
}

#[test]
fn c14() -> PolarsResult<()> {
    check!(C14, 28);
    Ok(())
}

#[test]
fn c15() -> PolarsResult<()> {
    check!(C15, 30);
    Ok(())
}

#[test]
fn c16() -> PolarsResult<()> {
    check!(C16, 32);
    Ok(())
}

#[test]
fn c16dc9() -> PolarsResult<()> {
    check!(C16DC9, 30);
    Ok(())
}

#[test]
fn c16dt9() -> PolarsResult<()> {
    check!(C16DT9, 30);
    Ok(())
}

#[test]
fn c17() -> PolarsResult<()> {
    check!(C17, 34);
    Ok(())
}

#[test]
fn c18() -> PolarsResult<()> {
    check!(C18, 36);
    Ok(())
}

#[test]
fn c18dc9() -> PolarsResult<()> {
    check!(C18DC9, 34);
    Ok(())
}

#[test]
fn c18dt9() -> PolarsResult<()> {
    check!(C18DT9, 34);
    Ok(())
}

#[test]
fn c18dc9dc12() -> PolarsResult<()> {
    check!(C18DC9DC12, 32);
    Ok(())
}

#[test]
fn c18dc6dc9dc12() -> PolarsResult<()> {
    check!(C18DC6DC9DC12, 30);
    Ok(())
}

#[test]
fn c18dc8dt10dc12() -> PolarsResult<()> {
    check!(C18DC8DT10DC12, 30);
    Ok(())
}

#[test]
fn c18dc9dc12dc15() -> PolarsResult<()> {
    check!(C18DC9DC12DC15, 30);
    Ok(())
}

#[test]
fn c18dc9dt11dt13() -> PolarsResult<()> {
    check!(C18DC9DT11DT13, 30);
    Ok(())
}

#[test]
fn c18dt9dt11dc13() -> PolarsResult<()> {
    check!(C18DT9DT11DC13, 30);
    Ok(())
}

#[test]
fn c18dt9dt11dt13() -> PolarsResult<()> {
    check!(C18DT9DT11DT13, 30);
    Ok(())
}

#[test]
fn c18dc6dc9dc12dc15() -> PolarsResult<()> {
    check!(C18DC6DC9DC12DC15, 28);
    Ok(())
}

#[test]
fn c19() -> PolarsResult<()> {
    check!(C19, 38);
    Ok(())
}

#[test]
fn c20() -> PolarsResult<()> {
    check!(C20, 40);
    Ok(())
}

#[test]
fn c20dc9() -> PolarsResult<()> {
    check!(C20DC9, 38);
    Ok(())
}

#[test]
fn c20dc11() -> PolarsResult<()> {
    check!(C20DC11, 38);
    Ok(())
}

#[test]
fn c20dc11dc14() -> PolarsResult<()> {
    check!(C20DC11DC14, 36);
    Ok(())
}

#[test]
fn c20dc5dc8dc11() -> PolarsResult<()> {
    check!(C20DC5DC8DC11, 34);
    Ok(())
}

#[test]
fn c20dc8dc11dc14() -> PolarsResult<()> {
    check!(C20DC8DC11DC14, 34);
    Ok(())
}

#[test]
fn c20dc11dc14dc17() -> PolarsResult<()> {
    check!(C20DC11DC14DC17, 34);
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14() -> PolarsResult<()> {
    check!(C20DC5DC8DC11DC14, 32);
    Ok(())
}

#[test]
fn c20dc8dc11dc14dc17() -> PolarsResult<()> {
    check!(C20DC8DC11DC14DC17, 32);
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14dc17() -> PolarsResult<()> {
    check!(C20DC5DC8DC11DC14DC17, 30);
    Ok(())
}

#[test]
fn c21() -> PolarsResult<()> {
    check!(C21, 42);
    Ok(())
}

#[test]
fn c22() -> PolarsResult<()> {
    check!(C22, 44);
    Ok(())
}

#[test]
fn c22dc13() -> PolarsResult<()> {
    check!(C22DC13, 42);
    Ok(())
}

#[test]
fn c22dc13dc16() -> PolarsResult<()> {
    check!(C22DC13DC16, 40);
    Ok(())
}

#[test]
fn c22dc5dc13dc16() -> PolarsResult<()> {
    check!(C22DC5DC13DC16, 38);
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16() -> PolarsResult<()> {
    check!(C22DC7DC10DC13DC16, 36);
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check!(C22DC7DC10DC13DC16DC19, 34);
    Ok(())
}

#[test]
fn c22dc4dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check!(C22DC4DC7DC10DC13DC16DC19, 32);
    Ok(())
}

#[test]
fn c23() -> PolarsResult<()> {
    check!(C23, 46);
    Ok(())
}

#[test]
fn c24() -> PolarsResult<()> {
    check!(C24, 48);
    Ok(())
}

#[test]
fn c24dc15() -> PolarsResult<()> {
    check!(C24DC15, 46);
    Ok(())
}

#[test]
fn c24dc15dc18() -> PolarsResult<()> {
    check!(C24DC15DC18, 44);
    Ok(())
}

#[test]
fn c24dc12dc15dc18() -> PolarsResult<()> {
    check!(C24DC12DC15DC18, 42);
    Ok(())
}

#[test]
fn c24dc9dc12dc15dc18() -> PolarsResult<()> {
    check!(C24DC9DC12DC15DC18, 40);
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18() -> PolarsResult<()> {
    check!(C24DC6DC9DC12DC15DC18, 38);
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18dc21() -> PolarsResult<()> {
    check!(C24DC6DC9DC12DC15DC18DC21, 36);
    Ok(())
}

#[test]
fn c25() -> PolarsResult<()> {
    check!(C25, 50);
    Ok(())
}

#[test]
fn c26() -> PolarsResult<()> {
    check!(C26, 52);
    Ok(())
}

#[test]
fn c26dc17() -> PolarsResult<()> {
    check!(C26DC17, 50);
    Ok(())
}

#[test]
fn c27() -> PolarsResult<()> {
    check!(C27, 54);
    Ok(())
}

#[test]
fn c28() -> PolarsResult<()> {
    check!(C28, 56);
    Ok(())
}

#[test]
fn c29() -> PolarsResult<()> {
    check!(C29, 58);
    Ok(())
}

#[test]
fn c30() -> PolarsResult<()> {
    check!(C30, 60);
    Ok(())
}

#[test]
fn c30dc21() -> PolarsResult<()> {
    check!(C30DC21, 58);
    Ok(())
}

#[test]
fn c31() -> PolarsResult<()> {
    check!(C31, 62);
    Ok(())
}

#[test]
fn c32() -> PolarsResult<()> {
    check!(C32, 64);
    Ok(())
}

#[test]
fn c33() -> PolarsResult<()> {
    check!(C33, 66);
    Ok(())
}

#[test]
fn c34() -> PolarsResult<()> {
    check!(C34, 68);
    Ok(())
}

#[test]
fn c35() -> PolarsResult<()> {
    check!(C35, 70);
    Ok(())
}

#[test]
fn c36() -> PolarsResult<()> {
    check!(C36, 72);
    Ok(())
}
