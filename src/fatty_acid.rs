use std::{array::IntoIter, ops::Deref};

/// Fatty acid
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct FattyAcid<'a, const N: usize>(pub(crate) [&'a str; N]);

impl<'a, const N: usize> AsRef<[&'a str]> for FattyAcid<'a, N> {
    fn as_ref(&self) -> &[&'a str] {
        &self.0
    }
}

impl<'a, const N: usize> Deref for FattyAcid<'a, N> {
    type Target = [&'a str; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, const N: usize> IntoIterator for FattyAcid<'a, N> {
    type Item = &'a str;

    type IntoIter = IntoIter<Self::Item, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// Represents the kind of fatty acid.
#[derive(Clone, Copy, Debug, Default)]
pub enum Kind {
    /// Fatty acid.
    ///
    /// `RCOOH`
    #[default]
    Rcooh,
    /// Fatty acid methyl ester.
    ///
    /// `RCOOH -H +CH3 => RCOOCH3`
    Rcooch3,
    /// Fatty acid anion \[RCOO\]-.
    ///
    /// `RCOOH -H => [RCOO]-`
    Rcoo,
    /// Fatty acid cation \[RCO\]+.
    ///
    /// `RCOOH -OH => [RCO]+`
    Rco,
}

/// Represents a fatty acid cation \[RCO\]+.
///
/// `RCOOH -OH => [RCO]+`
pub struct Rco<T>(pub T);

/// Represents a fatty acid anion \[RCOO\]-.
///
/// `RCOOH -H => [RCOO]-`
pub struct Rcoo<T>(pub T);

/// Represents a fatty acid.
///
/// `RCOOH`
pub struct Rcooh<T>(pub T);

/// Represents a fatty acid methyl ester.
///
/// `RCOOH -H +CH3 => RCOOCH3`
pub struct Rcooch3<T>(pub T);

// #[test]
// fn empty() {
//     assert_eq!(&*FattyAcid([]), &[]);
// }

#[test]
fn one() {
    assert_eq!(&*FattyAcid(["S"]), &["S"]);
}

#[test]
fn two_doubles() {
    assert_eq!(&*FattyAcid(["S", "DC", "DC", "S"]), &["S", "DC", "DC", "S"]);
    assert_eq!(&*FattyAcid(["S", "DT", "DT", "S"]), &["S", "DT", "DT", "S"]);
    assert_eq!(&*FattyAcid(["S", "D", "D", "S"]), &["S", "D", "D", "S"]);
}

#[test]
fn two_triples() {
    assert_eq!(&*FattyAcid(["S", "TC", "TC", "S"]), &["S", "TC", "TC", "S"]);
    assert_eq!(&*FattyAcid(["S", "TT", "TT", "S"]), &["S", "TT", "TT", "S"]);
    assert_eq!(&*FattyAcid(["S", "T", "T", "S"]), &["S", "T", "T", "S"]);
}
