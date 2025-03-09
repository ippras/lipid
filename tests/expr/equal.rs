use super::*;

#[test]
fn c4u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C4U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
        vec![
            Some(true),  // C4U0
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

#[test]
fn c5u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C5U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
        vec![
            Some(false), // C4U0
            Some(true),  // C5U0
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

#[test]
fn c6u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C6U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
        vec![
            Some(false), // C4U0
            Some(false), // C5U0
            Some(true),  // C6U0
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

#[test]
fn c7u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C7U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
        vec![
            Some(false), // C4U0
            Some(false), // C5U0
            Some(false), // C6U0
            Some(true),  // C7U0
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

#[test]
fn c8u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C8U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
        vec![
            Some(false), // C4U0
            Some(false), // C5U0
            Some(false), // C6U0
            Some(false), // C7U0
            Some(true),  // C8U0
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

#[test]
fn c9u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C9U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
        vec![
            Some(false), // C4U0
            Some(false), // C5U0
            Some(false), // C6U0
            Some(false), // C7U0
            Some(false), // C8U0
            Some(true),  // C9U0
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

#[test]
fn c10u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C10U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
        vec![
            Some(false), // C4U0
            Some(false), // C5U0
            Some(false), // C6U0
            Some(false), // C7U0
            Some(false), // C8U0
            Some(false), // C9U0
            Some(true),  // C10U0
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

#[test]
fn c11u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C11U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
        vec![
            Some(false), // C4U0
            Some(false), // C5U0
            Some(false), // C6U0
            Some(false), // C7U0
            Some(false), // C8U0
            Some(false), // C9U0
            Some(false), // C10U0
            Some(true),  // C11U0
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

#[test]
fn c12u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C12U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
        vec![
            Some(false), // C4U0
            Some(false), // C5U0
            Some(false), // C6U0
            Some(false), // C7U0
            Some(false), // C8U0
            Some(false), // C9U0
            Some(false), // C10U0
            Some(false), // C11U0
            Some(true),  // C12U0
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

#[test]
fn c13u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C13U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C13U0
            Some(false), // C14U0
            Some(false), // C15U0
            Some(false), // C16U0
            Some(false), // C16U1DC9
            Some(false), // C16U1DT9
            Some(false), // C17U0
            Some(false), // C18U0
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

#[test]
fn c14u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C14U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C14U0
            Some(false), // C15U0
            Some(false), // C16U0
            Some(false), // C16U1DC9
            Some(false), // C16U1DT9
            Some(false), // C17U0
            Some(false), // C18U0
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

#[test]
fn c15u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C15U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C15U0
            Some(false), // C16U0
            Some(false), // C16U1DC9
            Some(false), // C16U1DT9
            Some(false), // C17U0
            Some(false), // C18U0
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

#[test]
fn c16u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C16U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C16U0
            Some(false), // C16U1DC9
            Some(false), // C16U1DT9
            Some(false), // C17U0
            Some(false), // C18U0
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

#[test]
fn c16u1dc9() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C16U1DC9).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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

#[test]
fn c16u1dt9() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C16U1DT9).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(false), // C18U1DT9
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

#[test]
fn c17u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C17U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C17U0
            Some(false), // C18U0
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

#[test]
fn c18u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C18U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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

#[test]
fn c18u1dc9() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C18U1DC9).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C18U1DC9
            Some(false), // C18U1DT9
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

#[test]
fn c18u1dt9() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C18U1DT9).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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

#[test]
fn c18u2dc9dc12() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C18U2DC9DC12).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(false), // C18U3DC6DC9DC12
            Some(false), // C18U3DC8DT10DC12
            Some(false), // C18U3DC9DC12DC15
            Some(false), // C18U3DC9DT11DT13
            Some(false), // C18U3DT9DT11DC13
            Some(false), // C18U3DT9DT11DT13
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

#[test]
fn c18u3dc6dc9dc12() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .equal(C18U3DC6DC9DC12)
            .alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(false), // C18U2DC9DC12
            Some(true),  // C18U3DC6DC9DC12
            Some(false), // C18U3DC8DT10DC12
            Some(false), // C18U3DC9DC12DC15
            Some(false), // C18U3DC9DT11DT13
            Some(false), // C18U3DT9DT11DC13
            Some(false), // C18U3DT9DT11DT13
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

#[test]
fn c18u3dc8dt10dc12() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .equal(C18U3DC8DT10DC12)
            .alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(false), // C18U2DC9DC12
            Some(false), // C18U3DC6DC9DC12
            Some(true),  // C18U3DC8DT10DC12
            Some(false), // C18U3DC9DC12DC15
            Some(false), // C18U3DC9DT11DT13
            Some(false), // C18U3DT9DT11DC13
            Some(false), // C18U3DT9DT11DT13
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
#[test]
fn c18u3dc9dc12dc15() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .equal(C18U3DC9DC12DC15)
            .alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(false), // C18U2DC9DC12
            Some(false), // C18U3DC6DC9DC12
            Some(false), // C18U3DC8DT10DC12
            Some(true),  // C18U3DC9DC12DC15
            Some(false), // C18U3DC9DT11DT13
            Some(false), // C18U3DT9DT11DC13
            Some(false), // C18U3DT9DT11DT13
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

#[test]
fn c18u3dc9dt11dt13() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .equal(C18U3DC9DT11DT13)
            .alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(false), // C18U2DC9DC12
            Some(false), // C18U3DC6DC9DC12
            Some(false), // C18U3DC8DT10DC12
            Some(false), // C18U3DC9DC12DC15
            Some(true),  // C18U3DC9DT11DT13
            Some(false), // C18U3DT9DT11DC13
            Some(false), // C18U3DT9DT11DT13
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

#[test]
fn c18u3dt9dt11dc13() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .equal(C18U3DT9DT11DC13)
            .alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(false), // C18U2DC9DC12
            Some(false), // C18U3DC6DC9DC12
            Some(false), // C18U3DC8DT10DC12
            Some(false), // C18U3DC9DC12DC15
            Some(false), // C18U3DC9DT11DT13
            Some(true),  // C18U3DT9DT11DC13
            Some(false), // C18U3DT9DT11DT13
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

#[test]
fn c18u3dt9dt11dt13() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .equal(C18U3DT9DT11DT13)
            .alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(false), // C18U2DC9DC12
            Some(false), // C18U3DC6DC9DC12
            Some(false), // C18U3DC8DT10DC12
            Some(false), // C18U3DC9DC12DC15
            Some(false), // C18U3DC9DT11DT13
            Some(false), // C18U3DT9DT11DC13
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

#[test]
fn c18u4dc6dc9dc12dc15() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .equal(C18U4DC6DC9DC12DC15)
            .alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(false), // C18U2DC9DC12
            Some(false), // C18U3DC6DC9DC12
            Some(false), // C18U3DC8DT10DC12
            Some(false), // C18U3DC9DC12DC15
            Some(false), // C18U3DC9DT11DT13
            Some(false), // C18U3DT9DT11DC13
            Some(false), // C18U3DT9DT11DT13
            Some(true),  // C18U4DC6DC9DC12DC15
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

#[test]
fn c19u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C19U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(false), // C18U2DC9DC12
            Some(false), // C18U3DC6DC9DC12
            Some(false), // C18U3DC8DT10DC12
            Some(false), // C18U3DC9DC12DC15
            Some(false), // C18U3DC9DT11DT13
            Some(false), // C18U3DT9DT11DC13
            Some(false), // C18U3DT9DT11DT13
            Some(false), // C18U4DC6DC9DC12DC15
            Some(true),  // C19U0
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

#[test]
fn c20u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C20U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(false), // C18U2DC9DC12
            Some(false), // C18U3DC6DC9DC12
            Some(false), // C18U3DC8DT10DC12
            Some(false), // C18U3DC9DC12DC15
            Some(false), // C18U3DC9DT11DT13
            Some(false), // C18U3DT9DT11DC13
            Some(false), // C18U3DT9DT11DT13
            Some(false), // C18U4DC6DC9DC12DC15
            Some(false), // C19U0
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

#[test]
fn c20u1dc9() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C20U1DC9).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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

#[test]
fn c20u1dc11() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C20U1DC11).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(false), // C20U1DC9
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

#[test]
fn c20u2dc11dc14() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C20U2DC11DC14).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(false), // C20U1DC9
            Some(false), // C20U1DC11
            Some(true),  // C20U2DC11DC14
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

#[test]
fn c20u3dc5dc8dc11() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .equal(C20U3DC5DC8DC11)
            .alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(false), // C20U1DC9
            Some(false), // C20U1DC11
            Some(false), // C20U2DC11DC14
            Some(true),  // C20U3DC5DC8DC11
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

#[test]
fn c20u3dc8dc11dc14() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .equal(C20U3DC8DC11DC14)
            .alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(false), // C20U1DC9
            Some(false), // C20U1DC11
            Some(false), // C20U2DC11DC14
            Some(false), // C20U3DC5DC8DC11
            Some(true),  // C20U3DC8DC11DC14
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

#[test]
fn c20u3dc11dc14dc17() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .equal(C20U3DC11DC14DC17)
            .alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(false), // C20U1DC9
            Some(false), // C20U1DC11
            Some(false), // C20U2DC11DC14
            Some(false), // C20U3DC5DC8DC11
            Some(false), // C20U3DC8DC11DC14
            Some(true),  // C20U3DC11DC14DC17
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

#[test]
fn c20u4dc5dc8dc11dc14() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .equal(C20U4DC5DC8DC11DC14)
            .alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(false), // C20U1DC9
            Some(false), // C20U1DC11
            Some(false), // C20U2DC11DC14
            Some(false), // C20U3DC5DC8DC11
            Some(false), // C20U3DC8DC11DC14
            Some(false), // C20U3DC11DC14DC17
            Some(true),  // C20U4DC5DC8DC11DC14
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

#[test]
fn c20u4dc8dc11dc14dc17() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .equal(C20U4DC8DC11DC14DC17)
            .alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(false), // C20U1DC9
            Some(false), // C20U1DC11
            Some(false), // C20U2DC11DC14
            Some(false), // C20U3DC5DC8DC11
            Some(false), // C20U3DC8DC11DC14
            Some(false), // C20U3DC11DC14DC17
            Some(false), // C20U4DC5DC8DC11DC14
            Some(true),  // C20U4DC8DC11DC14DC17
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

#[test]
fn c20u5dc5dc8dc11dc14dc17() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .equal(C20U5DC5DC8DC11DC14DC17)
            .alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(false), // C20U1DC9
            Some(false), // C20U1DC11
            Some(false), // C20U2DC11DC14
            Some(false), // C20U3DC5DC8DC11
            Some(false), // C20U3DC8DC11DC14
            Some(false), // C20U3DC11DC14DC17
            Some(false), // C20U4DC5DC8DC11DC14
            Some(false), // C20U4DC8DC11DC14DC17
            Some(true),  // C20U5DC5DC8DC11DC14DC17
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

#[test]
fn c21u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C21U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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

#[test]
fn c22u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C22U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C22U0
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

#[test]
fn c22u1dc13() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C22U1DC13).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C22U1DC13
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

#[test]
fn c22u2dc13dc16() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C22U2DC13DC16).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C22U2DC13DC16
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

#[test]
fn c22u3dc5dc13dc16() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .equal(C22U3DC5DC13DC16)
            .alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C22U3DC5DC13DC16
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

#[test]
fn c22u4dc7dc10dc13dc16() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .equal(C22U4DC7DC10DC13DC16)
            .alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C22U4DC7DC10DC13DC16
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

#[test]
fn c22u5dc7dc10dc13dc16dc19() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .equal(C22U5DC7DC10DC13DC16DC19)
            .alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C22U5DC7DC10DC13DC16DC19
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

#[test]
fn c22u6dc4dc7dc10dc13dc16dc19() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .equal(C22U6DC4DC7DC10DC13DC16DC19)
            .alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C22U6DC4DC7DC10DC13DC16DC19
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

#[test]
fn c23u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C23U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C23U0
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

#[test]
fn c24u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C24U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C24U0
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

#[test]
fn c24u1dc15() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C24U1DC15).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C24U1DC15
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

#[test]
fn c24u2dc15dc18() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C24U2DC15DC18).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C24U2DC15DC18
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

#[test]
fn c24u3dc12dc15dc18() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .equal(C24U3DC12DC15DC18)
            .alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C24U3DC12DC15DC18
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

#[test]
fn c24u4dc9dc12dc15dc18() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .equal(C24U4DC9DC12DC15DC18)
            .alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C24U4DC9DC12DC15DC18
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

#[test]
fn c24u5dc6dc9dc12dc15dc18() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .equal(C24U5DC6DC9DC12DC15DC18)
            .alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C24U5DC6DC9DC12DC15DC18
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

#[test]
fn c24u6dc6dc9dc12dc15dc18dc21() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .equal(C24U6DC6DC9DC12DC15DC18DC21)
            .alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
fn c25u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C25U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C25U0
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
fn c26u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C26U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C26U0
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
fn c26u1dc17() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C26U1DC17).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C26U1DC17
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
fn c27u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C27U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C27U0
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
fn c28u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C28U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C28U0
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
fn c29u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C29U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C29U0
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
fn c30u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C30U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C30U0
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
fn c30u1dc21() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C30U1DC21).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
fn c31u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C31U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C31U0
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
fn c32u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C32U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C32U0
            Some(false), // C33U0
            Some(false), // C34U0
            Some(false), // C35U0
            Some(false), // C36U0
            None,        //
        ],
    );
}

#[test]
fn c33u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C33U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C33U0
            Some(false), // C34U0
            Some(false), // C35U0
            Some(false), // C36U0
            None,        //
        ],
    );
}

#[test]
fn c34u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C34U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C34U0
            Some(false), // C35U0
            Some(false), // C36U0
            None,        //
        ],
    );
}

#[test]
fn c35u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C35U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C35U0
            Some(false), // C36U0
            None,        //
        ],
    );
}

#[test]
fn c36u0() {
    let data_frame = SOURCE
        .clone()
        .lazy()
        .select([col("FattyAcid").fatty_acid().equal(C36U0).alias("")])
        .collect()
        .unwrap();
    let target = data_frame[""]
        .as_materialized_series()
        .bool()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        target,
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
            Some(true),  // C36U0
            None,        //
        ],
    );
}
