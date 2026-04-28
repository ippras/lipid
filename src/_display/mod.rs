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
