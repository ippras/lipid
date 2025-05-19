use lipid::prelude::*;
use polars::prelude::*;

macro_rules! check {
    ($identifier:ident) => {{
        let fatty_acid = FattyAcidChunked::try_from($identifier)?;
        let is_monounsaturated = fatty_acid.is_monounsaturated();
        assert!(is_monounsaturated == Some(expected::$identifier));
    }};
}

#[test]
fn c4() -> PolarsResult<()> {
    check!(C4);
    Ok(())
}

#[test]
fn c5() -> PolarsResult<()> {
    check!(C5);
    Ok(())
}

#[test]
fn c6() -> PolarsResult<()> {
    check!(C6);
    Ok(())
}

#[test]
fn c7() -> PolarsResult<()> {
    check!(C7);
    Ok(())
}

#[test]
fn c8() -> PolarsResult<()> {
    check!(C8);
    Ok(())
}

#[test]
fn c9() -> PolarsResult<()> {
    check!(C9);
    Ok(())
}

#[test]
fn c10() -> PolarsResult<()> {
    check!(C10);
    Ok(())
}

#[test]
fn c11() -> PolarsResult<()> {
    check!(C11);
    Ok(())
}

#[test]
fn c12() -> PolarsResult<()> {
    check!(C12);
    Ok(())
}

#[test]
fn c13() -> PolarsResult<()> {
    check!(C13);
    Ok(())
}

#[test]
fn c14() -> PolarsResult<()> {
    check!(C14);
    Ok(())
}

#[test]
fn c15() -> PolarsResult<()> {
    check!(C15);
    Ok(())
}

#[test]
fn c16() -> PolarsResult<()> {
    check!(C16);
    Ok(())
}

#[test]
fn c16dc9() -> PolarsResult<()> {
    check!(C16DC9);
    Ok(())
}

#[test]
fn c16dt9() -> PolarsResult<()> {
    check!(C16DT9);
    Ok(())
}

#[test]
fn c17() -> PolarsResult<()> {
    check!(C17);
    Ok(())
}

#[test]
fn c18() -> PolarsResult<()> {
    check!(C18);
    Ok(())
}

#[test]
fn c18dc9() -> PolarsResult<()> {
    check!(C18DC9);
    Ok(())
}

#[test]
fn c18dt9() -> PolarsResult<()> {
    check!(C18DT9);
    Ok(())
}

#[test]
fn c18dc9dc12() -> PolarsResult<()> {
    check!(C18DC9DC12);
    Ok(())
}

#[test]
fn c18dc6dc9dc12() -> PolarsResult<()> {
    check!(C18DC6DC9DC12);
    Ok(())
}

#[test]
fn c18dc8dt10dc12() -> PolarsResult<()> {
    check!(C18DC8DT10DC12);
    Ok(())
}

#[test]
fn c18dc9dc12dc15() -> PolarsResult<()> {
    check!(C18DC9DC12DC15);
    Ok(())
}

#[test]
fn c18dc9dt11dt13() -> PolarsResult<()> {
    check!(C18DC9DT11DT13);
    Ok(())
}

#[test]
fn c18dt9dt11dc13() -> PolarsResult<()> {
    check!(C18DT9DT11DC13);
    Ok(())
}

#[test]
fn c18dt9dt11dt13() -> PolarsResult<()> {
    check!(C18DT9DT11DT13);
    Ok(())
}

#[test]
fn c18dc6dc9dc12dc15() -> PolarsResult<()> {
    check!(C18DC6DC9DC12DC15);
    Ok(())
}

#[test]
fn c19() -> PolarsResult<()> {
    check!(C19);
    Ok(())
}

#[test]
fn c20() -> PolarsResult<()> {
    check!(C20);
    Ok(())
}

#[test]
fn c20dc9() -> PolarsResult<()> {
    check!(C20DC9);
    Ok(())
}

#[test]
fn c20dc11() -> PolarsResult<()> {
    check!(C20DC11);
    Ok(())
}

#[test]
fn c20dc11dc14() -> PolarsResult<()> {
    check!(C20DC11DC14);
    Ok(())
}

#[test]
fn c20dc5dc8dc11() -> PolarsResult<()> {
    check!(C20DC5DC8DC11);
    Ok(())
}

#[test]
fn c20dc8dc11dc14() -> PolarsResult<()> {
    check!(C20DC8DC11DC14);
    Ok(())
}

#[test]
fn c20dc11dc14dc17() -> PolarsResult<()> {
    check!(C20DC11DC14DC17);
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14() -> PolarsResult<()> {
    check!(C20DC5DC8DC11DC14);
    Ok(())
}

#[test]
fn c20dc8dc11dc14dc17() -> PolarsResult<()> {
    check!(C20DC8DC11DC14DC17);
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14dc17() -> PolarsResult<()> {
    check!(C20DC5DC8DC11DC14DC17);
    Ok(())
}

#[test]
fn c21() -> PolarsResult<()> {
    check!(C21);
    Ok(())
}

#[test]
fn c22() -> PolarsResult<()> {
    check!(C22);
    Ok(())
}

#[test]
fn c22dc13() -> PolarsResult<()> {
    check!(C22DC13);
    Ok(())
}

#[test]
fn c22dc13dc16() -> PolarsResult<()> {
    check!(C22DC13DC16);
    Ok(())
}

#[test]
fn c22dc5dc13dc16() -> PolarsResult<()> {
    check!(C22DC5DC13DC16);
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16() -> PolarsResult<()> {
    check!(C22DC7DC10DC13DC16);
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check!(C22DC7DC10DC13DC16DC19);
    Ok(())
}

#[test]
fn c22dc4dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check!(C22DC4DC7DC10DC13DC16DC19);
    Ok(())
}

#[test]
fn c23() -> PolarsResult<()> {
    check!(C23);
    Ok(())
}

#[test]
fn c24() -> PolarsResult<()> {
    check!(C24);
    Ok(())
}

#[test]
fn c24dc15() -> PolarsResult<()> {
    check!(C24DC15);
    Ok(())
}

#[test]
fn c24dc15dc18() -> PolarsResult<()> {
    check!(C24DC15DC18);
    Ok(())
}

#[test]
fn c24dc12dc15dc18() -> PolarsResult<()> {
    check!(C24DC12DC15DC18);
    Ok(())
}

#[test]
fn c24dc9dc12dc15dc18() -> PolarsResult<()> {
    check!(C24DC9DC12DC15DC18);
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18() -> PolarsResult<()> {
    check!(C24DC6DC9DC12DC15DC18);
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18dc21() -> PolarsResult<()> {
    check!(C24DC6DC9DC12DC15DC18DC21);
    Ok(())
}

#[test]
fn c25() -> PolarsResult<()> {
    check!(C25);
    Ok(())
}

#[test]
fn c26() -> PolarsResult<()> {
    check!(C26);
    Ok(())
}

#[test]
fn c26dc17() -> PolarsResult<()> {
    check!(C26DC17);
    Ok(())
}

#[test]
fn c27() -> PolarsResult<()> {
    check!(C27);
    Ok(())
}

#[test]
fn c28() -> PolarsResult<()> {
    check!(C28);
    Ok(())
}

#[test]
fn c29() -> PolarsResult<()> {
    check!(C29);
    Ok(())
}

#[test]
fn c30() -> PolarsResult<()> {
    check!(C30);
    Ok(())
}

#[test]
fn c30dc21() -> PolarsResult<()> {
    check!(C30DC21);
    Ok(())
}

#[test]
fn c31() -> PolarsResult<()> {
    check!(C31);
    Ok(())
}

#[test]
fn c32() -> PolarsResult<()> {
    check!(C32);
    Ok(())
}

#[test]
fn c33() -> PolarsResult<()> {
    check!(C33);
    Ok(())
}

#[test]
fn c34() -> PolarsResult<()> {
    check!(C34);
    Ok(())
}

#[test]
fn c35() -> PolarsResult<()> {
    check!(C35);
    Ok(())
}

#[test]
fn c36() -> PolarsResult<()> {
    check!(C36);
    Ok(())
}

mod expected {
    pub(super) const C4: bool = false;
    pub(super) const C5: bool = false;
    pub(super) const C6: bool = false;
    pub(super) const C7: bool = false;
    pub(super) const C8: bool = false;
    pub(super) const C9: bool = false;
    pub(super) const C10: bool = false;
    pub(super) const C11: bool = false;
    pub(super) const C12: bool = false;
    pub(super) const C13: bool = false;
    pub(super) const C14: bool = false;
    pub(super) const C15: bool = false;
    pub(super) const C16: bool = false;
    pub(super) const C16DC9: bool = true;
    pub(super) const C16DT9: bool = true;
    pub(super) const C17: bool = false;
    pub(super) const C18: bool = false;
    pub(super) const C18DC9: bool = true;
    pub(super) const C18DT9: bool = true;
    pub(super) const C18DC9DC12: bool = false;
    pub(super) const C18DC6DC9DC12: bool = false;
    pub(super) const C18DC8DT10DC12: bool = false;
    pub(super) const C18DC9DC12DC15: bool = false;
    pub(super) const C18DC9DT11DT13: bool = false;
    pub(super) const C18DT9DT11DC13: bool = false;
    pub(super) const C18DT9DT11DT13: bool = false;
    pub(super) const C18DC6DC9DC12DC15: bool = false;
    pub(super) const C19: bool = false;
    pub(super) const C20: bool = false;
    pub(super) const C20DC9: bool = true;
    pub(super) const C20DC11: bool = true;
    pub(super) const C20DC11DC14: bool = false;
    pub(super) const C20DC5DC8DC11: bool = false;
    pub(super) const C20DC8DC11DC14: bool = false;
    pub(super) const C20DC11DC14DC17: bool = false;
    pub(super) const C20DC5DC8DC11DC14: bool = false;
    pub(super) const C20DC8DC11DC14DC17: bool = false;
    pub(super) const C20DC5DC8DC11DC14DC17: bool = false;
    pub(super) const C21: bool = false;
    pub(super) const C22: bool = false;
    pub(super) const C22DC13: bool = true;
    pub(super) const C22DC13DC16: bool = false;
    pub(super) const C22DC5DC13DC16: bool = false;
    pub(super) const C22DC7DC10DC13DC16: bool = false;
    pub(super) const C22DC7DC10DC13DC16DC19: bool = false;
    pub(super) const C22DC4DC7DC10DC13DC16DC19: bool = false;
    pub(super) const C23: bool = false;
    pub(super) const C24: bool = false;
    pub(super) const C24DC15: bool = true;
    pub(super) const C24DC15DC18: bool = false;
    pub(super) const C24DC12DC15DC18: bool = false;
    pub(super) const C24DC9DC12DC15DC18: bool = false;
    pub(super) const C24DC6DC9DC12DC15DC18: bool = false;
    pub(super) const C24DC6DC9DC12DC15DC18DC21: bool = false;
    pub(super) const C25: bool = false;
    pub(super) const C26: bool = false;
    pub(super) const C26DC17: bool = true;
    pub(super) const C27: bool = false;
    pub(super) const C28: bool = false;
    pub(super) const C29: bool = false;
    pub(super) const C30: bool = false;
    pub(super) const C30DC21: bool = true;
    pub(super) const C31: bool = false;
    pub(super) const C32: bool = false;
    pub(super) const C33: bool = false;
    pub(super) const C34: bool = false;
    pub(super) const C35: bool = false;
    pub(super) const C36: bool = false;
}
