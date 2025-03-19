use super::FATTY_ACIDS;
use lipid::prelude::*;
use polars::prelude::*;

#[test]
fn bounds() -> PolarsResult<()> {
    let data_frame = FATTY_ACIDS
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().bounds().alias("")])
        .collect()?;
    let bounds = data_frame[""]
        .as_materialized_series()
        .u8()?
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        bounds,
        [
            Some(3),  // C4U0
            Some(4),  // C5U0
            Some(5),  // C6U0
            Some(6),  // C7U0
            Some(7),  // C8U0
            Some(8),  // C9U0
            Some(9),  // C10U0
            Some(10), // C11U0
            Some(11), // C12U0
            Some(12), // C13U0
            Some(13), // C14U0
            Some(14), // C15U0
            Some(15), // C16U0
            Some(15), // C16U1DC9
            Some(15), // C16U1DT9
            Some(16), // C17U0
            Some(17), // C18U0
            Some(17), // C18U1DC9
            Some(17), // C18U1DT9
            Some(17), // C18U2DC9DC12
            Some(17), // C18U3DC6DC9DC12
            Some(17), // C18U3DC8DT10DC12
            Some(17), // C18U3DC9DC12DC15
            Some(17), // C18U3DC9DT11DT13
            Some(17), // C18U3DT9DT11DC13
            Some(17), // C18U3DT9DT11DT13
            Some(17), // C18U4DC6DC9DC12DC15
            Some(18), // C19U0
            Some(19), // C20U0
            Some(19), // C20U1DC9
            Some(19), // C20U1DC11
            Some(19), // C20U2DC11DC14
            Some(19), // C20U3DC5DC8DC11
            Some(19), // C20U3DC8DC11DC14
            Some(19), // C20U3DC11DC14DC17
            Some(19), // C20U4DC5DC8DC11DC14
            Some(19), // C20U4DC8DC11DC14DC17
            Some(19), // C20U5DC5DC8DC11DC14DC17
            Some(20), // C21U0
            Some(21), // C22U0
            Some(21), // C22U1DC13
            Some(21), // C22U2DC13DC16
            Some(21), // C22U3DC5DC13DC16
            Some(21), // C22U4DC7DC10DC13DC16
            Some(21), // C22U5DC7DC10DC13DC16DC19
            Some(21), // C22U6DC4DC7DC10DC13DC16DC19
            Some(22), // C23U0
            Some(23), // C24U0
            Some(23), // C24U1DC15
            Some(23), // C24U2DC15DC18
            Some(23), // C24U3DC12DC15DC18
            Some(23), // C24U4DC9DC12DC15DC18
            Some(23), // C24U5DC6DC9DC12DC15DC18
            Some(23), // C24U6DC6DC9DC12DC15DC18DC21
            Some(24), // C25U0
            Some(25), // C26U0
            Some(25), // C26U1DC17
            Some(26), // C27U0
            Some(27), // C28U0
            Some(28), // C29U0
            Some(29), // C30U0
            Some(29), // C30U1DC21
            Some(30), // C31U0
            Some(31), // C32U0
            Some(32), // C33U0
            Some(33), // C34U0
            Some(34), // C35U0
            Some(35), // C36U0
            None,     //
        ],
    );
    Ok(())
}

#[test]
fn carbons() -> PolarsResult<()> {
    let data_frame = FATTY_ACIDS
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().carbons().alias("")])
        .collect()?;
    let carbons = data_frame[""]
        .as_materialized_series()
        .u8()?
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        carbons,
        [
            Some(4),  // C4U0
            Some(5),  // C5U0
            Some(6),  // C6U0
            Some(7),  // C7U0
            Some(8),  // C8U0
            Some(9),  // C9U0
            Some(10), // C10U0
            Some(11), // C11U0
            Some(12), // C12U0
            Some(13), // C13U0
            Some(14), // C14U0
            Some(15), // C15U0
            Some(16), // C16U0
            Some(16), // C16U1DC9
            Some(16), // C16U1DT9
            Some(17), // C17U0
            Some(18), // C18U0
            Some(18), // C18U1DC9
            Some(18), // C18U1DT9
            Some(18), // C18U2DC9DC12
            Some(18), // C18U3DC6DC9DC12
            Some(18), // C18U3DC8DT10DC12
            Some(18), // C18U3DC9DC12DC15
            Some(18), // C18U3DC9DT11DT13
            Some(18), // C18U3DT9DT11DC13
            Some(18), // C18U3DT9DT11DT13
            Some(18), // C18U4DC6DC9DC12DC15
            Some(19), // C19U0
            Some(20), // C20U0
            Some(20), // C20U1DC9
            Some(20), // C20U1DC11
            Some(20), // C20U2DC11DC14
            Some(20), // C20U3DC5DC8DC11
            Some(20), // C20U3DC8DC11DC14
            Some(20), // C20U3DC11DC14DC17
            Some(20), // C20U4DC5DC8DC11DC14
            Some(20), // C20U4DC8DC11DC14DC17
            Some(20), // C20U5DC5DC8DC11DC14DC17
            Some(21), // C21U0
            Some(22), // C22U0
            Some(22), // C22U1DC13
            Some(22), // C22U2DC13DC16
            Some(22), // C22U3DC5DC13DC16
            Some(22), // C22U4DC7DC10DC13DC16
            Some(22), // C22U5DC7DC10DC13DC16DC19
            Some(22), // C22U6DC4DC7DC10DC13DC16DC19
            Some(23), // C23U0
            Some(24), // C24U0
            Some(24), // C24U1DC15
            Some(24), // C24U2DC15DC18
            Some(24), // C24U3DC12DC15DC18
            Some(24), // C24U4DC9DC12DC15DC18
            Some(24), // C24U5DC6DC9DC12DC15DC18
            Some(24), // C24U6DC6DC9DC12DC15DC18DC21
            Some(25), // C25U0
            Some(26), // C26U0
            Some(26), // C26U1DC17
            Some(27), // C27U0
            Some(28), // C28U0
            Some(29), // C29U0
            Some(30), // C30U0
            Some(30), // C30U1DC21
            Some(31), // C31U0
            Some(32), // C32U0
            Some(33), // C33U0
            Some(34), // C34U0
            Some(35), // C35U0
            Some(36), // C36U0
            None,     //
        ],
    );
    Ok(())
}

#[test]
fn hydrogens() -> PolarsResult<()> {
    let data_frame = FATTY_ACIDS
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().hydrogens().alias("")])
        .collect()?;
    let hydrogens = data_frame[""]
        .as_materialized_series()
        .u8()?
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        hydrogens,
        [
            Some(8),  // C4U0
            Some(10), // C5U0
            Some(12), // C6U0
            Some(14), // C7U0
            Some(16), // C8U0
            Some(18), // C9U0
            Some(20), // C10U0
            Some(22), // C11U0
            Some(24), // C12U0
            Some(26), // C13U0
            Some(28), // C14U0
            Some(30), // C15U0
            Some(32), // C16U0
            Some(30), // C16U1DC9
            Some(30), // C16U1DT9
            Some(34), // C17U0
            Some(36), // C18U0
            Some(34), // C18U1DC9
            Some(34), // C18U1DT9
            Some(32), // C18U2DC9DC12
            Some(30), // C18U3DC6DC9DC12
            Some(30), // C18U3DC8DT10DC12
            Some(30), // C18U3DC9DC12DC15
            Some(30), // C18U3DC9DT11DT13
            Some(30), // C18U3DT9DT11DC13
            Some(30), // C18U3DT9DT11DT13
            Some(28), // C18U4DC6DC9DC12DC15
            Some(38), // C19U0
            Some(40), // C20U0
            Some(38), // C20U1DC9
            Some(38), // C20U1DC11
            Some(36), // C20U2DC11DC14
            Some(34), // C20U3DC5DC8DC11
            Some(34), // C20U3DC8DC11DC14
            Some(34), // C20U3DC11DC14DC17
            Some(32), // C20U4DC5DC8DC11DC14
            Some(32), // C20U4DC8DC11DC14DC17
            Some(30), // C20U5DC5DC8DC11DC14DC17
            Some(42), // C21U0
            Some(44), // C22U0
            Some(42), // C22U1DC13
            Some(40), // C22U2DC13DC16
            Some(38), // C22U3DC5DC13DC16
            Some(36), // C22U4DC7DC10DC13DC16
            Some(34), // C22U5DC7DC10DC13DC16DC19
            Some(32), // C22U6DC4DC7DC10DC13DC16DC19
            Some(46), // C23U0
            Some(48), // C24U0
            Some(46), // C24U1DC15
            Some(44), // C24U2DC15DC18
            Some(42), // C24U3DC12DC15DC18
            Some(40), // C24U4DC9DC12DC15DC18
            Some(38), // C24U5DC6DC9DC12DC15DC18
            Some(36), // C24U6DC6DC9DC12DC15DC18DC21
            Some(50), // C25U0
            Some(52), // C26U0
            Some(50), // C26U1DC17
            Some(54), // C27U0
            Some(56), // C28U0
            Some(58), // C29U0
            Some(60), // C30U0
            Some(58), // C30U1DC21
            Some(62), // C31U0
            Some(64), // C32U0
            Some(66), // C33U0
            Some(68), // C34U0
            Some(70), // C35U0
            Some(72), // C36U0
            None,     //
        ],
    );
    Ok(())
}

#[test]
fn unsaturation() -> PolarsResult<()> {
    let data_frame = FATTY_ACIDS
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().unsaturation().alias("")])
        .collect()?;
    let unsaturation = data_frame[""]
        .as_materialized_series()
        .u8()?
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        unsaturation,
        [
            Some(0), // C4U0
            Some(0), // C5U0
            Some(0), // C6U0
            Some(0), // C7U0
            Some(0), // C8U0
            Some(0), // C9U0
            Some(0), // C10U0
            Some(0), // C11U0
            Some(0), // C12U0
            Some(0), // C13U0
            Some(0), // C14U0
            Some(0), // C15U0
            Some(0), // C16U0
            Some(1), // C16U1DC9
            Some(1), // C16U1DT9
            Some(0), // C17U0
            Some(0), // C18U0
            Some(1), // C18U1DC9
            Some(1), // C18U1DT9
            Some(2), // C18U2DC9DC12
            Some(3), // C18U3DC6DC9DC12
            Some(3), // C18U3DC8DT10DC12
            Some(3), // C18U3DC9DC12DC15
            Some(3), // C18U3DC9DT11DT13
            Some(3), // C18U3DT9DT11DC13
            Some(3), // C18U3DT9DT11DT13
            Some(4), // C18U4DC6DC9DC12DC15
            Some(0), // C19U0
            Some(0), // C20U0
            Some(1), // C20U1DC9
            Some(1), // C20U1DC11
            Some(2), // C20U2DC11DC14
            Some(3), // C20U3DC5DC8DC11
            Some(3), // C20U3DC8DC11DC14
            Some(3), // C20U3DC11DC14DC17
            Some(4), // C20U4DC5DC8DC11DC14
            Some(4), // C20U4DC8DC11DC14DC17
            Some(5), // C20U5DC5DC8DC11DC14DC17
            Some(0), // C21U0
            Some(0), // C22U0
            Some(1), // C22U1DC13
            Some(2), // C22U2DC13DC16
            Some(3), // C22U3DC5DC13DC16
            Some(4), // C22U4DC7DC10DC13DC16
            Some(5), // C22U5DC7DC10DC13DC16DC19
            Some(6), // C22U6DC4DC7DC10DC13DC16DC19
            Some(0), // C23U0
            Some(0), // C24U0
            Some(1), // C24U1DC15
            Some(2), // C24U2DC15DC18
            Some(3), // C24U3DC12DC15DC18
            Some(4), // C24U4DC9DC12DC15DC18
            Some(5), // C24U5DC6DC9DC12DC15DC18
            Some(6), // C24U6DC6DC9DC12DC15DC18DC21
            Some(0), // C25U0
            Some(0), // C26U0
            Some(1), // C26U1DC17
            Some(0), // C27U0
            Some(0), // C28U0
            Some(0), // C29U0
            Some(0), // C30U0
            Some(1), // C30U1DC21
            Some(0), // C31U0
            Some(0), // C32U0
            Some(0), // C33U0
            Some(0), // C34U0
            Some(0), // C35U0
            Some(0), // C36U0
            None,    //
        ],
    );
    Ok(())
}
