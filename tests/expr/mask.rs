use super::*;

#[test]
fn is_saturated() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .is_saturated()
            .alias("IsSaturated")])
        .collect()
        .unwrap();
    let is_saturated = data_frame["IsSaturated"]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        is_saturated,
        vec![
            Some(true),  // C4U0
            Some(true),  // C5U0
            Some(true),  // C6U0
            Some(true),  // C7U0
            Some(true),  // C8U0
            Some(true),  // C9U0
            Some(true),  // C10U0
            Some(true),  // C11U0
            Some(true),  // C12U0
            Some(true),  // C13U0
            Some(true),  // C14U0
            Some(true),  // C15U0
            Some(true),  // C16U0
            Some(false), // C16U1DC9
            Some(false), // C16U1DT9
            Some(true),  // C17U0
            Some(true),  // C18U0
            Some(false), // C18U1DC9
            Some(false), // C18U1DT9
            Some(false), // C18U2DC9DC12
            Some(false), // C18U3DC6DC9DC12
            Some(false), // C18U3DC8DT10DC12
            Some(false), // C18U3DC9DC12DC15
            Some(false), // C18U3DC9DT11DT13
            Some(false), // C18U3DT9DT11DC13
            Some(false), // C18U3DT9DT11DT13
            Some(false), // C18U4DC6DC9DC12DC15
            Some(true),  // C19U0
            Some(true),  // C20U0
            Some(false), // C20U1DC9
            Some(false), // C20U1DC11
            Some(false), // C20U2DC11DC14
            Some(false), // C20U3DC5DC8DC11
            Some(false), // C20U3DC8DC11DC14
            Some(false), // C20U3DC11DC14DC17
            Some(false), // C20U4DC5DC8DC11DC14
            Some(false), // C20U4DC8DC11DC14DC17
            Some(false), // C20U5DC5DC8DC11DC14DC17
            Some(true),  // C21U0
            Some(true),  // C22U0
            Some(false), // C22U1DC13
            Some(false), // C22U2DC13DC16
            Some(false), // C22U3DC5DC13DC16
            Some(false), // C22U4DC7DC10DC13DC16
            Some(false), // C22U5DC7DC10DC13DC16DC19
            Some(false), // C22U6DC4DC7DC10DC13DC16DC19
            Some(true),  // C23U0
            Some(true),  // C24U0
            Some(false), // C24U1DC15
            Some(false), // C24U2DC15DC18
            Some(false), // C24U3DC12DC15DC18
            Some(false), // C24U4DC9DC12DC15DC18
            Some(false), // C24U5DC6DC9DC12DC15DC18
            Some(false), // C24U6DC6DC9DC12DC15DC18DC21
            Some(true),  // C25U0
            Some(true),  // C26U0
            Some(false), // C26U1DC17
            Some(true),  // C27U0
            Some(true),  // C28U0
            Some(true),  // C29U0
            Some(true),  // C30U0
            Some(false), // C30U1DC21
            Some(true),  // C31U0
            Some(true),  // C32U0
            Some(true),  // C33U0
            Some(true),  // C34U0
            Some(true),  // C35U0
            Some(true),  // C36U0
            None,        //
        ],
    );
}

#[test]
fn is_unsaturated() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .is_unsaturated()
            .alias("IsUnsaturated")])
        .collect()
        .unwrap();
    let is_unsaturated = data_frame["IsUnsaturated"]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        is_unsaturated,
        vec![
            Some(false), // C4U0
            Some(false), // C5U0
            Some(false), // C6U0
            Some(false), // C7U0
            Some(false), // C8U0
            Some(false), // C9U0
            Some(false), // C10U0
            Some(false), // C11U0
            Some(false), // C12U0
            Some(false), // C13U0
            Some(false), // C14U0
            Some(false), // C15U0
            Some(false), // C16U0
            Some(true),  // C16U1DC9
            Some(true),  // C16U1DT9
            Some(false), // C17U0
            Some(false), // C18U0
            Some(true),  // C18U1DC9
            Some(true),  // C18U1DT9
            Some(true),  // C18U2DC9DC12
            Some(true),  // C18U3DC6DC9DC12
            Some(true),  // C18U3DC8DT10DC12
            Some(true),  // C18U3DC9DC12DC15
            Some(true),  // C18U3DC9DT11DT13
            Some(true),  // C18U3DT9DT11DC13
            Some(true),  // C18U3DT9DT11DT13
            Some(true),  // C18U4DC6DC9DC12DC15
            Some(false), // C19U0
            Some(false), // C20U0
            Some(true),  // C20U1DC9
            Some(true),  // C20U1DC11
            Some(true),  // C20U2DC11DC14
            Some(true),  // C20U3DC5DC8DC11
            Some(true),  // C20U3DC8DC11DC14
            Some(true),  // C20U3DC11DC14DC17
            Some(true),  // C20U4DC5DC8DC11DC14
            Some(true),  // C20U4DC8DC11DC14DC17
            Some(true),  // C20U5DC5DC8DC11DC14DC17
            Some(false), // C21U0
            Some(false), // C22U0
            Some(true),  // C22U1DC13
            Some(true),  // C22U2DC13DC16
            Some(true),  // C22U3DC5DC13DC16
            Some(true),  // C22U4DC7DC10DC13DC16
            Some(true),  // C22U5DC7DC10DC13DC16DC19
            Some(true),  // C22U6DC4DC7DC10DC13DC16DC19
            Some(false), // C23U0
            Some(false), // C24U0
            Some(true),  // C24U1DC15
            Some(true),  // C24U2DC15DC18
            Some(true),  // C24U3DC12DC15DC18
            Some(true),  // C24U4DC9DC12DC15DC18
            Some(true),  // C24U5DC6DC9DC12DC15DC18
            Some(true),  // C24U6DC6DC9DC12DC15DC18DC21
            Some(false), // C25U0
            Some(false), // C26U0
            Some(true),  // C26U1DC17
            Some(false), // C27U0
            Some(false), // C28U0
            Some(false), // C29U0
            Some(false), // C30U0
            Some(true),  // C30U1DC21
            Some(false), // C31U0
            Some(false), // C32U0
            Some(false), // C33U0
            Some(false), // C34U0
            Some(false), // C35U0
            Some(false), // C36U0
            None,        //
        ],
    );
}

#[test]
fn is_monounsaturated() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .is_monounsaturated()
            .alias("IsMonounsaturated")])
        .collect()
        .unwrap();
    let is_monounsaturated = data_frame["IsMonounsaturated"]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        is_monounsaturated,
        vec![
            Some(false), // C4U0
            Some(false), // C5U0
            Some(false), // C6U0
            Some(false), // C7U0
            Some(false), // C8U0
            Some(false), // C9U0
            Some(false), // C10U0
            Some(false), // C11U0
            Some(false), // C12U0
            Some(false), // C13U0
            Some(false), // C14U0
            Some(false), // C15U0
            Some(false), // C16U0
            Some(true),  // C16U1DC9
            Some(true),  // C16U1DT9
            Some(false), // C17U0
            Some(false), // C18U0
            Some(true),  // C18U1DC9
            Some(true),  // C18U1DT9
            Some(false), // C18U2DC9DC12
            Some(false), // C18U3DC6DC9DC12
            Some(false), // C18U3DC8DT10DC12
            Some(false), // C18U3DC9DC12DC15
            Some(false), // C18U3DC9DT11DT13
            Some(false), // C18U3DT9DT11DC13
            Some(false), // C18U3DT9DT11DT13
            Some(false), // C18U4DC6DC9DC12DC15
            Some(false), // C19U0
            Some(false), // C20U0
            Some(true),  // C20U1DC9
            Some(true),  // C20U1DC11
            Some(false), // C20U2DC11DC14
            Some(false), // C20U3DC5DC8DC11
            Some(false), // C20U3DC8DC11DC14
            Some(false), // C20U3DC11DC14DC17
            Some(false), // C20U4DC5DC8DC11DC14
            Some(false), // C20U4DC8DC11DC14DC17
            Some(false), // C20U5DC5DC8DC11DC14DC17
            Some(false), // C21U0
            Some(false), // C22U0
            Some(true),  // C22U1DC13
            Some(false), // C22U2DC13DC16
            Some(false), // C22U3DC5DC13DC16
            Some(false), // C22U4DC7DC10DC13DC16
            Some(false), // C22U5DC7DC10DC13DC16DC19
            Some(false), // C22U6DC4DC7DC10DC13DC16DC19
            Some(false), // C23U0
            Some(false), // C24U0
            Some(true),  // C24U1DC15
            Some(false), // C24U2DC15DC18
            Some(false), // C24U3DC12DC15DC18
            Some(false), // C24U4DC9DC12DC15DC18
            Some(false), // C24U5DC6DC9DC12DC15DC18
            Some(false), // C24U6DC6DC9DC12DC15DC18DC21
            Some(false), // C25U0
            Some(false), // C26U0
            Some(true),  // C26U1DC17
            Some(false), // C27U0
            Some(false), // C28U0
            Some(false), // C29U0
            Some(false), // C30U0
            Some(true),  // C30U1DC21
            Some(false), // C31U0
            Some(false), // C32U0
            Some(false), // C33U0
            Some(false), // C34U0
            Some(false), // C35U0
            Some(false), // C36U0
            None,        //
        ],
    );
}

#[test]
fn is_polyunsaturated() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .is_polyunsaturated()
            .alias("IsPolyunsaturated")])
        .collect()
        .unwrap();
    let is_polyunsaturated = data_frame["IsPolyunsaturated"]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        is_polyunsaturated,
        vec![
            Some(false), // C4U0
            Some(false), // C5U0
            Some(false), // C6U0
            Some(false), // C7U0
            Some(false), // C8U0
            Some(false), // C9U0
            Some(false), // C10U0
            Some(false), // C11U0
            Some(false), // C12U0
            Some(false), // C13U0
            Some(false), // C14U0
            Some(false), // C15U0
            Some(false), // C16U0
            Some(false), // C16U1DC9
            Some(false), // C16U1DT9
            Some(false), // C17U0
            Some(false), // C18U0
            Some(false), // C18U1DC9
            Some(false), // C18U1DT9
            Some(true),  // C18U2DC9DC12
            Some(true),  // C18U3DC6DC9DC12
            Some(true),  // C18U3DC8DT10DC12
            Some(true),  // C18U3DC9DC12DC15
            Some(true),  // C18U3DC9DT11DT13
            Some(true),  // C18U3DT9DT11DC13
            Some(true),  // C18U3DT9DT11DT13
            Some(true),  // C18U4DC6DC9DC12DC15
            Some(false), // C19U0
            Some(false), // C20U0
            Some(false), // C20U1DC9
            Some(false), // C20U1DC11
            Some(true),  // C20U2DC11DC14
            Some(true),  // C20U3DC5DC8DC11
            Some(true),  // C20U3DC8DC11DC14
            Some(true),  // C20U3DC11DC14DC17
            Some(true),  // C20U4DC5DC8DC11DC14
            Some(true),  // C20U4DC8DC11DC14DC17
            Some(true),  // C20U5DC5DC8DC11DC14DC17
            Some(false), // C21U0
            Some(false), // C22U0
            Some(false), // C22U1DC13
            Some(true),  // C22U2DC13DC16
            Some(true),  // C22U3DC5DC13DC16
            Some(true),  // C22U4DC7DC10DC13DC16
            Some(true),  // C22U5DC7DC10DC13DC16DC19
            Some(true),  // C22U6DC4DC7DC10DC13DC16DC19
            Some(false), // C23U0
            Some(false), // C24U0
            Some(false), // C24U1DC15
            Some(true),  // C24U2DC15DC18
            Some(true),  // C24U3DC12DC15DC18
            Some(true),  // C24U4DC9DC12DC15DC18
            Some(true),  // C24U5DC6DC9DC12DC15DC18
            Some(true),  // C24U6DC6DC9DC12DC15DC18DC21
            Some(false), // C25U0
            Some(false), // C26U0
            Some(false), // C26U1DC17
            Some(false), // C27U0
            Some(false), // C28U0
            Some(false), // C29U0
            Some(false), // C30U0
            Some(false), // C30U1DC21
            Some(false), // C31U0
            Some(false), // C32U0
            Some(false), // C33U0
            Some(false), // C34U0
            Some(false), // C35U0
            Some(false), // C36U0
            None,        //
        ],
    );
}

#[test]
fn is_cis() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().is_cis().alias("IsCis")])
        .collect()
        .unwrap();
    let is_cis = data_frame["IsCis"]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        is_cis,
        vec![
            Some(false), // C4U0
            Some(false), // C5U0
            Some(false), // C6U0
            Some(false), // C7U0
            Some(false), // C8U0
            Some(false), // C9U0
            Some(false), // C10U0
            Some(false), // C11U0
            Some(false), // C12U0
            Some(false), // C13U0
            Some(false), // C14U0
            Some(false), // C15U0
            Some(false), // C16U0
            Some(true),  // C16U1DC9
            Some(false), // C16U1DT9
            Some(false), // C17U0
            Some(false), // C18U0
            Some(true),  // C18U1DC9
            Some(false), // C18U1DT9
            Some(true),  // C18U2DC9DC12
            Some(true),  // C18U3DC6DC9DC12
            Some(false), // C18U3DC8DT10DC12
            Some(true),  // C18U3DC9DC12DC15
            Some(false), // C18U3DC9DT11DT13
            Some(false), // C18U3DT9DT11DC13
            Some(false), // C18U3DT9DT11DT13
            Some(true),  // C18U4DC6DC9DC12DC15
            Some(false), // C19U0
            Some(false), // C20U0
            Some(true),  // C20U1DC9
            Some(true),  // C20U1DC11
            Some(true),  // C20U2DC11DC14
            Some(true),  // C20U3DC5DC8DC11
            Some(true),  // C20U3DC8DC11DC14
            Some(true),  // C20U3DC11DC14DC17
            Some(true),  // C20U4DC5DC8DC11DC14
            Some(true),  // C20U4DC8DC11DC14DC17
            Some(true),  // C20U5DC5DC8DC11DC14DC17
            Some(false), // C21U0
            Some(false), // C22U0
            Some(true),  // C22U1DC13
            Some(true),  // C22U2DC13DC16
            Some(true),  // C22U3DC5DC13DC16
            Some(true),  // C22U4DC7DC10DC13DC16
            Some(true),  // C22U5DC7DC10DC13DC16DC19
            Some(true),  // C22U6DC4DC7DC10DC13DC16DC19
            Some(false), // C23U0
            Some(false), // C24U0
            Some(true),  // C24U1DC15
            Some(true),  // C24U2DC15DC18
            Some(true),  // C24U3DC12DC15DC18
            Some(true),  // C24U4DC9DC12DC15DC18
            Some(true),  // C24U5DC6DC9DC12DC15DC18
            Some(true),  // C24U6DC6DC9DC12DC15DC18DC21
            Some(false), // C25U0
            Some(false), // C26U0
            Some(true),  // C26U1DC17
            Some(false), // C27U0
            Some(false), // C28U0
            Some(false), // C29U0
            Some(false), // C30U0
            Some(true),  // C30U1DC21
            Some(false), // C31U0
            Some(false), // C32U0
            Some(false), // C33U0
            Some(false), // C34U0
            Some(false), // C35U0
            Some(false), // C36U0
            None,        //
        ],
    );
}

#[test]
fn is_trans() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().is_trans().alias("IsTrans")])
        .collect()
        .unwrap();
    let is_trans = data_frame["IsTrans"]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        is_trans,
        vec![
            Some(false), // C4U0
            Some(false), // C5U0
            Some(false), // C6U0
            Some(false), // C7U0
            Some(false), // C8U0
            Some(false), // C9U0
            Some(false), // C10U0
            Some(false), // C11U0
            Some(false), // C12U0
            Some(false), // C13U0
            Some(false), // C14U0
            Some(false), // C15U0
            Some(false), // C16U0
            Some(false), // C16U1DC9
            Some(true),  // C16U1DT9
            Some(false), // C17U0
            Some(false), // C18U0
            Some(false), // C18U1DC9
            Some(true),  // C18U1DT9
            Some(false), // C18U2DC9DC12
            Some(false), // C18U3DC6DC9DC12
            Some(true),  // C18U3DC8DT10DC12
            Some(false), // C18U3DC9DC12DC15
            Some(true),  // C18U3DC9DT11DT13
            Some(true),  // C18U3DT9DT11DC13
            Some(true),  // C18U3DT9DT11DT13
            Some(false), // C18U4DC6DC9DC12DC15
            Some(false), // C19U0
            Some(false), // C20U0
            Some(false), // C20U1DC9
            Some(false), // C20U1DC11
            Some(false), // C20U2DC11DC14
            Some(false), // C20U3DC5DC8DC11
            Some(false), // C20U3DC8DC11DC14
            Some(false), // C20U3DC11DC14DC17
            Some(false), // C20U4DC5DC8DC11DC14
            Some(false), // C20U4DC8DC11DC14DC17
            Some(false), // C20U5DC5DC8DC11DC14DC17
            Some(false), // C21U0
            Some(false), // C22U0
            Some(false), // C22U1DC13
            Some(false), // C22U2DC13DC16
            Some(false), // C22U3DC5DC13DC16
            Some(false), // C22U4DC7DC10DC13DC16
            Some(false), // C22U5DC7DC10DC13DC16DC19
            Some(false), // C22U6DC4DC7DC10DC13DC16DC19
            Some(false), // C23U0
            Some(false), // C24U0
            Some(false), // C24U1DC15
            Some(false), // C24U2DC15DC18
            Some(false), // C24U3DC12DC15DC18
            Some(false), // C24U4DC9DC12DC15DC18
            Some(false), // C24U5DC6DC9DC12DC15DC18
            Some(false), // C24U6DC6DC9DC12DC15DC18DC21
            Some(false), // C25U0
            Some(false), // C26U0
            Some(false), // C26U1DC17
            Some(false), // C27U0
            Some(false), // C28U0
            Some(false), // C29U0
            Some(false), // C30U0
            Some(false), // C30U1DC21
            Some(false), // C31U0
            Some(false), // C32U0
            Some(false), // C33U0
            Some(false), // C34U0
            Some(false), // C35U0
            Some(false), // C36U0
            None,        //
        ],
    );
}
