use super::FATTY_ACIDS;
use lipid::prelude::*;
use polars::prelude::*;

#[test]
fn equivalent_carbon_number() -> PolarsResult<()> {
    let data_frame = FATTY_ACIDS
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .equivalent_carbon_number()
            .alias("EquivalentCarbonNumber")])
        .collect()?;
    let equivalent_carbon_number = data_frame["EquivalentCarbonNumber"]
        .as_materialized_series()
        .u8()?
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        equivalent_carbon_number,
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
            Some(14), // C16U1DC9
            Some(14), // C16U1DT9
            Some(17), // C17U0
            Some(18), // C18U0
            Some(16), // C18U1DC9
            Some(16), // C18U1DT9
            Some(14), // C18U2DC9DC12
            Some(12), // C18U3DC6DC9DC12
            Some(12), // C18U3DC8DT10DC12
            Some(12), // C18U3DC9DC12DC15
            Some(12), // C18U3DC9DT11DT13
            Some(12), // C18U3DT9DT11DC13
            Some(12), // C18U3DT9DT11DT13
            Some(10), // C18U4DC6DC9DC12DC15
            Some(19), // C19U0
            Some(20), // C20U0
            Some(18), // C20U1DC9
            Some(18), // C20U1DC11
            Some(16), // C20U2DC11DC14
            Some(14), // C20U3DC5DC8DC11
            Some(14), // C20U3DC8DC11DC14
            Some(14), // C20U3DC11DC14DC17
            Some(12), // C20U4DC5DC8DC11DC14
            Some(12), // C20U4DC8DC11DC14DC17
            Some(10), // C20U5DC5DC8DC11DC14DC17
            Some(21), // C21U0
            Some(22), // C22U0
            Some(20), // C22U1DC13
            Some(18), // C22U2DC13DC16
            Some(16), // C22U3DC5DC13DC16
            Some(14), // C22U4DC7DC10DC13DC16
            Some(12), // C22U5DC7DC10DC13DC16DC19
            Some(10), // C22U6DC4DC7DC10DC13DC16DC19
            Some(23), // C23U0
            Some(24), // C24U0
            Some(22), // C24U1DC15
            Some(20), // C24U2DC15DC18
            Some(18), // C24U3DC12DC15DC18
            Some(16), // C24U4DC9DC12DC15DC18
            Some(14), // C24U5DC6DC9DC12DC15DC18
            Some(12), // C24U6DC6DC9DC12DC15DC18DC21
            Some(25), // C25U0
            Some(26), // C26U0
            Some(24), // C26U1DC17
            Some(27), // C27U0
            Some(28), // C28U0
            Some(29), // C29U0
            Some(30), // C30U0
            Some(28), // C30U1DC21
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
fn equivalent_chain_length() -> PolarsResult<()> {
    let data_frame = FATTY_ACIDS
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .equivalent_chain_length(col("Index").cast(DataType::Float64), false)
            .alias("EquivalentChainLength")])
        .collect()?;
    let equivalent_chain_length = data_frame["EquivalentChainLength"]
        .as_materialized_series()
        .f64()?
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        equivalent_chain_length,
        [
            Some(4.0),                                  // 00 C4U0
            Some(5.0),                                  // 01 C5U0
            Some(6.0),                                  // 02 C6U0
            Some(7.0),                                  // 03 C7U0
            Some(8.0),                                  // 04 C8U0
            Some(9.0),                                  // 05 C9U0
            Some(10.0),                                 // 06 C10U0
            Some(11.0),                                 // 07 C11U0
            Some(12.0),                                 // 08 C12U0
            Some(13.0),                                 // 09 C13U0
            Some(14.0),                                 // 10 C14U0
            Some(15.0),                                 // 11 C15U0
            Some(16.0),                                 // 12 C16U0
            Some(16.0 + (13.0 - 12.0) / (15.0 - 12.0)), // 13 C16U1DC9
            Some(16.0 + (14.0 - 12.0) / (15.0 - 12.0)), // 14 C16U1DT9
            Some(17.0),                                 // 15 C17U0
            Some(18.0),                                 // 16 C18U0
            Some(18.0 + (17.0 - 16.0) / (27.0 - 16.0)), // 17 C18U1DC9
            Some(18.0 + (18.0 - 16.0) / (27.0 - 16.0)), // 18 C18U1DT9
            Some(18.0 + (19.0 - 16.0) / (27.0 - 16.0)), // 19 C18U2DC9DC12
            Some(18.0 + (20.0 - 16.0) / (27.0 - 16.0)), // 20 C18U3DC6DC9DC12
            Some(18.0 + (21.0 - 16.0) / (27.0 - 16.0)), // 21 C18U3DC8DT10DC12
            Some(18.0 + (22.0 - 16.0) / (27.0 - 16.0)), // 22 C18U3DC9DC12DC15
            Some(18.0 + (23.0 - 16.0) / (27.0 - 16.0)), // 23 C18U3DC9DT11DT13
            Some(18.0 + (24.0 - 16.0) / (27.0 - 16.0)), // 24 C18U3DT9DT11DC13
            Some(18.0 + (25.0 - 16.0) / (27.0 - 16.0)), // 25 C18U3DT9DT11DT13
            Some(18.0 + (26.0 - 16.0) / (27.0 - 16.0)), // 26 C18U4DC6DC9DC12DC15
            Some(19.0),                                 // 27 C19U0
            Some(20.0),                                 // 28 C20U0
            Some(20.0 + (29.0 - 28.0) / (38.0 - 28.0)), // 29 C20U1DC9
            Some(20.0 + (30.0 - 28.0) / (38.0 - 28.0)), // 30 C20U1DC11
            Some(20.0 + (31.0 - 28.0) / (38.0 - 28.0)), // 31 C20U2DC11DC14
            Some(20.0 + (32.0 - 28.0) / (38.0 - 28.0)), // 32 C20U3DC5DC8DC11
            Some(20.0 + (33.0 - 28.0) / (38.0 - 28.0)), // 33 C20U3DC8DC11DC14
            Some(20.0 + (34.0 - 28.0) / (38.0 - 28.0)), // 34 C20U3DC11DC14DC17
            Some(20.0 + (35.0 - 28.0) / (38.0 - 28.0)), // 35 C20U4DC5DC8DC11DC14
            Some(20.0 + (36.0 - 28.0) / (38.0 - 28.0)), // 36 C20U4DC8DC11DC14DC17
            Some(20.0 + (37.0 - 28.0) / (38.0 - 28.0)), // 37 C20U5DC5DC8DC11DC14DC17
            Some(21.0),                                 // 38 C21U0
            Some(22.0),                                 // 39 C22U0
            Some(22.0 + (40.0 - 39.0) / (46.0 - 39.0)), // 40 C22U1DC13
            Some(22.0 + (41.0 - 39.0) / (46.0 - 39.0)), // 41 C22U2DC13DC16
            Some(22.0 + (42.0 - 39.0) / (46.0 - 39.0)), // 42 C22U3DC5DC13DC16
            Some(22.0 + (43.0 - 39.0) / (46.0 - 39.0)), // 43 C22U4DC7DC10DC13DC16
            Some(22.0 + (44.0 - 39.0) / (46.0 - 39.0)), // 44 C22U5DC7DC10DC13DC16DC19
            Some(22.0 + (45.0 - 39.0) / (46.0 - 39.0)), // 45 C22U6DC4DC7DC10DC13DC16DC19
            Some(23.0),                                 // 46 C23U0
            Some(24.0),                                 // 47 C24U0
            Some(24.0 + (48.0 - 47.0) / (54.0 - 47.0)), // 48 C24U1DC15
            Some(24.0 + (49.0 - 47.0) / (54.0 - 47.0)), // 49 C24U2DC15DC18
            Some(24.0 + (50.0 - 47.0) / (54.0 - 47.0)), // 50 C24U3DC12DC15DC18
            Some(24.0 + (51.0 - 47.0) / (54.0 - 47.0)), // 51 C24U4DC9DC12DC15DC18
            Some(24.0 + (52.0 - 47.0) / (54.0 - 47.0)), // 52 C24U5DC6DC9DC12DC15DC18
            Some(24.0 + (53.0 - 47.0) / (54.0 - 47.0)), // 53 C24U6DC6DC9DC12DC15DC18DC21
            Some(25.0),                                 // 54 C25U0
            Some(26.0),                                 // 55 C26U0
            Some(26.0 + (56.0 - 55.0) / (57.0 - 55.0)), // 56 C26U1DC17
            Some(27.0),                                 // 57 C27U0
            Some(28.0),                                 // 58 C28U0
            Some(29.0),                                 // 59 C29U0
            Some(30.0),                                 // 60 C30U0
            Some(30.0 + (61.0 - 60.0) / (62.0 - 60.0)), // 61 C30U1DC21
            Some(31.0),                                 // 62 C31U0
            Some(32.0),                                 // 63 C32U0
            Some(33.0),                                 // 64 C33U0
            Some(34.0),                                 // 65 C34U0
            Some(35.0),                                 // 66 C35U0
            Some(36.0),                                 // 67 C36U0
            None,                                       // 68
        ],
    );
    Ok(())
}

#[test]
fn fractional_chain_length() -> PolarsResult<()> {
    let data_frame = FATTY_ACIDS
        .clone()
        .lazy()
        .select([
            col("FattyAcid")
                .fatty_acid()
                .fractional_chain_length(col("Index").cast(DataType::Float64), false)
                .alias("FractionalChainLength"),
            col("FattyAcid")
                .fatty_acid()
                .fractional_chain_length(col("Index").cast(DataType::Float64), true)
                .alias("LogarithmicFractionalChainLength"),
        ])
        .collect()?;
    let fractional_chain_length = data_frame["FractionalChainLength"]
        .as_materialized_series()
        .f64()?
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        fractional_chain_length,
        [
            Some(0.0),                           // 00 C4U0
            Some(0.0),                           // 01 C5U0
            Some(0.0),                           // 02 C6U0
            Some(0.0),                           // 03 C7U0
            Some(0.0),                           // 04 C8U0
            Some(0.0),                           // 05 C9U0
            Some(0.0),                           // 06 C10U0
            Some(0.0),                           // 07 C11U0
            Some(0.0),                           // 08 C12U0
            Some(0.0),                           // 09 C13U0
            Some(0.0),                           // 10 C14U0
            Some(0.0),                           // 11 C15U0
            Some(0.0),                           // 12 C16U0
            Some((13.0 - 12.0) / (15.0 - 12.0)), // 13 C16U1DC9
            Some((14.0 - 12.0) / (15.0 - 12.0)), // 14 C16U1DT9
            Some(0.0),                           // 15 C17U0
            Some(0.0),                           // 16 C18U0
            Some((17.0 - 16.0) / (27.0 - 16.0)), // 17 C18U1DC9
            Some((18.0 - 16.0) / (27.0 - 16.0)), // 18 C18U1DT9
            Some((19.0 - 16.0) / (27.0 - 16.0)), // 19 C18U2DC9DC12
            Some((20.0 - 16.0) / (27.0 - 16.0)), // 20 C18U3DC6DC9DC12
            Some((21.0 - 16.0) / (27.0 - 16.0)), // 21 C18U3DC8DT10DC12
            Some((22.0 - 16.0) / (27.0 - 16.0)), // 22 C18U3DC9DC12DC15
            Some((23.0 - 16.0) / (27.0 - 16.0)), // 23 C18U3DC9DT11DT13
            Some((24.0 - 16.0) / (27.0 - 16.0)), // 24 C18U3DT9DT11DC13
            Some((25.0 - 16.0) / (27.0 - 16.0)), // 25 C18U3DT9DT11DT13
            Some((26.0 - 16.0) / (27.0 - 16.0)), // 26 C18U4DC6DC9DC12DC15
            Some(0.0),                           // 27 C19U0
            Some(0.0),                           // 28 C20U0
            Some((29.0 - 28.0) / (38.0 - 28.0)), // 29 C20U1DC9
            Some((30.0 - 28.0) / (38.0 - 28.0)), // 30 C20U1DC11
            Some((31.0 - 28.0) / (38.0 - 28.0)), // 31 C20U2DC11DC14
            Some((32.0 - 28.0) / (38.0 - 28.0)), // 32 C20U3DC5DC8DC11
            Some((33.0 - 28.0) / (38.0 - 28.0)), // 33 C20U3DC8DC11DC14
            Some((34.0 - 28.0) / (38.0 - 28.0)), // 34 C20U3DC11DC14DC17
            Some((35.0 - 28.0) / (38.0 - 28.0)), // 35 C20U4DC5DC8DC11DC14
            Some((36.0 - 28.0) / (38.0 - 28.0)), // 36 C20U4DC8DC11DC14DC17
            Some((37.0 - 28.0) / (38.0 - 28.0)), // 37 C20U5DC5DC8DC11DC14DC17
            Some(0.0),                           // 38 C21U0
            Some(0.0),                           // 39 C22U0
            Some((40.0 - 39.0) / (46.0 - 39.0)), // 40 C22U1DC13
            Some((41.0 - 39.0) / (46.0 - 39.0)), // 41 C22U2DC13DC16
            Some((42.0 - 39.0) / (46.0 - 39.0)), // 42 C22U3DC5DC13DC16
            Some((43.0 - 39.0) / (46.0 - 39.0)), // 43 C22U4DC7DC10DC13DC16
            Some((44.0 - 39.0) / (46.0 - 39.0)), // 44 C22U5DC7DC10DC13DC16DC19
            Some((45.0 - 39.0) / (46.0 - 39.0)), // 45 C22U6DC4DC7DC10DC13DC16DC19
            Some(0.0),                           // 46 C23U0
            Some(0.0),                           // 47 C24U0
            Some((48.0 - 47.0) / (54.0 - 47.0)), // 48 C24U1DC15
            Some((49.0 - 47.0) / (54.0 - 47.0)), // 49 C24U2DC15DC18
            Some((50.0 - 47.0) / (54.0 - 47.0)), // 50 C24U3DC12DC15DC18
            Some((51.0 - 47.0) / (54.0 - 47.0)), // 51 C24U4DC9DC12DC15DC18
            Some((52.0 - 47.0) / (54.0 - 47.0)), // 52 C24U5DC6DC9DC12DC15DC18
            Some((53.0 - 47.0) / (54.0 - 47.0)), // 53 C24U6DC6DC9DC12DC15DC18DC21
            Some(0.0),                           // 54 C25U0
            Some(0.0),                           // 55 C26U0
            Some((56.0 - 55.0) / (57.0 - 55.0)), // 56 C26U1DC17
            Some(0.0),                           // 57 C27U0
            Some(0.0),                           // 58 C28U0
            Some(0.0),                           // 59 C29U0
            Some(0.0),                           // 60 C30U0
            Some((61.0 - 60.0) / (62.0 - 60.0)), // 61 C30U1DC21
            Some(0.0),                           // 62 C31U0
            Some(0.0),                           // 63 C32U0
            Some(0.0),                           // 64 C33U0
            Some(0.0),                           // 65 C34U0
            Some(0.0),                           // 66 C35U0
            Some(0.0),                           // 67 C36U0
            None,                                // 68
        ],
    );
    Ok(())
}
