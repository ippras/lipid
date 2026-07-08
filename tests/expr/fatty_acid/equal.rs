use super::*;

macro_rules! check {
    ($identifier:ident, $expected:expr) => {{
        let data_frame = fatty_acids()?
            .lazy()
            .select([col(FATTY_ACID).fatty_acid().equal($identifier.clone())])
            .collect()?;
        let is_equal: Vec<_> = data_frame[FATTY_ACID].bool()?.into_iter().collect();
        assert_eq!(is_equal, $expected);
    }};
}

#[test]
fn c4u0() -> PolarsResult<()> {
    check!(
        C4U0,
        [
            Some(true),  // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c5u0() -> PolarsResult<()> {
    check!(
        C5U0,
        [
            Some(false), // C4
            Some(true),  // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c6u0() -> PolarsResult<()> {
    check!(
        C6U0,
        [
            Some(false), // C4
            Some(false), // C5
            Some(true),  // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c7u0() -> PolarsResult<()> {
    check!(
        C7U0,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(true),  // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c8u0() -> PolarsResult<()> {
    check!(
        C8U0,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(true),  // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c9u0() -> PolarsResult<()> {
    check!(
        C9U0,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(true),  // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c10u0() -> PolarsResult<()> {
    check!(
        C10U0,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(true),  // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c11u0() -> PolarsResult<()> {
    check!(
        C11U0,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(true),  // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c12u0() -> PolarsResult<()> {
    check!(
        C12U0,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(true),  // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c13u0() -> PolarsResult<()> {
    check!(
        C13U0,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(true),  // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c14u0() -> PolarsResult<()> {
    check!(
        C14U0,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(true),  // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c15u0() -> PolarsResult<()> {
    check!(
        C15U0,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(true),  // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c16u0() -> PolarsResult<()> {
    check!(
        C16U0,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(true),  // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c16u1dc9() -> PolarsResult<()> {
    check!(
        C16U1C9,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(true),  // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c16u1dt9() -> PolarsResult<()> {
    check!(
        C16U1T9,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(true),  // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c17u0() -> PolarsResult<()> {
    check!(
        C17U0,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(true),  // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c18u0() -> PolarsResult<()> {
    check!(
        C18U0,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(true),  // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c18u1dc9() -> PolarsResult<()> {
    check!(
        C18U1C9,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(true),  // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c18u1dt9() -> PolarsResult<()> {
    check!(
        C18U1T9,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(true),  // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c18u2dc9dc12() -> PolarsResult<()> {
    check!(
        C18U2C9C12,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(true),  // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c18u3dc6dc9dc12() -> PolarsResult<()> {
    check!(
        C18U3C6C9C12,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(true),  // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c18u3dc8dt10dc12() -> PolarsResult<()> {
    check!(
        C18U3C8T10C12,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(true),  // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}
#[test]
fn c18u3dc9dc12dc15() -> PolarsResult<()> {
    check!(
        C18U3C9C12C15,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(true),  // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c18u3dc9dt11dt13() -> PolarsResult<()> {
    check!(
        C18U3C9T11T13,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(true),  // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c18u3dt9dt11dc13() -> PolarsResult<()> {
    check!(
        C18U3T9T11C13,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(true),  // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c18u3dt9dt11dt13() -> PolarsResult<()> {
    check!(
        C18U3T9T11T13,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(true),  // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c18u4dc6dc9dc12dc15() -> PolarsResult<()> {
    check!(
        C18U4C6C9C12C15,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(true),  // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c19u0() -> PolarsResult<()> {
    check!(
        C19U0,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(true),  // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c20u0() -> PolarsResult<()> {
    check!(
        C20U0,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(true),  // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c20u1dc9() -> PolarsResult<()> {
    check!(
        C20U1C9,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(true),  // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c20u1dc11() -> PolarsResult<()> {
    check!(
        C20U1C11,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(true),  // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c20u2dc11dc14() -> PolarsResult<()> {
    check!(
        C20U2C11C14,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(true),  // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c20u3dc5dc8dc11() -> PolarsResult<()> {
    check!(
        C20U3C5C8C11,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(true),  // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c20u3dc8dc11dc14() -> PolarsResult<()> {
    check!(
        C20U3C8C11C14,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(true),  // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c20u3dc11dc14dc17() -> PolarsResult<()> {
    check!(
        C20U3C11C14C17,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(true),  // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c20u4dc5dc8dc11dc14() -> PolarsResult<()> {
    check!(
        C20U4C5C8C11C14,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(true),  // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c20u4dc8dc11dc14dc17() -> PolarsResult<()> {
    check!(
        C20U4C8C11C14C17,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(true),  // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c20u5dc5dc8dc11dc14dc17() -> PolarsResult<()> {
    check!(
        C20U5C5C8C11C14C17,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(true),  // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c21u0() -> PolarsResult<()> {
    check!(
        C21U0,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(true),  // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c22u0() -> PolarsResult<()> {
    check!(
        C22U0,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(true),  // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c22u1dc13() -> PolarsResult<()> {
    check!(
        C22U1C13,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(true),  // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c22u2dc13dc16() -> PolarsResult<()> {
    check!(
        C22U2C13C16,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(true),  // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c22u3dc5dc13dc16() -> PolarsResult<()> {
    check!(
        C22U3C5C13C16,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(true),  // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c22u4dc7dc10dc13dc16() -> PolarsResult<()> {
    check!(
        C22U4C7C10C13C16,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(true),  // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c22u5dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check!(
        C22U5C7C10C13C16C19,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(true),  // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c22u6dc4dc7dc10dc13dc16dc19() -> PolarsResult<()> {
    check!(
        C22U6C4C7C10C13C16C19,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(true),  // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c23u0() -> PolarsResult<()> {
    check!(
        C23U0,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(true),  // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c24u0() -> PolarsResult<()> {
    check!(
        C24U0,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(true),  // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c24u1dc15() -> PolarsResult<()> {
    check!(
        C24U1C15,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(true),  // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c24u2dc15dc18() -> PolarsResult<()> {
    check!(
        C24U2C15C18,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(true),  // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c24u3dc12dc15dc18() -> PolarsResult<()> {
    check!(
        C24U3C12C15C18,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(true),  // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c24u4dc9dc12dc15dc18() -> PolarsResult<()> {
    check!(
        C24U4C9C12C15C18,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(true),  // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c24u5dc6dc9dc12dc15dc18() -> PolarsResult<()> {
    check!(
        C24U5C6C9C12C15C18,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(true),  // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c24u6dc6dc9dc12dc15dc18dc21() -> PolarsResult<()> {
    check!(
        C24U6C6C9C12C15C18C21,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(true),  // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c25u0() -> PolarsResult<()> {
    check!(
        C25U0,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(true),  // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c26u0() -> PolarsResult<()> {
    check!(
        C26U0,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(true),  // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c26u1dc17() -> PolarsResult<()> {
    check!(
        C26U1C17,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(true),  // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c27u0() -> PolarsResult<()> {
    check!(
        C27U0,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(true),  // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c28u0() -> PolarsResult<()> {
    check!(
        C28U0,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(true),  // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c29u0() -> PolarsResult<()> {
    check!(
        C29U0,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(true),  // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c30u0() -> PolarsResult<()> {
    check!(
        C30U0,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(true),  // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c30u1dc21() -> PolarsResult<()> {
    check!(
        C30U1C21,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(true),  // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c31u0() -> PolarsResult<()> {
    check!(
        C31U0,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(true),  // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c32u0() -> PolarsResult<()> {
    check!(
        C32U0,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(true),  // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c33u0() -> PolarsResult<()> {
    check!(
        C33U0,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(true),  // C33
            Some(false), // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c34u0() -> PolarsResult<()> {
    check!(
        C34U0,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(true),  // C34
            Some(false), // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c35u0() -> PolarsResult<()> {
    check!(
        C35U0,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(true),  // C35
            Some(false), // C36
        ]
    );
    Ok(())
}

#[test]
fn c36u0() -> PolarsResult<()> {
    check!(
        C36U0,
        [
            Some(false), // C4
            Some(false), // C5
            Some(false), // C6
            Some(false), // C7
            Some(false), // C8
            Some(false), // C9
            Some(false), // C10
            Some(false), // C11
            Some(false), // C12
            Some(false), // C13
            Some(false), // C14
            Some(false), // C15
            Some(false), // C16
            Some(false), // C16C9
            Some(false), // C16T9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18C9
            Some(false), // C18T9
            Some(false), // C18C9C12
            Some(false), // C18C6C9C12
            Some(false), // C18C8T10C12
            Some(false), // C18C9C12C15
            Some(false), // C18C9T11T13
            Some(false), // C18T9T11C13
            Some(false), // C18T9T11T13
            Some(false), // C18C6C9C12C15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20C9
            Some(false), // C20C11
            Some(false), // C20C11C14
            Some(false), // C20C5C8C11
            Some(false), // C20C8C11C14
            Some(false), // C20C11C14C17
            Some(false), // C20C5C8C11C14
            Some(false), // C20C8C11C14C17
            Some(false), // C20C5C8C11C14C17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22C13
            Some(false), // C22C13C16
            Some(false), // C22C5C13C16
            Some(false), // C22C7C10C13C16
            Some(false), // C22C7C10C13C16C19
            Some(false), // C22C4C7C10C13C16C19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24C15
            Some(false), // C24C15C18
            Some(false), // C24C12C15C18
            Some(false), // C24C9C12C15C18
            Some(false), // C24C6C9C12C15C18
            Some(false), // C24C6C9C12C15C18C21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26C17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30C21
            Some(false), // C31
            Some(false), // C32
            Some(false), // C33
            Some(false), // C34
            Some(false), // C35
            Some(true),  // C36
        ]
    );
    Ok(())
}
