use super::*;

macro_rules! check {
    ($identifier:ident, $expected:expr) => {{
        let data_frame = fatty_acids()?
            .lazy()
            .select([col(FATTY_ACID).fatty_acid().equal($identifier)])
            .collect()?;
        let got: Vec<_> = data_frame[FATTY_ACID].bool()?.into_iter().collect();
        assert_eq!(got, $expected);
    }};
}

#[test]
fn c4u0() -> PolarsResult<()> {
    check!(
        C4,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C5,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C6,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C7,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C8,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C9,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C10,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C11,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C12,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C13,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C14,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C15,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C16,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C16DC9,
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
            Some(true),  // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C16DT9,
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
            Some(false), // C16DC9
            Some(true),  // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C17,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(true),  // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C18,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(true),  // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C18DC9,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(true),  // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C18DT9,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(true),  // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C18DC9DC12,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(true),  // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C18DC6DC9DC12,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(true),  // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C18DC8DT10DC12,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(true),  // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C18DC9DC12DC15,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(true),  // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C18DC9DT11DT13,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(true),  // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C18DT9DT11DC13,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(true),  // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C18DT9DT11DT13,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(true),  // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C18DC6DC9DC12DC15,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(true),  // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C19,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(true),  // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C20,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(true),  // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C20DC9,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(true),  // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C20DC11,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(true),  // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C20DC11DC14,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(true),  // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C20DC5DC8DC11,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(true),  // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C20DC8DC11DC14,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(true),  // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C20DC11DC14DC17,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(true),  // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C20DC5DC8DC11DC14,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(true),  // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C20DC8DC11DC14DC17,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(true),  // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C20DC5DC8DC11DC14DC17,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(true),  // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C21,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(true),  // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C22,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(true),  // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C22DC13,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(true),  // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C22DC13DC16,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(true),  // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C22DC5DC13DC16,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(true),  // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C22DC7DC10DC13DC16,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(true),  // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C22DC7DC10DC13DC16DC19,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(true),  // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C22DC4DC7DC10DC13DC16DC19,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(true),  // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C23,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(true),  // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C24,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(true),  // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C24DC15,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(true),  // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C24DC15DC18,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(true),  // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C24DC12DC15DC18,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(true),  // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C24DC9DC12DC15DC18,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(true),  // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C24DC6DC9DC12DC15DC18,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(true),  // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C24DC6DC9DC12DC15DC18DC21,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(true),  // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C25,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(true),  // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C26,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(true),  // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C26DC17,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(true),  // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C27,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(true),  // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C28,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(true),  // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C29,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(true),  // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C30,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(true),  // C30
            Some(false), // C30DC21
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
        C30DC21,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(true),  // C30DC21
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
        C31,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C32,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C33,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C34,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C35,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
        C36,
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
            Some(false), // C16DC9
            Some(false), // C16DT9
            Some(false), // C17
            Some(false), // C18
            Some(false), // C18DC9
            Some(false), // C18DT9
            Some(false), // C18DC9DC12
            Some(false), // C18DC6DC9DC12
            Some(false), // C18DC8DT10DC12
            Some(false), // C18DC9DC12DC15
            Some(false), // C18DC9DT11DT13
            Some(false), // C18DT9DT11DC13
            Some(false), // C18DT9DT11DT13
            Some(false), // C18DC6DC9DC12DC15
            Some(false), // C19
            Some(false), // C20
            Some(false), // C20DC9
            Some(false), // C20DC11
            Some(false), // C20DC11DC14
            Some(false), // C20DC5DC8DC11
            Some(false), // C20DC8DC11DC14
            Some(false), // C20DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14
            Some(false), // C20DC8DC11DC14DC17
            Some(false), // C20DC5DC8DC11DC14DC17
            Some(false), // C21
            Some(false), // C22
            Some(false), // C22DC13
            Some(false), // C22DC13DC16
            Some(false), // C22DC5DC13DC16
            Some(false), // C22DC7DC10DC13DC16
            Some(false), // C22DC7DC10DC13DC16DC19
            Some(false), // C22DC4DC7DC10DC13DC16DC19
            Some(false), // C23
            Some(false), // C24
            Some(false), // C24DC15
            Some(false), // C24DC15DC18
            Some(false), // C24DC12DC15DC18
            Some(false), // C24DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18
            Some(false), // C24DC6DC9DC12DC15DC18DC21
            Some(false), // C25
            Some(false), // C26
            Some(false), // C26DC17
            Some(false), // C27
            Some(false), // C28
            Some(false), // C29
            Some(false), // C30
            Some(false), // C30DC21
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
