use super::*;

macro_rules! check {
    ($identifier:ident, $expected:expr) => {{
        let data_frame = fatty_acids_with_row_index()?
            .lazy()
            .select([col(FATTY_ACID)
                .fatty_acid()
                .equivalent_chain_length(col("Index").cast(DataType::Float64), false)])
            .collect()?;
        let ecl = data_frame["EquivalentChainLength"]
            .f64()?
            .get(index!($identifier))
            .unwrap();
        assert!((ecl - $expected).abs() < f64::EPSILON);
    }};
}

#[test]
fn c4() -> PolarsResult<()> {
    check!(C4, 4.0);
    Ok(())
}

#[test]
fn c5() -> PolarsResult<()> {
    check!(C5, 5.0);
    Ok(())
}

#[test]
fn c6() -> PolarsResult<()> {
    check!(C6, 6.0);
    Ok(())
}

#[test]
fn c7() -> PolarsResult<()> {
    check!(C7, 7.0);
    Ok(())
}

#[test]
fn c8() -> PolarsResult<()> {
    check!(C8, 8.0);
    Ok(())
}

#[test]
fn c9() -> PolarsResult<()> {
    check!(C9, 9.0);
    Ok(())
}

#[test]
fn c10() -> PolarsResult<()> {
    check!(C10, 10.0);
    Ok(())
}

#[test]
fn c11() -> PolarsResult<()> {
    check!(C11, 11.0);
    Ok(())
}

#[test]
fn c12() -> PolarsResult<()> {
    check!(C12, 12.0);
    Ok(())
}

#[test]
fn c13() -> PolarsResult<()> {
    check!(C13, 13.0);
    Ok(())
}

#[test]
fn c14() -> PolarsResult<()> {
    check!(C14, 14.0);
    Ok(())
}

#[test]
fn c15() -> PolarsResult<()> {
    check!(C15, 15.0);
    Ok(())
}

#[test]
fn c16() -> PolarsResult<()> {
    check!(C16, 16.0);
    Ok(())
}

#[test]
fn c16dc9() -> PolarsResult<()> {
    check!(C16DC9, 16.0 + (13.0 - 12.0) / (15.0 - 12.0));
    Ok(())
}

#[test]
fn c16dt9() -> PolarsResult<()> {
    check!(C16DT9, 16.0 + (14.0 - 12.0) / (15.0 - 12.0));
    Ok(())
}

#[test]
fn c17() -> PolarsResult<()> {
    check!(C17, 17.0);
    Ok(())
}

#[test]
fn c18() -> PolarsResult<()> {
    check!(C18, 18.0);
    Ok(())
}

#[test]
fn c18dc9() -> PolarsResult<()> {
    check!(C18DC9, 18.0 + (17.0 - 16.0) / (27.0 - 16.0));
    Ok(())
}

#[test]
fn c18dt9() -> PolarsResult<()> {
    check!(C18DT9, 18.0 + (18.0 - 16.0) / (27.0 - 16.0));
    Ok(())
}

#[test]
fn c18dc9dc12() -> PolarsResult<()> {
    check!(C18DC9DC12, 18.0 + (19.0 - 16.0) / (27.0 - 16.0));
    Ok(())
}

#[test]
fn c18dc6dc9dc12() -> PolarsResult<()> {
    check!(C18DC6DC9DC12, 18.0 + (20.0 - 16.0) / (27.0 - 16.0));
    Ok(())
}

#[test]
fn c18dc8dt10dc12() -> PolarsResult<()> {
    check!(C18DC8DT10DC12, 18.0 + (21.0 - 16.0) / (27.0 - 16.0));
    Ok(())
}

#[test]
fn c18dc9dc12dc15() -> PolarsResult<()> {
    check!(C18DC9DC12DC15, 18.0 + (22.0 - 16.0) / (27.0 - 16.0));
    Ok(())
}

#[test]
fn c18dc9dt11dt13() -> PolarsResult<()> {
    check!(C18DC9DT11DT13, 18.0 + (23.0 - 16.0) / (27.0 - 16.0));
    Ok(())
}

#[test]
fn c18dt9dt11dc13() -> PolarsResult<()> {
    check!(C18DT9DT11DC13, 18.0 + (24.0 - 16.0) / (27.0 - 16.0));
    Ok(())
}

#[test]
fn c18dt9dt11dt13() -> PolarsResult<()> {
    check!(C18DT9DT11DT13, 18.0 + (25.0 - 16.0) / (27.0 - 16.0));
    Ok(())
}

#[test]
fn c18dc6dc9dc12dc15() -> PolarsResult<()> {
    check!(C18DC6DC9DC12DC15, 18.0 + (26.0 - 16.0) / (27.0 - 16.0));
    Ok(())
}

#[test]
fn c19() -> PolarsResult<()> {
    check!(C19, 19.0);
    Ok(())
}

#[test]
fn c20() -> PolarsResult<()> {
    check!(C20, 20.0);
    Ok(())
}

#[test]
fn c20dc9() -> PolarsResult<()> {
    check!(C20DC9, 20.0 + (29.0 - 28.0) / (38.0 - 28.0));
    Ok(())
}

#[test]
fn c20dc11() -> PolarsResult<()> {
    check!(C20DC11, 20.0 + (30.0 - 28.0) / (38.0 - 28.0));
    Ok(())
}

#[test]
fn c20dc11dc14() -> PolarsResult<()> {
    check!(C20DC11DC14, 20.0 + (31.0 - 28.0) / (38.0 - 28.0));
    Ok(())
}

#[test]
fn c20dc5dc8dc11() -> PolarsResult<()> {
    check!(C20DC5DC8DC11, 20.0 + (32.0 - 28.0) / (38.0 - 28.0));
    Ok(())
}

#[test]
fn c20dc8dc11dc14() -> PolarsResult<()> {
    check!(C20DC8DC11DC14, 20.0 + (33.0 - 28.0) / (38.0 - 28.0));
    Ok(())
}

#[test]
fn c20dc11dc14dc17() -> PolarsResult<()> {
    check!(C20DC11DC14DC17, 20.0 + (34.0 - 28.0) / (38.0 - 28.0));
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14() -> PolarsResult<()> {
    check!(C20DC5DC8DC11DC14, 20.0 + (35.0 - 28.0) / (38.0 - 28.0));
    Ok(())
}

#[test]
fn c20dc8dc11dc14dc17() -> PolarsResult<()> {
    check!(C20DC8DC11DC14DC17, 20.0 + (36.0 - 28.0) / (38.0 - 28.0));
    Ok(())
}

#[test]
fn c20dc5dc8dc11dc14dc17() -> PolarsResult<()> {
    check!(C20DC5DC8DC11DC14DC17, 20.0 + (37.0 - 28.0) / (38.0 - 28.0));
    Ok(())
}

#[test]
fn c21() -> PolarsResult<()> {
    check!(C21, 21.0);
    Ok(())
}

#[test]
fn c22() -> PolarsResult<()> {
    check!(C22, 22.0);
    Ok(())
}

#[test]
fn c22dc13() -> PolarsResult<()> {
    check!(C22DC13, 22.0 + (40.0 - 39.0) / (46.0 - 39.0));
    Ok(())
}

#[test]
fn c22dc13dc16() -> PolarsResult<()> {
    check!(C22DC13DC16, 22.0 + (41.0 - 39.0) / (46.0 - 39.0));
    Ok(())
}

#[test]
fn c22dc5dc13dc16() -> PolarsResult<()> {
    check!(C22DC5DC13DC16, 22.0 + (42.0 - 39.0) / (46.0 - 39.0));
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16() -> PolarsResult<()> {
    check!(C22DC7DC10DC13DC16, 22.0 + (43.0 - 39.0) / (46.0 - 39.0));
    Ok(())
}

#[test]
fn c22dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check!(C22DC7DC10DC13DC16DC19, 22.0 + (44.0 - 39.0) / (46.0 - 39.0));
    Ok(())
}

#[test]
fn c22dc4dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check!(
        C22DC4DC7DC10DC13DC16DC19,
        22.0 + (45.0 - 39.0) / (46.0 - 39.0)
    );
    Ok(())
}

#[test]
fn c23() -> PolarsResult<()> {
    check!(C23, 23.0);
    Ok(())
}

#[test]
fn c24() -> PolarsResult<()> {
    check!(C24, 24.0);
    Ok(())
}

#[test]
fn c24dc15() -> PolarsResult<()> {
    check!(C24DC15, 24.0 + (48.0 - 47.0) / (54.0 - 47.0));
    Ok(())
}

#[test]
fn c24dc15dc18() -> PolarsResult<()> {
    check!(C24DC15DC18, 24.0 + (49.0 - 47.0) / (54.0 - 47.0));
    Ok(())
}

#[test]
fn c24dc12dc15dc18() -> PolarsResult<()> {
    check!(C24DC12DC15DC18, 24.0 + (50.0 - 47.0) / (54.0 - 47.0));
    Ok(())
}

#[test]
fn c24dc9dc12dc15dc18() -> PolarsResult<()> {
    check!(C24DC9DC12DC15DC18, 24.0 + (51.0 - 47.0) / (54.0 - 47.0));
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18() -> PolarsResult<()> {
    check!(C24DC6DC9DC12DC15DC18, 24.0 + (52.0 - 47.0) / (54.0 - 47.0));
    Ok(())
}

#[test]
fn c24dc6dc9dc12dc15dc18dc21() -> PolarsResult<()> {
    check!(
        C24DC6DC9DC12DC15DC18DC21,
        24.0 + (53.0 - 47.0) / (54.0 - 47.0)
    );
    Ok(())
}

#[test]
fn c25() -> PolarsResult<()> {
    check!(C25, 25.0);
    Ok(())
}

#[test]
fn c26() -> PolarsResult<()> {
    check!(C26, 26.0);
    Ok(())
}

#[test]
fn c26dc17() -> PolarsResult<()> {
    check!(C26DC17, 26.0 + (56.0 - 55.0) / (57.0 - 55.0));
    Ok(())
}

#[test]
fn c27() -> PolarsResult<()> {
    check!(C27, 27.0);
    Ok(())
}

#[test]
fn c28() -> PolarsResult<()> {
    check!(C28, 28.0);
    Ok(())
}

#[test]
fn c29() -> PolarsResult<()> {
    check!(C29, 29.0);
    Ok(())
}

#[test]
fn c30() -> PolarsResult<()> {
    check!(C30, 30.0);
    Ok(())
}

#[test]
fn c30dc21() -> PolarsResult<()> {
    check!(C30DC21, 30.0 + (61.0 - 60.0) / (62.0 - 60.0));
    Ok(())
}

#[test]
fn c31() -> PolarsResult<()> {
    check!(C31, 31.0);
    Ok(())
}

#[test]
fn c32() -> PolarsResult<()> {
    check!(C32, 32.0);
    Ok(())
}

#[test]
fn c33() -> PolarsResult<()> {
    check!(C33, 33.0);
    Ok(())
}

#[test]
fn c34() -> PolarsResult<()> {
    check!(C34, 34.0);
    Ok(())
}

#[test]
fn c35() -> PolarsResult<()> {
    check!(C35, 35.0);
    Ok(())
}

#[test]
fn c36() -> PolarsResult<()> {
    check!(C36, 36.0);
    Ok(())
}
