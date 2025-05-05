use super::fatty_acid;
use lipid::prelude::*;
use polars::prelude::*;

fn check_display(fatty_acid: FattyAcidChunked, expected: [&str; 2]) {
    let display = fatty_acid.display(Default::default());
    assert_eq!(display.to_string(), expected[0]);
    assert_eq!(format!("{display:#}"), expected[1]);
}

#[test]
fn c4() -> PolarsResult<()> {
    check_display(fatty_acid(&C4)?, expected::C4);
    Ok(())
}

#[test]
fn c5() -> PolarsResult<()> {
    check_display(fatty_acid(&C5)?, expected::C5);
    Ok(())
}

#[test]
fn c6() -> PolarsResult<()> {
    check_display(fatty_acid(&C6)?, expected::C6);
    Ok(())
}

#[test]
fn c7() -> PolarsResult<()> {
    check_display(fatty_acid(&C7)?, expected::C7);
    Ok(())
}

#[test]
fn c8() -> PolarsResult<()> {
    check_display(fatty_acid(&C8)?, expected::C8);
    Ok(())
}

#[test]
fn c9() -> PolarsResult<()> {
    check_display(fatty_acid(&C9)?, expected::C9);
    Ok(())
}

#[test]
fn c10() -> PolarsResult<()> {
    check_display(fatty_acid(&C10)?, expected::C10);
    Ok(())
}

#[test]
fn c11() -> PolarsResult<()> {
    check_display(fatty_acid(&C11)?, expected::C11);
    Ok(())
}

#[test]
fn c12() -> PolarsResult<()> {
    check_display(fatty_acid(&C12)?, expected::C12);
    Ok(())
}

#[test]
fn c13() -> PolarsResult<()> {
    check_display(fatty_acid(&C13)?, expected::C13);
    Ok(())
}

#[test]
fn c14() -> PolarsResult<()> {
    check_display(fatty_acid(&C14)?, expected::C14);
    Ok(())
}

#[test]
fn c15() -> PolarsResult<()> {
    check_display(fatty_acid(&C15)?, expected::C15);
    Ok(())
}

#[test]
fn c16() -> PolarsResult<()> {
    check_display(fatty_acid(&C16)?, expected::C16);
    Ok(())
}

#[test]
fn c16dc9() -> PolarsResult<()> {
    check_display(fatty_acid(&C16DC9)?, expected::C16DC9);
    Ok(())
}

#[test]
fn c16dt9() -> PolarsResult<()> {
    check_display(fatty_acid(&C16DT9)?, expected::C16DT9);
    Ok(())
}

#[test]
fn c17() -> PolarsResult<()> {
    check_display(fatty_acid(&C17)?, expected::C17);
    Ok(())
}

#[test]
fn c18() -> PolarsResult<()> {
    check_display(fatty_acid(&C18)?, expected::C18);
    Ok(())
}

#[test]
fn c18dc9() -> PolarsResult<()> {
    check_display(fatty_acid(&C18DC9)?, expected::C18DC9);
    Ok(())
}

#[test]
fn c18dt9() -> PolarsResult<()> {
    check_display(fatty_acid(&C18DT9)?, expected::C18DT9);
    Ok(())
}

#[test]
fn c18dc9dc12() -> PolarsResult<()> {
    check_display(fatty_acid(&C18DC9DC12)?, expected::C18DC9DC12);
    Ok(())
}

#[test]
fn c18dc6dc9dc12() -> PolarsResult<()> {
    check_display(fatty_acid(&C18DC6DC9DC12)?, expected::C18DC6DC9DC12);
    Ok(())
}

#[test]
fn c18dc8dt10dc12() -> PolarsResult<()> {
    check_display(fatty_acid(&C18DC8DT10DC12)?, expected::C18DC8DT10DC12);
    Ok(())
}

#[test]
fn c18dc9dc12dc15() -> PolarsResult<()> {
    check_display(fatty_acid(&C18DC9DC12DC15)?, expected::C18DC9DC12DC15);
    Ok(())
}

#[test]
fn c18dc9dt11dt13() -> PolarsResult<()> {
    check_display(fatty_acid(&C18DC9DT11DT13)?, expected::C18DC9DT11DT13);
    Ok(())
}

#[test]
fn c18dt9dt11dc13() -> PolarsResult<()> {
    check_display(fatty_acid(&C18DT9DT11DC13)?, expected::C18DT9DT11DC13);
    Ok(())
}

#[test]
fn c18dt9dt11dt13() -> PolarsResult<()> {
    check_display(fatty_acid(&C18DT9DT11DT13)?, expected::C18DT9DT11DT13);
    Ok(())
}

#[test]
fn c18dc6dc9dc12dc15() -> PolarsResult<()> {
    check_display(fatty_acid(&C18DC6DC9DC12DC15)?, expected::C18DC6DC9DC12DC15);
    Ok(())
}

#[test]
fn c19() -> PolarsResult<()> {
    check_display(fatty_acid(&C19)?, expected::C19);
    Ok(())
}

#[test]
fn c20() -> PolarsResult<()> {
    check_display(fatty_acid(&C20)?, expected::C20);
    Ok(())
}

#[test]
fn c20dc9() -> PolarsResult<()> {
    check_display(fatty_acid(&C20DC9)?, expected::C20DC9);
    Ok(())
}

#[test]
fn c20dc11() -> PolarsResult<()> {
    check_display(fatty_acid(&C20DC11)?, expected::C20DC11);
    Ok(())
}

#[test]
fn c20dc11dc14() -> PolarsResult<()> {
    check_display(fatty_acid(&C20DC11DC14)?, expected::C20DC11DC14);
    Ok(())
}

#[test]
fn c20dc5dc8dc11() -> PolarsResult<()> {
    check_display(fatty_acid(&C20DC5DC8DC11)?, expected::C20DC5DC8DC11);
    Ok(())
}

#[test]
fn c20dc8dc11dc14() -> PolarsResult<()> {
    check_display(fatty_acid(&C20DC8DC11DC14)?, expected::C20DC8DC11DC14);
    Ok(())
}

#[test]
fn c20dc11dc14dc17() -> PolarsResult<()> {
    check_display(fatty_acid(&C20DC11DC14DC17)?, expected::C20DC11DC14DC17);
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14() -> PolarsResult<()> {
    check_display(fatty_acid(&C20DC5DC8DC11DC14)?, expected::C20DC5DC8DC11DC14);
    Ok(())
}

#[test]
fn c20dc8dc11dc14dc17() -> PolarsResult<()> {
    check_display(
        fatty_acid(&C20DC8DC11DC14DC17)?,
        expected::C20DC8DC11DC14DC17,
    );
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14dc17() -> PolarsResult<()> {
    check_display(
        fatty_acid(&C20DC5DC8DC11DC14DC17)?,
        expected::C20DC5DC8DC11DC14DC17,
    );
    Ok(())
}

#[test]
fn c21() -> PolarsResult<()> {
    check_display(fatty_acid(&C21)?, expected::C21);
    Ok(())
}

#[test]
fn c22() -> PolarsResult<()> {
    check_display(fatty_acid(&C22)?, expected::C22);
    Ok(())
}

#[test]
fn c22dc13() -> PolarsResult<()> {
    check_display(fatty_acid(&C22DC13)?, expected::C22DC13);
    Ok(())
}

#[test]
fn c22dc13dc16() -> PolarsResult<()> {
    check_display(fatty_acid(&C22DC13DC16)?, expected::C22DC13DC16);
    Ok(())
}

#[test]
fn c22dc5dc13dc16() -> PolarsResult<()> {
    check_display(fatty_acid(&C22DC5DC13DC16)?, expected::C22DC5DC13DC16);
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16() -> PolarsResult<()> {
    check_display(
        fatty_acid(&C22DC7DC10DC13DC16)?,
        expected::C22DC7DC10DC13DC16,
    );
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check_display(
        fatty_acid(&C22DC7DC10DC13DC16DC19)?,
        expected::C22DC7DC10DC13DC16DC19,
    );
    Ok(())
}

#[test]
fn c22dc4dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check_display(
        fatty_acid(&C22DC4DC7DC10DC13DC16DC19)?,
        expected::C22DC4DC7DC10DC13DC16DC19,
    );
    Ok(())
}

#[test]
fn c23() -> PolarsResult<()> {
    check_display(fatty_acid(&C23)?, expected::C23);
    Ok(())
}

#[test]
fn c24() -> PolarsResult<()> {
    check_display(fatty_acid(&C24)?, expected::C24);
    Ok(())
}

#[test]
fn c24dc15() -> PolarsResult<()> {
    check_display(fatty_acid(&C24DC15)?, expected::C24DC15);
    Ok(())
}

#[test]
fn c24dc15dc18() -> PolarsResult<()> {
    check_display(fatty_acid(&C24DC15DC18)?, expected::C24DC15DC18);
    Ok(())
}

#[test]
fn c24dc12dc15dc18() -> PolarsResult<()> {
    check_display(fatty_acid(&C24DC12DC15DC18)?, expected::C24DC12DC15DC18);
    Ok(())
}

#[test]
fn c24dc9dc12dc15dc18() -> PolarsResult<()> {
    check_display(
        fatty_acid(&C24DC9DC12DC15DC18)?,
        expected::C24DC9DC12DC15DC18,
    );
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18() -> PolarsResult<()> {
    check_display(
        fatty_acid(&C24DC6DC9DC12DC15DC18)?,
        expected::C24DC6DC9DC12DC15DC18,
    );
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18dc21() -> PolarsResult<()> {
    check_display(
        fatty_acid(&C24DC6DC9DC12DC15DC18DC21)?,
        expected::C24DC6DC9DC12DC15DC18DC21,
    );
    Ok(())
}

#[test]
fn c25() -> PolarsResult<()> {
    check_display(fatty_acid(&C25)?, expected::C25);
    Ok(())
}

#[test]
fn c26() -> PolarsResult<()> {
    check_display(fatty_acid(&C26)?, expected::C26);
    Ok(())
}

#[test]
fn c26dc17() -> PolarsResult<()> {
    check_display(fatty_acid(&C26DC17)?, expected::C26DC17);
    Ok(())
}

#[test]
fn c27() -> PolarsResult<()> {
    check_display(fatty_acid(&C27)?, expected::C27);
    Ok(())
}

#[test]
fn c28() -> PolarsResult<()> {
    check_display(fatty_acid(&C28)?, expected::C28);
    Ok(())
}

#[test]
fn c29() -> PolarsResult<()> {
    check_display(fatty_acid(&C29)?, expected::C29);
    Ok(())
}

#[test]
fn c30() -> PolarsResult<()> {
    check_display(fatty_acid(&C30)?, expected::C30);
    Ok(())
}

#[test]
fn c30dc21() -> PolarsResult<()> {
    check_display(fatty_acid(&C30DC21)?, expected::C30DC21);
    Ok(())
}

#[test]
fn c31() -> PolarsResult<()> {
    check_display(fatty_acid(&C31)?, expected::C31);
    Ok(())
}

#[test]
fn c32() -> PolarsResult<()> {
    check_display(fatty_acid(&C32)?, expected::C32);
    Ok(())
}

#[test]
fn c33() -> PolarsResult<()> {
    check_display(fatty_acid(&C33)?, expected::C33);
    Ok(())
}

#[test]
fn c34() -> PolarsResult<()> {
    check_display(fatty_acid(&C34)?, expected::C34);
    Ok(())
}

#[test]
fn c35() -> PolarsResult<()> {
    check_display(fatty_acid(&C35)?, expected::C35);
    Ok(())
}

#[test]
fn c36() -> PolarsResult<()> {
    check_display(fatty_acid(&C36)?, expected::C36);
    Ok(())
}

#[rustfmt::skip]
mod expected {
    pub(super) const C4: [&str; 2] = ["4:0", "4:0"];
    pub(super) const C5: [&str; 2] = ["5:0", "5:0"];
    pub(super) const C6: [&str; 2] = ["6:0", "6:0"];
    pub(super) const C7: [&str; 2] = ["7:0", "7:0"];
    pub(super) const C8: [&str; 2] = ["8:0", "8:0"];
    pub(super) const C9: [&str; 2] = ["9:0", "9:0"];
    pub(super) const C10: [&str; 2] = ["10:0", "10:0"];
    pub(super) const C11: [&str; 2] = ["11:0", "11:0"];
    pub(super) const C12: [&str; 2] = ["12:0", "12:0"];
    pub(super) const C13: [&str; 2] = ["13:0", "13:0"];
    pub(super) const C14: [&str; 2] = ["14:0", "14:0"];
    pub(super) const C15: [&str; 2] = ["15:0", "15:0"];
    pub(super) const C16: [&str; 2] = ["16:0", "16:0"];
    pub(super) const C16DC9: [&str; 2] = ["16:1", "16:1Δ9"];
    pub(super) const C16DT9: [&str; 2] = ["16:1", "16:1Δ9t"];
    pub(super) const C17: [&str; 2] = ["17:0", "17:0"];
    pub(super) const C18: [&str; 2] = ["18:0", "18:0"];
    pub(super) const C18DC9: [&str; 2] = ["18:1", "18:1Δ9"];
    pub(super) const C18DT9: [&str; 2] = ["18:1", "18:1Δ9t"];
    pub(super) const C18DC9DC12: [&str; 2] = ["18:2", "18:2Δ9,12"];
    pub(super) const C18DC6DC9DC12: [&str; 2] = ["18:3", "18:3Δ6,9,12"];
    pub(super) const C18DC8DT10DC12: [&str; 2] = ["18:3", "18:3Δ8,10t,12"];
    pub(super) const C18DC9DC12DC15: [&str; 2] = ["18:3", "18:3Δ9,12,15"];
    pub(super) const C18DC9DT11DT13: [&str; 2] = ["18:3", "18:3Δ9,11t,13t"];
    pub(super) const C18DT9DT11DC13: [&str; 2] = ["18:3", "18:3Δ9t,11t,13"];
    pub(super) const C18DT9DT11DT13: [&str; 2] = ["18:3", "18:3Δ9t,11t,13t"];
    pub(super) const C18DC6DC9DC12DC15: [&str; 2] = ["18:4", "18:4Δ6,9,12,15"];
    pub(super) const C19: [&str; 2] = ["19:0", "19:0"];
    pub(super) const C20: [&str; 2] = ["20:0", "20:0"];
    pub(super) const C20DC9: [&str; 2] = ["20:1", "20:1Δ9"];
    pub(super) const C20DC11: [&str; 2] = ["20:1", "20:1Δ11"];
    pub(super) const C20DC11DC14: [&str; 2] = ["20:2", "20:2Δ11,14"];
    pub(super) const C20DC5DC8DC11: [&str; 2] = ["20:3", "20:3Δ5,8,11"];
    pub(super) const C20DC8DC11DC14: [&str; 2] = ["20:3", "20:3Δ8,11,14"];
    pub(super) const C20DC11DC14DC17: [&str; 2] = ["20:3", "20:3Δ11,14,17"];
    pub(super) const C20DC5DC8DC11DC14: [&str; 2] = ["20:4", "20:4Δ5,8,11,14"];
    pub(super) const C20DC8DC11DC14DC17: [&str; 2] = ["20:4", "20:4Δ8,11,14,17"];
    pub(super) const C20DC5DC8DC11DC14DC17: [&str; 2] = ["20:5", "20:5Δ5,8,11,14,17"];
    pub(super) const C21: [&str; 2] = ["21:0", "21:0"];
    pub(super) const C22: [&str; 2] = ["22:0", "22:0"];
    pub(super) const C22DC13: [&str; 2] = ["22:1", "22:1Δ13"];
    pub(super) const C22DC13DC16: [&str; 2] = ["22:2", "22:2Δ13,16"];
    pub(super) const C22DC5DC13DC16: [&str; 2] = ["22:3", "22:3Δ5,13,16"];
    pub(super) const C22DC7DC10DC13DC16: [&str; 2] = ["22:4", "22:4Δ7,10,13,16"];
    pub(super) const C22DC7DC10DC13DC16DC19: [&str; 2] = ["22:5", "22:5Δ7,10,13,16,19"];
    pub(super) const C22DC4DC7DC10DC13DC16DC19: [&str; 2] = ["22:6", "22:6Δ4,7,10,13,16,19"];
    pub(super) const C23: [&str; 2] = ["23:0", "23:0"];
    pub(super) const C24: [&str; 2] = ["24:0", "24:0"];
    pub(super) const C24DC15: [&str; 2] = ["24:1", "24:1Δ15"];
    pub(super) const C24DC15DC18: [&str; 2] = ["24:2", "24:2Δ15,18"];
    pub(super) const C24DC12DC15DC18: [&str; 2] = ["24:3", "24:3Δ12,15,18"];
    pub(super) const C24DC9DC12DC15DC18: [&str; 2] = ["24:4", "24:4Δ9,12,15,18"];
    pub(super) const C24DC6DC9DC12DC15DC18: [&str; 2] = ["24:5", "24:5Δ6,9,12,15,18"];
    pub(super) const C24DC6DC9DC12DC15DC18DC21: [&str; 2] = ["24:6", "24:6Δ6,9,12,15,18,21"];
    pub(super) const C25: [&str; 2] = ["25:0", "25:0"];
    pub(super) const C26: [&str; 2] = ["26:0", "26:0"];
    pub(super) const C26DC17: [&str; 2] = ["26:1", "26:1Δ17"];
    pub(super) const C27: [&str; 2] = ["27:0", "27:0"];
    pub(super) const C28: [&str; 2] = ["28:0", "28:0"];
    pub(super) const C29: [&str; 2] = ["29:0", "29:0"];
    pub(super) const C30: [&str; 2] = ["30:0", "30:0"];
    pub(super) const C30DC21: [&str; 2] = ["30:1", "30:1Δ21"];
    pub(super) const C31: [&str; 2] = ["31:0", "31:0"];
    pub(super) const C32: [&str; 2] = ["32:0", "32:0"];
    pub(super) const C33: [&str; 2] = ["33:0", "33:0"];
    pub(super) const C34: [&str; 2] = ["34:0", "34:0"];
    pub(super) const C35: [&str; 2] = ["35:0", "35:0"];
    pub(super) const C36: [&str; 2] = ["36:0", "36:0"];
}
