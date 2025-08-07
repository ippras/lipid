use std::fmt::{Display, Formatter, Result, Write as _};

/// Display common options
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Options {
    pub bounds: Elision,
    pub isomerism: Elision,
}

/// Elision
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub enum Elision {
    Explicit,
    #[default]
    Implicit,
}

/// Fatty acid
#[derive(Clone, Debug, Default)]
pub struct FattyAcid {
    pub carbon: u8,
    pub unsaturated: Vec<Unsaturated>,
}

impl FattyAcid {
    pub fn new(carbon: u8, unsaturated: Vec<Unsaturated>) -> Self {
        Self {
            carbon,
            unsaturated,
        }
    }

    pub fn delta(&self) -> Delta<&Self> {
        Delta(self)
    }

    pub fn id(&self) -> Id<&Self> {
        Id(self)
    }
}

// impl Display for FattyAcid {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         Display::fmt(&self.carbon, f)?;
//         f.write_char(':')?;
//         Display::fmt(&self.unsaturated.len(), f)?;
//         let mut iter = self.unsaturated.iter();
//         if let Some(unsaturated) = iter.next() {
//             f.write_char('Δ')?;
//             Display::fmt(unsaturated, f)?;
//             for unsaturated in iter {
//                 f.write_char(',')?;
//                 Display::fmt(unsaturated, f)?;
//             }
//         }
//         Ok(())
//     }
// }

/// Unsaturated bound
#[derive(Clone, Copy, Debug, Default)]
pub struct Unsaturated {
    pub index: Option<u8>,
    pub parity: Option<bool>,
    pub triple: Option<bool>,
}

impl Unsaturated {
    pub fn new(index: Option<u8>, parity: Option<bool>, triple: Option<bool>) -> Self {
        Self {
            index,
            parity,
            triple,
        }
    }
}

// impl Display for Unsaturated {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         match self.index {
//             None => f.write_char('*')?,
//             Some(index) => Display::fmt(&index, f)?,
//         }
//         match self.triple {
//             None => f.write_char('u')?,
//             Some(false) => match self.parity {
//                 None => f.write_char('o')?, // Olefinic
//                 Some(false) => f.write_char('c')?,
//                 Some(true) => f.write_char('t')?,
//             },
//             Some(true) => f.write_char('a')?, // Acetylenic
//         }
//         Ok(())
//     }
// }

/// Delta
#[derive(Clone, Debug, Default)]
pub struct Delta<T>(T);

impl Display for Delta<&FattyAcid> {
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

impl Display for Delta<&Unsaturated> {
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

impl Display for Id<&FattyAcid> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.write_char('c')?;
        Display::fmt(&self.0.carbon, f)?;
        for unsaturated in &self.0.unsaturated {
            Display::fmt(&Id(unsaturated), f)?;
        }
        Ok(())
    }
}

impl Display for Id<&Unsaturated> {
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
