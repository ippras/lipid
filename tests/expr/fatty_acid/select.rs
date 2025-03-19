use super::*;

#[test]
fn saturated() -> PolarsResult<()> {
    let data_frame = FATTY_ACIDS
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .nullify(col("FattyAcid").fatty_acid().is_saturated())
            .alias("")])
        .collect()
        .unwrap();
    let saturated = data_frame[""]
        .as_materialized_series()
        .fatty_acid()?
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        saturated,
        [
            Some(Series::from_iter(C4U0)),  // C4U0
            Some(Series::from_iter(C5U0)),  // C5U0
            Some(Series::from_iter(C6U0)),  // C6U0
            Some(Series::from_iter(C7U0)),  // C7U0
            Some(Series::from_iter(C8U0)),  // C8U0
            Some(Series::from_iter(C9U0)),  // C9U0
            Some(Series::from_iter(C10U0)), // C10U0
            Some(Series::from_iter(C11U0)), // C11U0
            Some(Series::from_iter(C12U0)), // C12U0
            Some(Series::from_iter(C13U0)), // C13U0
            Some(Series::from_iter(C14U0)), // C14U0
            Some(Series::from_iter(C15U0)), // C15U0
            Some(Series::from_iter(C16U0)), // C16U0
            None,                           // C16U1DC9
            None,                           // C16U1DT9
            Some(Series::from_iter(C17U0)), // C17U0
            Some(Series::from_iter(C18U0)), // C18U0
            None,                           // C18U1DC9
            None,                           // C18U1DT9
            None,                           // C18U2DC9DC12
            None,                           // C18U3DC6DC9DC12
            None,                           // C18U3DC8DT10DC12
            None,                           // C18U3DC9DC12DC15
            None,                           // C18U3DC9DT11DT13
            None,                           // C18U3DT9DT11DC13
            None,                           // C18U3DT9DT11DT13
            None,                           // C18U4DC6DC9DC12DC15
            Some(Series::from_iter(C19U0)), // C19U0
            Some(Series::from_iter(C20U0)), // C20U0
            None,                           // C20U1DC9
            None,                           // C20U1DC11
            None,                           // C20U2DC11DC14
            None,                           // C20U3DC5DC8DC11
            None,                           // C20U3DC8DC11DC14
            None,                           // C20U3DC11DC14DC17
            None,                           // C20U4DC5DC8DC11DC14
            None,                           // C20U4DC8DC11DC14DC17
            None,                           // C20U5DC5DC8DC11DC14DC17
            Some(Series::from_iter(C21U0)), // C21U0
            Some(Series::from_iter(C22U0)), // C22U0
            None,                           // C22U1DC13
            None,                           // C22U2DC13DC16
            None,                           // C22U3DC5DC13DC16
            None,                           // C22U4DC7DC10DC13DC16
            None,                           // C22U5DC7DC10DC13DC16DC19
            None,                           // C22U6DC4DC7DC10DC13DC16DC19
            Some(Series::from_iter(C23U0)), // C23U0
            Some(Series::from_iter(C24U0)), // C24U0
            None,                           // C24U1DC15
            None,                           // C24U2DC15DC18
            None,                           // C24U3DC12DC15DC18
            None,                           // C24U4DC9DC12DC15DC18
            None,                           // C24U5DC6DC9DC12DC15DC18
            None,                           // C24U6DC6DC9DC12DC15DC18DC21
            Some(Series::from_iter(C25U0)), // C25U0
            Some(Series::from_iter(C26U0)), // C26U0
            None,                           // C26U1DC17
            Some(Series::from_iter(C27U0)), // C27U0
            Some(Series::from_iter(C28U0)), // C28U0
            Some(Series::from_iter(C29U0)), // C29U0
            Some(Series::from_iter(C30U0)), // C30U0
            None,                           // C30U1DC21
            Some(Series::from_iter(C31U0)), // C31U0
            Some(Series::from_iter(C32U0)), // C32U0
            Some(Series::from_iter(C33U0)), // C33U0
            Some(Series::from_iter(C34U0)), // C34U0
            Some(Series::from_iter(C35U0)), // C35U0
            Some(Series::from_iter(C36U0)), // C36U0
            None,                           //
        ],
    );
    Ok(())
}

#[test]
fn unsaturated() -> PolarsResult<()> {
    let data_frame = FATTY_ACIDS
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .nullify(col("FattyAcid").fatty_acid().is_unsaturated(None))
            .alias("")])
        .collect()?;
    let unsaturated = data_frame[""]
        .as_materialized_series()
        .fatty_acid()?
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        unsaturated,
        [
            None,                                                 // C4U0
            None,                                                 // C5U0
            None,                                                 // C6U0
            None,                                                 // C7U0
            None,                                                 // C8U0
            None,                                                 // C9U0
            None,                                                 // C10U0
            None,                                                 // C11U0
            None,                                                 // C12U0
            None,                                                 // C13U0
            None,                                                 // C14U0
            None,                                                 // C15U0
            None,                                                 // C16U0
            Some(Series::from_iter(C16U1DC9)),                    // C16U1DC9
            Some(Series::from_iter(C16U1DT9)),                    // C16U1DT9
            None,                                                 // C17U0
            None,                                                 // C18U0
            Some(Series::from_iter(C18U1DC9)),                    // C18U1DC9
            Some(Series::from_iter(C18U1DT9)),                    // C18U1DT9
            Some(Series::from_iter(C18U2DC9DC12)),                // C18U2DC9DC12
            Some(Series::from_iter(C18U3DC6DC9DC12)),             // C18U3DC6DC9DC12
            Some(Series::from_iter(C18U3DC8DT10DC12)),            // C18U3DC8DT10DC12
            Some(Series::from_iter(C18U3DC9DC12DC15)),            // C18U3DC9DC12DC15
            Some(Series::from_iter(C18U3DC9DT11DT13)),            // C18U3DC9DT11DT13
            Some(Series::from_iter(C18U3DT9DT11DC13)),            // C18U3DT9DT11DC13
            Some(Series::from_iter(C18U3DT9DT11DT13)),            // C18U3DT9DT11DT13
            Some(Series::from_iter(C18U4DC6DC9DC12DC15)),         // C18U4DC6DC9DC12DC15
            None,                                                 // C19U0
            None,                                                 // C20U0
            Some(Series::from_iter(C20U1DC9)),                    // C20U1DC9
            Some(Series::from_iter(C20U1DC11)),                   // C20U1DC11
            Some(Series::from_iter(C20U2DC11DC14)),               // C20U2DC11DC14
            Some(Series::from_iter(C20U3DC5DC8DC11)),             // C20U3DC5DC8DC11
            Some(Series::from_iter(C20U3DC8DC11DC14)),            // C20U3DC8DC11DC14
            Some(Series::from_iter(C20U3DC11DC14DC17)),           // C20U3DC11DC14DC17
            Some(Series::from_iter(C20U4DC5DC8DC11DC14)),         // C20U4DC5DC8DC11DC14
            Some(Series::from_iter(C20U4DC8DC11DC14DC17)),        // C20U4DC8DC11DC14DC17
            Some(Series::from_iter(C20U5DC5DC8DC11DC14DC17)),     // C20U5DC5DC8DC11DC14DC17
            None,                                                 // C21U0
            None,                                                 // C22U0
            Some(Series::from_iter(C22U1DC13)),                   // C22U1DC13
            Some(Series::from_iter(C22U2DC13DC16)),               // C22U2DC13DC16
            Some(Series::from_iter(C22U3DC5DC13DC16)),            // C22U3DC5DC13DC16
            Some(Series::from_iter(C22U4DC7DC10DC13DC16)),        // C22U4DC7DC10DC13DC16
            Some(Series::from_iter(C22U5DC7DC10DC13DC16DC19)),    // C22U5DC7DC10DC13DC16DC19
            Some(Series::from_iter(C22U6DC4DC7DC10DC13DC16DC19)), // C22U6DC4DC7DC10DC13DC16DC19
            None,                                                 // C23U0
            None,                                                 // C24U0
            Some(Series::from_iter(C24U1DC15)),                   // C24U1DC15
            Some(Series::from_iter(C24U2DC15DC18)),               // C24U2DC15DC18
            Some(Series::from_iter(C24U3DC12DC15DC18)),           // C24U3DC12DC15DC18
            Some(Series::from_iter(C24U4DC9DC12DC15DC18)),        // C24U4DC9DC12DC15DC18
            Some(Series::from_iter(C24U5DC6DC9DC12DC15DC18)),     // C24U5DC6DC9DC12DC15DC18
            Some(Series::from_iter(C24U6DC6DC9DC12DC15DC18DC21)), // C24U6DC6DC9DC12DC15DC18DC21
            None,                                                 // C25U0
            None,                                                 // C26U0
            Some(Series::from_iter(C26U1DC17)),                   // C26U1DC17
            None,                                                 // C27U0
            None,                                                 // C28U0
            None,                                                 // C29U0
            None,                                                 // C30U0
            Some(Series::from_iter(C30U1DC21)),                   // C30U1DC21
            None,                                                 // C31U0
            None,                                                 // C32U0
            None,                                                 // C33U0
            None,                                                 // C34U0
            None,                                                 // C35U0
            None,                                                 // C36U0
            None,                                                 //
        ],
    );
    Ok(())
}

#[test]
fn monounsaturated() -> PolarsResult<()> {
    let data_frame = FATTY_ACIDS
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .nullify(col("FattyAcid").fatty_acid().is_monounsaturated())
            .alias("")])
        .collect()?;
    let monounsaturated = data_frame[""]
        .as_materialized_series()
        .fatty_acid()?
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        monounsaturated,
        [
            None,                               // C4U0
            None,                               // C5U0
            None,                               // C6U0
            None,                               // C7U0
            None,                               // C8U0
            None,                               // C9U0
            None,                               // C10U0
            None,                               // C11U0
            None,                               // C12U0
            None,                               // C13U0
            None,                               // C14U0
            None,                               // C15U0
            None,                               // C16U0
            Some(Series::from_iter(C16U1DC9)),  // C16U1DC9
            Some(Series::from_iter(C16U1DT9)),  // C16U1DT9
            None,                               // C17U0
            None,                               // C18U0
            Some(Series::from_iter(C18U1DC9)),  // C18U1DC9
            Some(Series::from_iter(C18U1DT9)),  // C18U1DT9
            None,                               // C18U2DC9DC12
            None,                               // C18U3DC6DC9DC12
            None,                               // C18U3DC8DT10DC12
            None,                               // C18U3DC9DC12DC15
            None,                               // C18U3DC9DT11DT13
            None,                               // C18U3DT9DT11DC13
            None,                               // C18U3DT9DT11DT13
            None,                               // C18U4DC6DC9DC12DC15
            None,                               // C19U0
            None,                               // C20U0
            Some(Series::from_iter(C20U1DC9)),  // C20U1DC9
            Some(Series::from_iter(C20U1DC11)), // C20U1DC11
            None,                               // C20U2DC11DC14
            None,                               // C20U3DC5DC8DC11
            None,                               // C20U3DC8DC11DC14
            None,                               // C20U3DC11DC14DC17
            None,                               // C20U4DC5DC8DC11DC14
            None,                               // C20U4DC8DC11DC14DC17
            None,                               // C20U5DC5DC8DC11DC14DC17
            None,                               // C21U0
            None,                               // C22U0
            Some(Series::from_iter(C22U1DC13)), // C22U1DC13
            None,                               // C22U2DC13DC16
            None,                               // C22U3DC5DC13DC16
            None,                               // C22U4DC7DC10DC13DC16
            None,                               // C22U5DC7DC10DC13DC16DC19
            None,                               // C22U6DC4DC7DC10DC13DC16DC19
            None,                               // C23U0
            None,                               // C24U0
            Some(Series::from_iter(C24U1DC15)), // C24U1DC15
            None,                               // C24U2DC15DC18
            None,                               // C24U3DC12DC15DC18
            None,                               // C24U4DC9DC12DC15DC18
            None,                               // C24U5DC6DC9DC12DC15DC18
            None,                               // C24U6DC6DC9DC12DC15DC18DC21
            None,                               // C25U0
            None,                               // C26U0
            Some(Series::from_iter(C26U1DC17)), // C26U1DC17
            None,                               // C27U0
            None,                               // C28U0
            None,                               // C29U0
            None,                               // C30U0
            Some(Series::from_iter(C30U1DC21)), // C30U1DC21
            None,                               // C31U0
            None,                               // C32U0
            None,                               // C33U0
            None,                               // C34U0
            None,                               // C35U0
            None,                               // C36U0
            None,                               //
        ],
    );
    Ok(())
}

#[test]
fn polyunsaturated() -> PolarsResult<()> {
    let data_frame = FATTY_ACIDS
        .clone()
        .lazy()
        .select([col("FattyAcid")
            .fatty_acid()
            .nullify(col("FattyAcid").fatty_acid().is_polyunsaturated())
            .alias("")])
        .collect()?;
    let polyunsaturated = data_frame[""]
        .as_materialized_series()
        .fatty_acid()?
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(
        polyunsaturated,
        [
            None,                                                 // C4U0
            None,                                                 // C5U0
            None,                                                 // C6U0
            None,                                                 // C7U0
            None,                                                 // C8U0
            None,                                                 // C9U0
            None,                                                 // C10U0
            None,                                                 // C11U0
            None,                                                 // C12U0
            None,                                                 // C13U0
            None,                                                 // C14U0
            None,                                                 // C15U0
            None,                                                 // C16U0
            None,                                                 // C16U1DC9
            None,                                                 // C16U1DT9
            None,                                                 // C17U0
            None,                                                 // C18U0
            None,                                                 // C18U1DC9
            None,                                                 // C18U1DT9
            Some(Series::from_iter(C18U2DC9DC12)),                // C18U2DC9DC12
            Some(Series::from_iter(C18U3DC6DC9DC12)),             // C18U3DC6DC9DC12
            Some(Series::from_iter(C18U3DC8DT10DC12)),            // C18U3DC8DT10DC12
            Some(Series::from_iter(C18U3DC9DC12DC15)),            // C18U3DC9DC12DC15
            Some(Series::from_iter(C18U3DC9DT11DT13)),            // C18U3DC9DT11DT13
            Some(Series::from_iter(C18U3DT9DT11DC13)),            // C18U3DT9DT11DC13
            Some(Series::from_iter(C18U3DT9DT11DT13)),            // C18U3DT9DT11DT13
            Some(Series::from_iter(C18U4DC6DC9DC12DC15)),         // C18U4DC6DC9DC12DC15
            None,                                                 // C19U0
            None,                                                 // C20U0
            None,                                                 // C20U1DC9
            None,                                                 // C20U1DC11
            Some(Series::from_iter(C20U2DC11DC14)),               // C20U2DC11DC14
            Some(Series::from_iter(C20U3DC5DC8DC11)),             // C20U3DC5DC8DC11
            Some(Series::from_iter(C20U3DC8DC11DC14)),            // C20U3DC8DC11DC14
            Some(Series::from_iter(C20U3DC11DC14DC17)),           // C20U3DC11DC14DC17
            Some(Series::from_iter(C20U4DC5DC8DC11DC14)),         // C20U4DC5DC8DC11DC14
            Some(Series::from_iter(C20U4DC8DC11DC14DC17)),        // C20U4DC8DC11DC14DC17
            Some(Series::from_iter(C20U5DC5DC8DC11DC14DC17)),     // C20U5DC5DC8DC11DC14DC17
            None,                                                 // C21U0
            None,                                                 // C22U0
            None,                                                 // C22U1DC13
            Some(Series::from_iter(C22U2DC13DC16)),               // C22U2DC13DC16
            Some(Series::from_iter(C22U3DC5DC13DC16)),            // C22U3DC5DC13DC16
            Some(Series::from_iter(C22U4DC7DC10DC13DC16)),        // C22U4DC7DC10DC13DC16
            Some(Series::from_iter(C22U5DC7DC10DC13DC16DC19)),    // C22U5DC7DC10DC13DC16DC19
            Some(Series::from_iter(C22U6DC4DC7DC10DC13DC16DC19)), // C22U6DC4DC7DC10DC13DC16DC19
            None,                                                 // C23U0
            None,                                                 // C24U0
            None,                                                 // C24U1DC15
            Some(Series::from_iter(C24U2DC15DC18)),               // C24U2DC15DC18
            Some(Series::from_iter(C24U3DC12DC15DC18)),           // C24U3DC12DC15DC18
            Some(Series::from_iter(C24U4DC9DC12DC15DC18)),        // C24U4DC9DC12DC15DC18
            Some(Series::from_iter(C24U5DC6DC9DC12DC15DC18)),     // C24U5DC6DC9DC12DC15DC18
            Some(Series::from_iter(C24U6DC6DC9DC12DC15DC18DC21)), // C24U6DC6DC9DC12DC15DC18DC21
            None,                                                 // C25U0
            None,                                                 // C26U0
            None,                                                 // C26U1DC17
            None,                                                 // C27U0
            None,                                                 // C28U0
            None,                                                 // C29U0
            None,                                                 // C30U0
            None,                                                 // C30U1DC21
            None,                                                 // C31U0
            None,                                                 // C32U0
            None,                                                 // C33U0
            None,                                                 // C34U0
            None,                                                 // C35U0
            None,                                                 // C36U0
            None,                                                 //
        ],
    );
    Ok(())
}
