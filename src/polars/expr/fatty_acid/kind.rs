use super::FattyAcidExpr;

/// Fatty acid [RCO]+
///
/// `RCOOH -OH => [RCO]+`
pub struct Rco(pub FattyAcidExpr);

/// Fatty acid [RCOO]-
///
/// `RCOOH -H => [RCOO]-`
pub struct Rcoo(pub FattyAcidExpr);

/// Fatty acid
///
/// `RCOOH`
pub struct Rcooh(pub FattyAcidExpr);

/// Fatty acid methyl ester
///
/// `RCOOH -H +CH3 => RCOOCH3`
pub struct Rcooch3(pub FattyAcidExpr);

/// Extension methods for [`FattyAcidExpr`]
pub trait FattyAcidExprExt {
    fn rco(self) -> Rco;

    fn rcoo(self) -> Rcoo;

    fn rcooh(self) -> Rcooh;

    fn rcooch3(self) -> Rcooch3;
}

impl FattyAcidExprExt for FattyAcidExpr {
    fn rco(self) -> Rco {
        Rco(self)
    }

    fn rcoo(self) -> Rcoo {
        Rcoo(self)
    }

    fn rcooh(self) -> Rcooh {
        Rcooh(self)
    }

    fn rcooch3(self) -> Rcooch3 {
        Rcooch3(self)
    }
}
