use crate::r#struct::unsaturated::Unsaturated;
use std::fmt::{Display, Formatter, Result, Write as _};

/// Fatty acid
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FattyAcid<T, U> {
    pub carbon: T,
    pub unsaturated: U,
}

impl FattyAcid<u8, Vec<Unsaturated<Option<u8>, Option<bool>, Option<bool>>>> {
    pub fn delta(&self) -> Delta<&Self> {
        Delta(self)
    }

    pub fn id(&self) -> Id<&Self> {
        Id(self)
    }
}

/// Delta
#[derive(Clone, Debug, Default)]
pub struct Delta<T>(pub(super) T);

impl Display for Delta<&FattyAcid<u8, Vec<Unsaturated<Option<u8>, Option<bool>, Option<bool>>>>> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(&self.0.carbon, f)?;
        f.write_char(':')?;
        Display::fmt(&self.0.unsaturated.len(), f)?;
        let mut iter = self.0.unsaturated.iter();
        if let Some(unsaturated) = iter.next() {
            f.write_char('Δ')?;
            Display::fmt(&Delta(unsaturated), f)?;
            for unsaturated in iter {
                f.write_char(',')?;
                Display::fmt(&Delta(unsaturated), f)?;
            }
        }
        Ok(())
    }
}

impl Display for Delta<&Unsaturated<Option<u8>, Option<bool>, Option<bool>>> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.0.index {
            None => f.write_char('0')?,
            Some(index) => Display::fmt(&index, f)?,
        }
        match self.0.triple {
            None => f.write_char('u')?, // Unsaturated
            Some(false) => match self.0.parity {
                None => f.write_char('o')?,        // Olefinic
                Some(false) => f.write_char('c')?, // Cis
                Some(true) => f.write_char('t')?,  // Trans
            },
            Some(true) => f.write_char('a')?, // Acetylenic
        }
        Ok(())
    }
}

/// Id
#[derive(Clone, Debug, Default)]
pub struct Id<T>(T);

impl Display for Id<&FattyAcid<u8, Vec<Unsaturated<Option<u8>, Option<bool>, Option<bool>>>>> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.write_char('c')?;
        Display::fmt(&self.0.carbon, f)?;
        for unsaturated in &self.0.unsaturated {
            Display::fmt(&Id(unsaturated), f)?;
        }
        Ok(())
    }
}

impl Display for Id<&Unsaturated<Option<u8>, Option<bool>, Option<bool>>> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.0.triple {
            None => f.write_char('u')?, // Unsaturated
            Some(false) => match self.0.parity {
                None => f.write_char('o')?,        // Olefinic
                Some(false) => f.write_char('c')?, // Cis
                Some(true) => f.write_char('t')?,  // Trans
            },
            Some(true) => f.write_char('a')?, // Acetylenic
        }
        match self.0.index {
            None => f.write_char('0')?,
            Some(index) => Display::fmt(&index, f)?,
        }
        Ok(())
    }
}
