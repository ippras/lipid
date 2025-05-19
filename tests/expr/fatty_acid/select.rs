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
    let got: Vec<_> = data_frame[""].fatty_acid_list().into_iter().collect();
    assert_eq!(
        got,
        [
            Some(FattyAcidChunked::try_from(C4)?),  // C4
            Some(FattyAcidChunked::try_from(C5)?),  // C5
            Some(FattyAcidChunked::try_from(C6)?),  // C6
            Some(FattyAcidChunked::try_from(C7)?),  // C7
            Some(FattyAcidChunked::try_from(C8)?),  // C8
            Some(FattyAcidChunked::try_from(C9)?),  // C9
            Some(FattyAcidChunked::try_from(C10)?), // C10
            Some(FattyAcidChunked::try_from(C11)?), // C11
            Some(FattyAcidChunked::try_from(C12)?), // C12
            Some(FattyAcidChunked::try_from(C13)?), // C13
            Some(FattyAcidChunked::try_from(C14)?), // C14
            Some(FattyAcidChunked::try_from(C15)?), // C15
            Some(FattyAcidChunked::try_from(C16)?), // C16
            None,                                   // C16DC9
            None,                                   // C16DT9
            Some(FattyAcidChunked::try_from(C17)?), // C17
            Some(FattyAcidChunked::try_from(C18)?), // C18
            None,                                   // C18DC9
            None,                                   // C18DT9
            None,                                   // C18DC9DC12
            None,                                   // C18DC6DC9DC12
            None,                                   // C18DC8DT10DC12
            None,                                   // C18DC9DC12DC15
            None,                                   // C18DC9DT11DT13
            None,                                   // C18DT9DT11DC13
            None,                                   // C18DT9DT11DT13
            None,                                   // C18DC6DC9DC12DC15
            Some(FattyAcidChunked::try_from(C19)?), // C19
            Some(FattyAcidChunked::try_from(C20)?), // C20
            None,                                   // C20DC9
            None,                                   // C20DC11
            None,                                   // C20DC11DC14
            None,                                   // C20DC5DC8DC11
            None,                                   // C20DC8DC11DC14
            None,                                   // C20DC11DC14DC17
            None,                                   // C20DC5DC8DC11DC14
            None,                                   // C20DC8DC11DC14DC17
            None,                                   // C20DC5DC8DC11DC14DC17
            Some(FattyAcidChunked::try_from(C21)?), // C21
            Some(FattyAcidChunked::try_from(C22)?), // C22
            None,                                   // C22DC13
            None,                                   // C22DC13DC16
            None,                                   // C22DC5DC13DC16
            None,                                   // C22DC7DC10DC13DC16
            None,                                   // C22DC7DC10DC13DC16DC19
            None,                                   // C22DC4DC7DC10DC13DC16DC19
            Some(FattyAcidChunked::try_from(C23)?), // C23
            Some(FattyAcidChunked::try_from(C24)?), // C24
            None,                                   // C24DC15
            None,                                   // C24DC15DC18
            None,                                   // C24DC12DC15DC18
            None,                                   // C24DC9DC12DC15DC18
            None,                                   // C24DC6DC9DC12DC15DC18
            None,                                   // C24DC6DC9DC12DC15DC18DC21
            Some(FattyAcidChunked::try_from(C25)?), // C25
            Some(FattyAcidChunked::try_from(C26)?), // C26
            None,                                   // C26DC17
            Some(FattyAcidChunked::try_from(C27)?), // C27
            Some(FattyAcidChunked::try_from(C28)?), // C28
            Some(FattyAcidChunked::try_from(C29)?), // C29
            Some(FattyAcidChunked::try_from(C30)?), // C30
            None,                                   // C30DC21
            Some(FattyAcidChunked::try_from(C31)?), // C31
            Some(FattyAcidChunked::try_from(C32)?), // C32
            Some(FattyAcidChunked::try_from(C33)?), // C33
            Some(FattyAcidChunked::try_from(C34)?), // C34
            Some(FattyAcidChunked::try_from(C35)?), // C35
            Some(FattyAcidChunked::try_from(C36)?), // C36
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
    let got: Vec<_> = data_frame[""].fatty_acid_list().into_iter().collect();
    assert_eq!(
        got,
        [
            None,                                                         // C4
            None,                                                         // C5
            None,                                                         // C6
            None,                                                         // C7
            None,                                                         // C8
            None,                                                         // C9
            None,                                                         // C10
            None,                                                         // C11
            None,                                                         // C12
            None,                                                         // C13
            None,                                                         // C14
            None,                                                         // C15
            None,                                                         // C16
            Some(FattyAcidChunked::try_from(C16DC9)?),                    // C16DC9
            Some(FattyAcidChunked::try_from(C16DT9)?),                    // C16DT9
            None,                                                         // C17
            None,                                                         // C18
            Some(FattyAcidChunked::try_from(C18DC9)?),                    // C18DC9
            Some(FattyAcidChunked::try_from(C18DT9)?),                    // C18DT9
            Some(FattyAcidChunked::try_from(C18DC9DC12)?),                // C18DC9DC12
            Some(FattyAcidChunked::try_from(C18DC6DC9DC12)?),             // C18DC6DC9DC12
            Some(FattyAcidChunked::try_from(C18DC8DT10DC12)?),            // C18DC8DT10DC12
            Some(FattyAcidChunked::try_from(C18DC9DC12DC15)?),            // C18DC9DC12DC15
            Some(FattyAcidChunked::try_from(C18DC9DT11DT13)?),            // C18DC9DT11DT13
            Some(FattyAcidChunked::try_from(C18DT9DT11DC13)?),            // C18DT9DT11DC13
            Some(FattyAcidChunked::try_from(C18DT9DT11DT13)?),            // C18DT9DT11DT13
            Some(FattyAcidChunked::try_from(C18DC6DC9DC12DC15)?),         // C18DC6DC9DC12DC15
            None,                                                         // C19
            None,                                                         // C20
            Some(FattyAcidChunked::try_from(C20DC9)?),                    // C20DC9
            Some(FattyAcidChunked::try_from(C20DC11)?),                   // C20DC11
            Some(FattyAcidChunked::try_from(C20DC11DC14)?),               // C20DC11DC14
            Some(FattyAcidChunked::try_from(C20DC5DC8DC11)?),             // C20DC5DC8DC11
            Some(FattyAcidChunked::try_from(C20DC8DC11DC14)?),            // C20DC8DC11DC14
            Some(FattyAcidChunked::try_from(C20DC11DC14DC17)?),           // C20DC11DC14DC17
            Some(FattyAcidChunked::try_from(C20DC5DC8DC11DC14)?),         // C20DC5DC8DC11DC14
            Some(FattyAcidChunked::try_from(C20DC8DC11DC14DC17)?),        // C20DC8DC11DC14DC17
            Some(FattyAcidChunked::try_from(C20DC5DC8DC11DC14DC17)?),     // C20DC5DC8DC11DC14DC17
            None,                                                         // C21
            None,                                                         // C22
            Some(FattyAcidChunked::try_from(C22DC13)?),                   // C22DC13
            Some(FattyAcidChunked::try_from(C22DC13DC16)?),               // C22DC13DC16
            Some(FattyAcidChunked::try_from(C22DC5DC13DC16)?),            // C22DC5DC13DC16
            Some(FattyAcidChunked::try_from(C22DC7DC10DC13DC16)?),        // C22DC7DC10DC13DC16
            Some(FattyAcidChunked::try_from(C22DC7DC10DC13DC16DC19)?),    // C22DC7DC10DC13DC16DC19
            Some(FattyAcidChunked::try_from(C22DC4DC7DC10DC13DC16DC19)?), // C22DC4DC7DC10DC13DC16DC19
            None,                                                         // C23
            None,                                                         // C24
            Some(FattyAcidChunked::try_from(C24DC15)?),                   // C24DC15
            Some(FattyAcidChunked::try_from(C24DC15DC18)?),               // C24DC15DC18
            Some(FattyAcidChunked::try_from(C24DC12DC15DC18)?),           // C24DC12DC15DC18
            Some(FattyAcidChunked::try_from(C24DC9DC12DC15DC18)?),        // C24DC9DC12DC15DC18
            Some(FattyAcidChunked::try_from(C24DC6DC9DC12DC15DC18)?),     // C24DC6DC9DC12DC15DC18
            Some(FattyAcidChunked::try_from(C24DC6DC9DC12DC15DC18DC21)?), // C24DC6DC9DC12DC15DC18DC21
            None,                                                         // C25
            None,                                                         // C26
            Some(FattyAcidChunked::try_from(C26DC17)?),                   // C26DC17
            None,                                                         // C27
            None,                                                         // C28
            None,                                                         // C29
            None,                                                         // C30
            Some(FattyAcidChunked::try_from(C30DC21)?),                   // C30DC21
            None,                                                         // C31
            None,                                                         // C32
            None,                                                         // C33
            None,                                                         // C34
            None,                                                         // C35
            None,                                                         // C36
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
    let got: Vec<_> = data_frame[""].fatty_acid_list().into_iter().collect();
    assert_eq!(
        got,
        [
            None,                                       // C4
            None,                                       // C5
            None,                                       // C6
            None,                                       // C7
            None,                                       // C8
            None,                                       // C9
            None,                                       // C10
            None,                                       // C11
            None,                                       // C12
            None,                                       // C13
            None,                                       // C14
            None,                                       // C15
            None,                                       // C16
            Some(FattyAcidChunked::try_from(C16DC9)?),  // C16DC9
            Some(FattyAcidChunked::try_from(C16DT9)?),  // C16DT9
            None,                                       // C17
            None,                                       // C18
            Some(FattyAcidChunked::try_from(C18DC9)?),  // C18DC9
            Some(FattyAcidChunked::try_from(C18DT9)?),  // C18DT9
            None,                                       // C18DC9DC12
            None,                                       // C18DC6DC9DC12
            None,                                       // C18DC8DT10DC12
            None,                                       // C18DC9DC12DC15
            None,                                       // C18DC9DT11DT13
            None,                                       // C18DT9DT11DC13
            None,                                       // C18DT9DT11DT13
            None,                                       // C18DC6DC9DC12DC15
            None,                                       // C19
            None,                                       // C20
            Some(FattyAcidChunked::try_from(C20DC9)?),  // C20DC9
            Some(FattyAcidChunked::try_from(C20DC11)?), // C20DC11
            None,                                       // C20DC11DC14
            None,                                       // C20DC5DC8DC11
            None,                                       // C20DC8DC11DC14
            None,                                       // C20DC11DC14DC17
            None,                                       // C20DC5DC8DC11DC14
            None,                                       // C20DC8DC11DC14DC17
            None,                                       // C20DC5DC8DC11DC14DC17
            None,                                       // C21
            None,                                       // C22
            Some(FattyAcidChunked::try_from(C22DC13)?), // C22DC13
            None,                                       // C22DC13DC16
            None,                                       // C22DC5DC13DC16
            None,                                       // C22DC7DC10DC13DC16
            None,                                       // C22DC7DC10DC13DC16DC19
            None,                                       // C22DC4DC7DC10DC13DC16DC19
            None,                                       // C23
            None,                                       // C24
            Some(FattyAcidChunked::try_from(C24DC15)?), // C24DC15
            None,                                       // C24DC15DC18
            None,                                       // C24DC12DC15DC18
            None,                                       // C24DC9DC12DC15DC18
            None,                                       // C24DC6DC9DC12DC15DC18
            None,                                       // C24DC6DC9DC12DC15DC18DC21
            None,                                       // C25
            None,                                       // C26
            Some(FattyAcidChunked::try_from(C26DC17)?), // C26DC17
            None,                                       // C27
            None,                                       // C28
            None,                                       // C29
            None,                                       // C30
            Some(FattyAcidChunked::try_from(C30DC21)?), // C30DC21
            None,                                       // C31
            None,                                       // C32
            None,                                       // C33
            None,                                       // C34
            None,                                       // C35
            None,                                       // C36
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
    let got: Vec<_> = data_frame[""].fatty_acid_list().into_iter().collect();
    assert_eq!(
        got,
        [
            None,                                                         // C4
            None,                                                         // C5
            None,                                                         // C6
            None,                                                         // C7
            None,                                                         // C8
            None,                                                         // C9
            None,                                                         // C10
            None,                                                         // C11
            None,                                                         // C12
            None,                                                         // C13
            None,                                                         // C14
            None,                                                         // C15
            None,                                                         // C16
            None,                                                         // C16DC9
            None,                                                         // C16DT9
            None,                                                         // C17
            None,                                                         // C18
            None,                                                         // C18DC9
            None,                                                         // C18DT9
            Some(FattyAcidChunked::try_from(C18DC9DC12)?),                // C18DC9DC12
            Some(FattyAcidChunked::try_from(C18DC6DC9DC12)?),             // C18DC6DC9DC12
            Some(FattyAcidChunked::try_from(C18DC8DT10DC12)?),            // C18DC8DT10DC12
            Some(FattyAcidChunked::try_from(C18DC9DC12DC15)?),            // C18DC9DC12DC15
            Some(FattyAcidChunked::try_from(C18DC9DT11DT13)?),            // C18DC9DT11DT13
            Some(FattyAcidChunked::try_from(C18DT9DT11DC13)?),            // C18DT9DT11DC13
            Some(FattyAcidChunked::try_from(C18DT9DT11DT13)?),            // C18DT9DT11DT13
            Some(FattyAcidChunked::try_from(C18DC6DC9DC12DC15)?),         // C18DC6DC9DC12DC15
            None,                                                         // C19
            None,                                                         // C20
            None,                                                         // C20DC9
            None,                                                         // C20DC11
            Some(FattyAcidChunked::try_from(C20DC11DC14)?),               // C20DC11DC14
            Some(FattyAcidChunked::try_from(C20DC5DC8DC11)?),             // C20DC5DC8DC11
            Some(FattyAcidChunked::try_from(C20DC8DC11DC14)?),            // C20DC8DC11DC14
            Some(FattyAcidChunked::try_from(C20DC11DC14DC17)?),           // C20DC11DC14DC17
            Some(FattyAcidChunked::try_from(C20DC5DC8DC11DC14)?),         // C20DC5DC8DC11DC14
            Some(FattyAcidChunked::try_from(C20DC8DC11DC14DC17)?),        // C20DC8DC11DC14DC17
            Some(FattyAcidChunked::try_from(C20DC5DC8DC11DC14DC17)?),     // C20DC5DC8DC11DC14DC17
            None,                                                         // C21
            None,                                                         // C22
            None,                                                         // C22DC13
            Some(FattyAcidChunked::try_from(C22DC13DC16)?),               // C22DC13DC16
            Some(FattyAcidChunked::try_from(C22DC5DC13DC16)?),            // C22DC5DC13DC16
            Some(FattyAcidChunked::try_from(C22DC7DC10DC13DC16)?),        // C22DC7DC10DC13DC16
            Some(FattyAcidChunked::try_from(C22DC7DC10DC13DC16DC19)?),    // C22DC7DC10DC13DC16DC19
            Some(FattyAcidChunked::try_from(C22DC4DC7DC10DC13DC16DC19)?), // C22DC4DC7DC10DC13DC16DC19
            None,                                                         // C23
            None,                                                         // C24
            None,                                                         // C24DC15
            Some(FattyAcidChunked::try_from(C24DC15DC18)?),               // C24DC15DC18
            Some(FattyAcidChunked::try_from(C24DC12DC15DC18)?),           // C24DC12DC15DC18
            Some(FattyAcidChunked::try_from(C24DC9DC12DC15DC18)?),        // C24DC9DC12DC15DC18
            Some(FattyAcidChunked::try_from(C24DC6DC9DC12DC15DC18)?),     // C24DC6DC9DC12DC15DC18
            Some(FattyAcidChunked::try_from(C24DC6DC9DC12DC15DC18DC21)?), // C24DC6DC9DC12DC15DC18DC21
            None,                                                         // C25
            None,                                                         // C26
            None,                                                         // C26DC17
            None,                                                         // C27
            None,                                                         // C28
            None,                                                         // C29
            None,                                                         // C30
            None,                                                         // C30DC21
            None,                                                         // C31
            None,                                                         // C32
            None,                                                         // C33
            None,                                                         // C34
            None,                                                         // C35
            None,                                                         // C36
        ],
    );
    Ok(())
}
