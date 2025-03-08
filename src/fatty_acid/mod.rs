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

pub mod r#const;
