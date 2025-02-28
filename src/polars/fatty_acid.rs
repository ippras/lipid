/// Fatty acid kind
#[derive(Clone, Copy, Debug, Default)]
pub enum Kind {
    /// Fatty acid
    ///
    /// `RCOOH`
    #[default]
    Rcooh,
    /// Fatty acid methyl ester
    ///
    /// `RCOOH -H +CH3 => RCOOCH3`
    Rcooch3,
    /// Fatty acid [RCOO]-
    ///
    /// `RCOOH -H => [RCOO]-`
    Rcoo,
    /// Fatty acid [RCO]+
    ///
    /// `RCOOH -OH => [RCO]+`
    Rco,
}

/// Fatty acid [RCO]+
///
/// `RCOOH -OH => [RCO]+`
pub struct Rco<T>(pub T);

/// Fatty acid [RCOO]-
///
/// `RCOOH -H => [RCOO]-`
pub struct Rcoo<T>(pub T);

/// Fatty acid
///
/// `RCOOH`
pub struct Rcooh<T>(pub T);

/// Fatty acid methyl ester
///
/// `RCOOH -H +CH3 => RCOOCH3`
pub struct Rcooch3<T>(pub T);
