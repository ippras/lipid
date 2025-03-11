use super::SOURCE;
use lipid::prelude::*;
use polars::prelude::*;

#[test]
fn equivalent_carbon_number() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .equivalent_carbon_number()
            .alias("EquivalentCarbonNumber")])
        .collect()
        .unwrap();
    let equivalent_carbon_number = data_frame["EquivalentCarbonNumber"]
        .as_materialized_series()
        .u8()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        equivalent_carbon_number,
        vec![
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
            None      //
        ],
    );
}
