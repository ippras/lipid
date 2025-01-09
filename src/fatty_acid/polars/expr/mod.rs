use polars::prelude::*;

/// Extension methods for [`Expr`]
pub trait ExprExt {
    fn fatty_acid(self) -> FattyAcidExpr;
}

impl ExprExt for Expr {
    fn fatty_acid(self) -> FattyAcidExpr {
        FattyAcidExpr(self)
    }
}

/// Fatty acid [`Expr`]
#[derive(Clone, Debug)]
pub struct FattyAcidExpr(pub Expr);

impl FattyAcidExpr {
    /// Carbons
    pub fn carbons(self) -> Expr {
        self.0.struct_().field_by_name("Carbons")
    }

    /// Unsaturated
    pub fn unsaturated(self) -> UnsaturatedExpr {
        UnsaturatedExpr(self.0.struct_().field_by_name("Unsaturated"))
    }

    /// Equal
    pub fn equal(self, other: impl Into<FattyAcidExpr>) -> Expr {
        self.0.eq(other.into().0)
    }

    /// Replace unsaturated with null
    pub fn saturated_or_null(self, expr: Expr) -> Expr {
        ternary_expr(self.is_saturated(), expr, lit(NULL))
    }

    /// Replace saturated with null
    pub fn unsaturated_or_null(self, expr: Expr) -> Expr {
        ternary_expr(self.is_saturated().not(), expr, lit(NULL))
    }

    // /// Double bounds count
    // pub fn d(&self) -> Expr {
    //     self.0
    //         .clone()
    //         .struct_()
    //         .field_by_name("Doubles")
    //         .list()
    //         .len()
    // }

    // /// Triple bounds count
    // pub fn t(&self) -> Expr {
    //     self.0
    //         .clone()
    //         .struct_()
    //         .field_by_name("Triples")
    //         .list()
    //         .len()
    // }

    // pub fn r#type(self) -> Expr {
    //     ternary_expr(self.saturated(), lit("S"), lit("U"))
    // }

    // pub fn unsaturated(self) -> Expr {
    //     self.saturated().not()
    // }

    // pub fn unsaturation(self) -> Expr {
    //     self.d() + lit(2) * self.t()
    // }
}

impl FattyAcidExpr {
    /// Bounds
    pub fn bounds(self) -> Expr {
        (self.carbons() - lit(1)).clip_min(lit(0))
    }

    /// Hydrogens
    ///
    /// `H = 2C - 2U`
    pub fn hydrogens(self) -> Expr {
        lit(2) * self.clone().carbons() - lit(2) * self.unsaturated().sum()
    }

    /// Is saturated
    pub fn is_saturated(self) -> Expr {
        self.unsaturated().len().eq(0)
    }

    /// [`bounds`]
    pub fn b(self) -> Expr {
        self.bounds()
    }

    /// [`carbons`]
    pub fn c(self) -> Expr {
        self.carbons()
    }

    /// [`hydrogens`]
    pub fn h(self) -> Expr {
        self.hydrogens()
    }

    /// [`saturated`]
    pub fn s(self) -> Expr {
        self.is_saturated()
    }

    /// [`unsaturation`]
    pub fn u(self) -> Expr {
        self.unsaturated().len()
    }
}

impl From<FattyAcidExpr> for Expr {
    fn from(value: FattyAcidExpr) -> Self {
        value.0
    }
}

/// Unsaturated [`Expr`]
#[derive(Clone, Debug)]
pub struct UnsaturatedExpr(pub Expr);

impl UnsaturatedExpr {
    /// Unsaturated
    ///
    /// The number of unsaturated bonds.
    pub fn len(self) -> Expr {
        self.0
            .list()
            .eval(col("").struct_().field_by_name("Unsaturation"), true)
            .list()
            .len()
    }

    /// Unsaturation (sum)
    pub fn sum(self) -> Expr {
        self.0
            .list()
            .eval(col("").struct_().field_by_name("Unsaturation"), true)
            .list()
            .sum()
    }

    /// List
    pub fn list(self) -> ListNameSpace {
        self.0.list()
    }

    /// Equal
    pub fn equal(self, other: UnsaturatedExpr) -> Expr {
        self.0.eq(other)
    }
}

impl From<UnsaturatedExpr> for Expr {
    fn from(value: UnsaturatedExpr) -> Self {
        value.0
    }
}

pub mod chain_length;
pub mod r#const;
pub mod factor;
pub mod filter;
pub mod find;
pub mod mass;
pub mod short;
