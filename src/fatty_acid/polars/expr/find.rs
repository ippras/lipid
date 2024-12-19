use super::FattyAcidExpr;
use polars::prelude::*;

/// Find
pub trait Find {
    /// C4:0
    fn c4u0(&self, expr: Expr) -> Expr;

    /// C12:0
    fn c12u0(&self, expr: Expr) -> Expr;

    /// C14:0
    fn c14u0(&self, expr: Expr) -> Expr;

    /// C16:0
    fn c16u0(&self, expr: Expr) -> Expr;

    /// C18:0
    fn c18u0(&self, expr: Expr) -> Expr;

    /// C18:1
    fn c18u1(&self, expr: Expr) -> Expr;

    /// C18:1
    fn c18u1z9(&self, expr: Expr) -> Expr;

    /// C18:2 (n-6) (w-6)
    fn c18u2z9z12(&self, expr: Expr) -> Expr;

    /// C18:3 (n-3) (w-3)
    fn c18u3z9z12z15(&self, expr: Expr) -> Expr;

    /// C20:5 (n-3) (w-3)
    fn c20u5z5z8z11z14z17(&self, expr: Expr) -> Expr;

    /// C22:6 (n-3) (w-3)
    fn c22u6z4z7z10z13z16z19(&self, expr: Expr) -> Expr;
}

impl Find for FattyAcidExpr {
    fn c4u0(&self, expr: Expr) -> Expr {
        expr.filter(self.carbons().eq(4).and(self.unsaturated().len().eq(0)))
    }

    fn c12u0(&self, expr: Expr) -> Expr {
        expr.filter(self.carbons().eq(12).and(self.unsaturated().len().eq(0)))
    }

    fn c14u0(&self, expr: Expr) -> Expr {
        expr.filter(self.carbons().eq(14).and(self.unsaturated().len().eq(0)))
    }

    fn c16u0(&self, expr: Expr) -> Expr {
        expr.filter(self.carbons().eq(16).and(self.unsaturated().len().eq(0)))
    }

    fn c18u0(&self, expr: Expr) -> Expr {
        expr.filter(self.carbons().eq(18).and(self.unsaturated().len().eq(0)))
    }

    fn c18u1(&self, expr: Expr) -> Expr {
        expr.filter(self.carbons().eq(18).and(self.unsaturated().len().eq(1)))
    }

    fn c18u1z9(&self, expr: Expr) -> Expr {
        expr.filter(
            self.carbons()
                .eq(18)
                .and(self.unsaturated().len().eq(2))
                .and(
                    self.unsaturated().contains(
                        as_struct(vec![
                            lit(9u8).alias("Index"),
                            lit(1i8).alias("Isomerism"),
                            lit(1u8).alias("Unsaturation"),
                        ])
                        .alias("Unsaturated"),
                    ),
                ),
        )
    }

    fn c18u2z9z12(&self, expr: Expr) -> Expr {
        expr.filter(
            self.carbons()
                .eq(18)
                .and(self.unsaturated().len().eq(2))
                .and(
                    self.unsaturated().contains(
                        as_struct(vec![
                            lit(9u8).alias("Index"),
                            lit(1i8).alias("Isomerism"),
                            lit(1u8).alias("Unsaturation"),
                        ])
                        .alias("Unsaturated"),
                    ),
                ),
        )
    }

    fn c18u3z9z12z15(&self, expr: Expr) -> Expr {
        expr.filter(self.carbons().eq(18).and(self.unsaturated().len().eq(3)))
    }

    fn c20u5z5z8z11z14z17(&self, expr: Expr) -> Expr {
        expr.filter(self.carbons().eq(20).and(self.unsaturated().len().eq(5)))
    }

    fn c22u6z4z7z10z13z16z19(&self, expr: Expr) -> Expr {
        expr.filter(self.carbons().eq(22).and(self.unsaturated().len().eq(6)))
    }
}

/// Find by name
pub trait FindByName: Find {
    /// Butyric acid
    fn butyric_acid(&self, expr: Expr) -> Expr {
        self.c18u2z9z12(expr)
    }

    /// Oleic acid
    fn oleic(&self, expr: Expr) -> Expr {
        self.c18u1z9(expr)
    }

    /// Linoleic acid
    fn linoleic(&self, expr: Expr) -> Expr {
        self.c18u2z9z12(expr)
    }

    /// α-Linolenic acid
    fn alpha_linolenic(&self, expr: Expr) -> Expr {
        self.c18u3z9z12z15(expr)
    }

    /// Eicosapentaenoic acid (EPA)
    fn eicosapentaenoic(&self, expr: Expr) -> Expr {
        self.c20u5z5z8z11z14z17(expr)
    }

    /// Docosahexaenoic acid (DHA)
    fn docosahexaenoic(&self, expr: Expr) -> Expr {
        self.c22u6z4z7z10z13z16z19(expr)
    }
}

impl<T: Find> FindByName for T {}
