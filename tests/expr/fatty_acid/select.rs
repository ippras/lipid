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
            None,                                   // C16C9
            None,                                   // C16T9
            Some(FattyAcidChunked::try_from(C17)?), // C17
            Some(FattyAcidChunked::try_from(C18)?), // C18
            None,                                   // C18C9
            None,                                   // C18T9
            None,                                   // C18C9C12
            None,                                   // C18C6C9C12
            None,                                   // C18C8T10C12
            None,                                   // C18C9C12C15
            None,                                   // C18C9T11T13
            None,                                   // C18T9T11C13
            None,                                   // C18T9T11T13
            None,                                   // C18C6C9C12C15
            Some(FattyAcidChunked::try_from(C19)?), // C19
            Some(FattyAcidChunked::try_from(C20)?), // C20
            None,                                   // C20C9
            None,                                   // C20C11
            None,                                   // C20C11C14
            None,                                   // C20C5C8C11
            None,                                   // C20C8C11C14
            None,                                   // C20C11C14C17
            None,                                   // C20C5C8C11C14
            None,                                   // C20C8C11C14C17
            None,                                   // C20C5C8C11C14C17
            Some(FattyAcidChunked::try_from(C21)?), // C21
            Some(FattyAcidChunked::try_from(C22)?), // C22
            None,                                   // C22C13
            None,                                   // C22C13C16
            None,                                   // C22C5C13C16
            None,                                   // C22C7C10C13C16
            None,                                   // C22C7C10C13C16C19
            None,                                   // C22C4C7C10C13C16C19
            Some(FattyAcidChunked::try_from(C23)?), // C23
            Some(FattyAcidChunked::try_from(C24)?), // C24
            None,                                   // C24C15
            None,                                   // C24C15C18
            None,                                   // C24C12C15C18
            None,                                   // C24C9C12C15C18
            None,                                   // C24C6C9C12C15C18
            None,                                   // C24C6C9C12C15C18C21
            Some(FattyAcidChunked::try_from(C25)?), // C25
            Some(FattyAcidChunked::try_from(C26)?), // C26
            None,                                   // C26C17
            Some(FattyAcidChunked::try_from(C27)?), // C27
            Some(FattyAcidChunked::try_from(C28)?), // C28
            Some(FattyAcidChunked::try_from(C29)?), // C29
            Some(FattyAcidChunked::try_from(C30)?), // C30
            None,                                   // C30C21
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
            Some(FattyAcidChunked::try_from(C16C9)?),                    // C16C9
            Some(FattyAcidChunked::try_from(C16T9)?),                    // C16T9
            None,                                                         // C17
            None,                                                         // C18
            Some(FattyAcidChunked::try_from(C18C9)?),                    // C18C9
            Some(FattyAcidChunked::try_from(C18T9)?),                    // C18T9
            Some(FattyAcidChunked::try_from(C18C9C12)?),                // C18C9C12
            Some(FattyAcidChunked::try_from(C18C6C9C12)?),             // C18C6C9C12
            Some(FattyAcidChunked::try_from(C18C8T10C12)?),            // C18C8T10C12
            Some(FattyAcidChunked::try_from(C18C9C12C15)?),            // C18C9C12C15
            Some(FattyAcidChunked::try_from(C18C9T11T13)?),            // C18C9T11T13
            Some(FattyAcidChunked::try_from(C18T9T11C13)?),            // C18T9T11C13
            Some(FattyAcidChunked::try_from(C18T9T11T13)?),            // C18T9T11T13
            Some(FattyAcidChunked::try_from(C18C6C9C12C15)?),         // C18C6C9C12C15
            None,                                                         // C19
            None,                                                         // C20
            Some(FattyAcidChunked::try_from(C20C9)?),                    // C20C9
            Some(FattyAcidChunked::try_from(C20C11)?),                   // C20C11
            Some(FattyAcidChunked::try_from(C20C11C14)?),               // C20C11C14
            Some(FattyAcidChunked::try_from(C20C5C8C11)?),             // C20C5C8C11
            Some(FattyAcidChunked::try_from(C20C8C11C14)?),            // C20C8C11C14
            Some(FattyAcidChunked::try_from(C20C11C14C17)?),           // C20C11C14C17
            Some(FattyAcidChunked::try_from(C20C5C8C11C14)?),         // C20C5C8C11C14
            Some(FattyAcidChunked::try_from(C20C8C11C14C17)?),        // C20C8C11C14C17
            Some(FattyAcidChunked::try_from(C20C5C8C11C14C17)?),     // C20C5C8C11C14C17
            None,                                                         // C21
            None,                                                         // C22
            Some(FattyAcidChunked::try_from(C22C13)?),                   // C22C13
            Some(FattyAcidChunked::try_from(C22C13C16)?),               // C22C13C16
            Some(FattyAcidChunked::try_from(C22C5C13C16)?),            // C22C5C13C16
            Some(FattyAcidChunked::try_from(C22C7C10C13C16)?),        // C22C7C10C13C16
            Some(FattyAcidChunked::try_from(C22C7C10C13C16C19)?),    // C22C7C10C13C16C19
            Some(FattyAcidChunked::try_from(C22C4C7C10C13C16C19)?), // C22C4C7C10C13C16C19
            None,                                                         // C23
            None,                                                         // C24
            Some(FattyAcidChunked::try_from(C24C15)?),                   // C24C15
            Some(FattyAcidChunked::try_from(C24C15C18)?),               // C24C15C18
            Some(FattyAcidChunked::try_from(C24C12C15C18)?),           // C24C12C15C18
            Some(FattyAcidChunked::try_from(C24C9C12C15C18)?),        // C24C9C12C15C18
            Some(FattyAcidChunked::try_from(C24C6C9C12C15C18)?),     // C24C6C9C12C15C18
            Some(FattyAcidChunked::try_from(C24C6C9C12C15C18C21)?), // C24C6C9C12C15C18C21
            None,                                                         // C25
            None,                                                         // C26
            Some(FattyAcidChunked::try_from(C26C17)?),                   // C26C17
            None,                                                         // C27
            None,                                                         // C28
            None,                                                         // C29
            None,                                                         // C30
            Some(FattyAcidChunked::try_from(C30C21)?),                   // C30C21
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
            Some(FattyAcidChunked::try_from(C16C9)?),  // C16C9
            Some(FattyAcidChunked::try_from(C16T9)?),  // C16T9
            None,                                       // C17
            None,                                       // C18
            Some(FattyAcidChunked::try_from(C18C9)?),  // C18C9
            Some(FattyAcidChunked::try_from(C18T9)?),  // C18T9
            None,                                       // C18C9C12
            None,                                       // C18C6C9C12
            None,                                       // C18C8T10C12
            None,                                       // C18C9C12C15
            None,                                       // C18C9T11T13
            None,                                       // C18T9T11C13
            None,                                       // C18T9T11T13
            None,                                       // C18C6C9C12C15
            None,                                       // C19
            None,                                       // C20
            Some(FattyAcidChunked::try_from(C20C9)?),  // C20C9
            Some(FattyAcidChunked::try_from(C20C11)?), // C20C11
            None,                                       // C20C11C14
            None,                                       // C20C5C8C11
            None,                                       // C20C8C11C14
            None,                                       // C20C11C14C17
            None,                                       // C20C5C8C11C14
            None,                                       // C20C8C11C14C17
            None,                                       // C20C5C8C11C14C17
            None,                                       // C21
            None,                                       // C22
            Some(FattyAcidChunked::try_from(C22C13)?), // C22C13
            None,                                       // C22C13C16
            None,                                       // C22C5C13C16
            None,                                       // C22C7C10C13C16
            None,                                       // C22C7C10C13C16C19
            None,                                       // C22C4C7C10C13C16C19
            None,                                       // C23
            None,                                       // C24
            Some(FattyAcidChunked::try_from(C24C15)?), // C24C15
            None,                                       // C24C15C18
            None,                                       // C24C12C15C18
            None,                                       // C24C9C12C15C18
            None,                                       // C24C6C9C12C15C18
            None,                                       // C24C6C9C12C15C18C21
            None,                                       // C25
            None,                                       // C26
            Some(FattyAcidChunked::try_from(C26C17)?), // C26C17
            None,                                       // C27
            None,                                       // C28
            None,                                       // C29
            None,                                       // C30
            Some(FattyAcidChunked::try_from(C30C21)?), // C30C21
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
            None,                                                         // C16C9
            None,                                                         // C16T9
            None,                                                         // C17
            None,                                                         // C18
            None,                                                         // C18C9
            None,                                                         // C18T9
            Some(FattyAcidChunked::try_from(C18C9C12)?),                // C18C9C12
            Some(FattyAcidChunked::try_from(C18C6C9C12)?),             // C18C6C9C12
            Some(FattyAcidChunked::try_from(C18C8T10C12)?),            // C18C8T10C12
            Some(FattyAcidChunked::try_from(C18C9C12C15)?),            // C18C9C12C15
            Some(FattyAcidChunked::try_from(C18C9T11T13)?),            // C18C9T11T13
            Some(FattyAcidChunked::try_from(C18T9T11C13)?),            // C18T9T11C13
            Some(FattyAcidChunked::try_from(C18T9T11T13)?),            // C18T9T11T13
            Some(FattyAcidChunked::try_from(C18C6C9C12C15)?),         // C18C6C9C12C15
            None,                                                         // C19
            None,                                                         // C20
            None,                                                         // C20C9
            None,                                                         // C20C11
            Some(FattyAcidChunked::try_from(C20C11C14)?),               // C20C11C14
            Some(FattyAcidChunked::try_from(C20C5C8C11)?),             // C20C5C8C11
            Some(FattyAcidChunked::try_from(C20C8C11C14)?),            // C20C8C11C14
            Some(FattyAcidChunked::try_from(C20C11C14C17)?),           // C20C11C14C17
            Some(FattyAcidChunked::try_from(C20C5C8C11C14)?),         // C20C5C8C11C14
            Some(FattyAcidChunked::try_from(C20C8C11C14C17)?),        // C20C8C11C14C17
            Some(FattyAcidChunked::try_from(C20C5C8C11C14C17)?),     // C20C5C8C11C14C17
            None,                                                         // C21
            None,                                                         // C22
            None,                                                         // C22C13
            Some(FattyAcidChunked::try_from(C22C13C16)?),               // C22C13C16
            Some(FattyAcidChunked::try_from(C22C5C13C16)?),            // C22C5C13C16
            Some(FattyAcidChunked::try_from(C22C7C10C13C16)?),        // C22C7C10C13C16
            Some(FattyAcidChunked::try_from(C22C7C10C13C16C19)?),    // C22C7C10C13C16C19
            Some(FattyAcidChunked::try_from(C22C4C7C10C13C16C19)?), // C22C4C7C10C13C16C19
            None,                                                         // C23
            None,                                                         // C24
            None,                                                         // C24C15
            Some(FattyAcidChunked::try_from(C24C15C18)?),               // C24C15C18
            Some(FattyAcidChunked::try_from(C24C12C15C18)?),           // C24C12C15C18
            Some(FattyAcidChunked::try_from(C24C9C12C15C18)?),        // C24C9C12C15C18
            Some(FattyAcidChunked::try_from(C24C6C9C12C15C18)?),     // C24C6C9C12C15C18
            Some(FattyAcidChunked::try_from(C24C6C9C12C15C18C21)?), // C24C6C9C12C15C18C21
            None,                                                         // C25
            None,                                                         // C26
            None,                                                         // C26C17
            None,                                                         // C27
            None,                                                         // C28
            None,                                                         // C29
            None,                                                         // C30
            None,                                                         // C30C21
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
