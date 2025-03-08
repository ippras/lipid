use super::FattyAcidExpr;
use crate::prelude::*;

impl FattyAcidExpr {
    /// Converts the fatty acid expression to a fatty acid cation \[RCO\]+.
    ///
    /// # Returns
    ///
    /// An instance of [`Rco`] containing the fatty acid expression.
    pub fn rco(self) -> Rco<Self> {
        Rco(self)
    }

    /// Converts the fatty acid expression to a fatty acid anion \[RCOO\]-.
    ///
    /// # Returns
    ///
    /// An instance of [`Rcoo`] containing the fatty acid expression.
    pub fn rcoo(self) -> Rcoo<Self> {
        Rcoo(self)
    }

    /// Converts the fatty acid expression to a fatty acid \[RCOOH\].
    ///
    /// # Returns
    ///
    /// An instance of [`Rcooh`] containing the fatty acid expression.
    pub fn rcooh(self) -> Rcooh<Self> {
        Rcooh(self)
    }

    /// Converts the fatty acid expression to a fatty acid methyl ester
    /// \[RCOOCH3\].
    ///
    /// # Returns
    ///
    /// An instance of [`Rcooch3`] containing the fatty acid expression.
    pub fn rcooch3(self) -> Rcooch3<Self> {
        Rcooch3(self)
    }
}
