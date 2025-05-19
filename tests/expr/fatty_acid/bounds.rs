use super::*;

macro_rules! check {
    ($identifier:ident, $expected:expr) => {{
        let data_frame = fatty_acid($identifier)?
            .lazy()
            .select([col(FATTY_ACID).fatty_acid().bounds()])
            .collect()?;
        let got = data_frame[FATTY_ACID].u8()?.get(0);
        assert_eq!(got, Some($expected));
    }};
}

#[test]
fn c4() -> PolarsResult<()> {
    check!(C4, 3);
    Ok(())
}

#[test]
fn c5() -> PolarsResult<()> {
    check!(C5, 4);
    Ok(())
}

#[test]
fn c6() -> PolarsResult<()> {
    check!(C6, 5);
    Ok(())
}

#[test]
fn c7() -> PolarsResult<()> {
    check!(C7, 6);
    Ok(())
}

#[test]
fn c8() -> PolarsResult<()> {
    check!(C8, 7);
    Ok(())
}

#[test]
fn c9() -> PolarsResult<()> {
    check!(C9, 8);
    Ok(())
}

#[test]
fn c10() -> PolarsResult<()> {
    check!(C10, 9);
    Ok(())
}

#[test]
fn c11() -> PolarsResult<()> {
    check!(C11, 10);
    Ok(())
}

#[test]
fn c12() -> PolarsResult<()> {
    check!(C12, 11);
    Ok(())
}

#[test]
fn c13() -> PolarsResult<()> {
    check!(C13, 12);
    Ok(())
}

#[test]
fn c14() -> PolarsResult<()> {
    check!(C14, 13);
    Ok(())
}

#[test]
fn c15() -> PolarsResult<()> {
    check!(C15, 14);
    Ok(())
}

#[test]
fn c16() -> PolarsResult<()> {
    check!(C16, 15);
    Ok(())
}

#[test]
fn c16dc9() -> PolarsResult<()> {
    check!(C16DC9, 15);
    Ok(())
}

#[test]
fn c16dt9() -> PolarsResult<()> {
    check!(C16DT9, 15);
    Ok(())
}

#[test]
fn c17() -> PolarsResult<()> {
    check!(C17, 16);
    Ok(())
}

#[test]
fn c18() -> PolarsResult<()> {
    check!(C18, 17);
    Ok(())
}

#[test]
fn c18dc9() -> PolarsResult<()> {
    check!(C18DC9, 17);
    Ok(())
}

#[test]
fn c18dt9() -> PolarsResult<()> {
    check!(C18DT9, 17);
    Ok(())
}

#[test]
fn c18dc9dc12() -> PolarsResult<()> {
    check!(C18DC9DC12, 17);
    Ok(())
}

#[test]
fn c18dc6dc9dc12() -> PolarsResult<()> {
    check!(C18DC6DC9DC12, 17);
    Ok(())
}

#[test]
fn c18dc8dt10dc12() -> PolarsResult<()> {
    check!(C18DC8DT10DC12, 17);
    Ok(())
}

#[test]
fn c18dc9dc12dc15() -> PolarsResult<()> {
    check!(C18DC9DC12DC15, 17);
    Ok(())
}

#[test]
fn c18dc9dt11dt13() -> PolarsResult<()> {
    check!(C18DC9DT11DT13, 17);
    Ok(())
}

#[test]
fn c18dt9dt11dc13() -> PolarsResult<()> {
    check!(C18DT9DT11DC13, 17);
    Ok(())
}

#[test]
fn c18dt9dt11dt13() -> PolarsResult<()> {
    check!(C18DT9DT11DT13, 17);
    Ok(())
}

#[test]
fn c18dc6dc9dc12dc15() -> PolarsResult<()> {
    check!(C18DC6DC9DC12DC15, 17);
    Ok(())
}

#[test]
fn c19() -> PolarsResult<()> {
    check!(C19, 18);
    Ok(())
}

#[test]
fn c20() -> PolarsResult<()> {
    check!(C20, 19);
    Ok(())
}

#[test]
fn c20dc9() -> PolarsResult<()> {
    check!(C20DC9, 19);
    Ok(())
}

#[test]
fn c20dc11() -> PolarsResult<()> {
    check!(C20DC11, 19);
    Ok(())
}

#[test]
fn c20dc11dc14() -> PolarsResult<()> {
    check!(C20DC11DC14, 19);
    Ok(())
}

#[test]
fn c20dc5dc8dc11() -> PolarsResult<()> {
    check!(C20DC5DC8DC11, 19);
    Ok(())
}

#[test]
fn c20dc8dc11dc14() -> PolarsResult<()> {
    check!(C20DC8DC11DC14, 19);
    Ok(())
}

#[test]
fn c20dc11dc14dc17() -> PolarsResult<()> {
    check!(C20DC11DC14DC17, 19);
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14() -> PolarsResult<()> {
    check!(C20DC5DC8DC11DC14, 19);
    Ok(())
}

#[test]
fn c20dc8dc11dc14dc17() -> PolarsResult<()> {
    check!(C20DC8DC11DC14DC17, 19);
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14dc17() -> PolarsResult<()> {
    check!(C20DC5DC8DC11DC14DC17, 19);
    Ok(())
}

#[test]
fn c21() -> PolarsResult<()> {
    check!(C21, 20);
    Ok(())
}

#[test]
fn c22() -> PolarsResult<()> {
    check!(C22, 21);
    Ok(())
}

#[test]
fn c22dc13() -> PolarsResult<()> {
    check!(C22DC13, 21);
    Ok(())
}

#[test]
fn c22dc13dc16() -> PolarsResult<()> {
    check!(C22DC13DC16, 21);
    Ok(())
}

#[test]
fn c22dc5dc13dc16() -> PolarsResult<()> {
    check!(C22DC5DC13DC16, 21);
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16() -> PolarsResult<()> {
    check!(C22DC7DC10DC13DC16, 21);
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check!(C22DC7DC10DC13DC16DC19, 21);
    Ok(())
}

#[test]
fn c22dc4dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check!(C22DC4DC7DC10DC13DC16DC19, 21);
    Ok(())
}

#[test]
fn c23() -> PolarsResult<()> {
    check!(C23, 22);
    Ok(())
}

#[test]
fn c24() -> PolarsResult<()> {
    check!(C24, 23);
    Ok(())
}

#[test]
fn c24dc15() -> PolarsResult<()> {
    check!(C24DC15, 23);
    Ok(())
}

#[test]
fn c24dc15dc18() -> PolarsResult<()> {
    check!(C24DC15DC18, 23);
    Ok(())
}

#[test]
fn c24dc12dc15dc18() -> PolarsResult<()> {
    check!(C24DC12DC15DC18, 23);
    Ok(())
}

#[test]
fn c24dc9dc12dc15dc18() -> PolarsResult<()> {
    check!(C24DC9DC12DC15DC18, 23);
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18() -> PolarsResult<()> {
    check!(C24DC6DC9DC12DC15DC18, 23);
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18dc21() -> PolarsResult<()> {
    check!(C24DC6DC9DC12DC15DC18DC21, 23);
    Ok(())
}

#[test]
fn c25() -> PolarsResult<()> {
    check!(C25, 24);
    Ok(())
}

#[test]
fn c26() -> PolarsResult<()> {
    check!(C26, 25);
    Ok(())
}

#[test]
fn c26dc17() -> PolarsResult<()> {
    check!(C26DC17, 25);
    Ok(())
}

#[test]
fn c27() -> PolarsResult<()> {
    check!(C27, 26);
    Ok(())
}

#[test]
fn c28() -> PolarsResult<()> {
    check!(C28, 27);
    Ok(())
}

#[test]
fn c29() -> PolarsResult<()> {
    check!(C29, 28);
    Ok(())
}

#[test]
fn c30() -> PolarsResult<()> {
    check!(C30, 29);
    Ok(())
}

#[test]
fn c30dc21() -> PolarsResult<()> {
    check!(C30DC21, 29);
    Ok(())
}

#[test]
fn c31() -> PolarsResult<()> {
    check!(C31, 30);
    Ok(())
}

#[test]
fn c32() -> PolarsResult<()> {
    check!(C32, 31);
    Ok(())
}

#[test]
fn c33() -> PolarsResult<()> {
    check!(C33, 32);
    Ok(())
}

#[test]
fn c34() -> PolarsResult<()> {
    check!(C34, 33);
    Ok(())
}

#[test]
fn c35() -> PolarsResult<()> {
    check!(C35, 34);
    Ok(())
}

#[test]
fn c36() -> PolarsResult<()> {
    check!(C36, 35);
    Ok(())
}
