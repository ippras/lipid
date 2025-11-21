use crate::prelude::*;
use polars::prelude::*;

/// Triacylglycerol
pub const TRIACYLGLYCEROL: &str = "Triacylglycerol";
/// Stereospecific numbers 1
pub const STEREOSPECIFIC_NUMBERS1: &str = "StereospecificNumbers1";
/// Stereospecific numbers 2
pub const STEREOSPECIFIC_NUMBERS2: &str = "StereospecificNumbers2";
/// Stereospecific numbers 3
pub const STEREOSPECIFIC_NUMBERS3: &str = "StereospecificNumbers3";
/// Stereospecific numbers (1 and 2) or (2 and 3)
pub const STEREOSPECIFIC_NUMBERS12_23: &str = "StereospecificNumbers12(23)";
/// Stereospecific numbers 1 and 3
pub const STEREOSPECIFIC_NUMBERS13: &str = "StereospecificNumbers13";
/// Stereospecific numbers 1 or 3
pub const STEREOSPECIFIC_NUMBERS1_3: &str = "StereospecificNumbers1(3)";
/// Stereospecific numbers 1 and 2 and 3
pub const STEREOSPECIFIC_NUMBERS123: &str = "StereospecificNumbers123";
/// Label
pub const LABEL: &str = "Label";

/// Triacylglycerol [`Expr`]
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct TriacylglycerolExpr(pub Expr);

impl TriacylglycerolExpr {
    pub fn stereospecific_number1(self) -> Expr {
        self.0.struct_().field_by_name(STEREOSPECIFIC_NUMBERS1)
    }

    pub fn stereospecific_number2(self) -> Expr {
        self.0.struct_().field_by_name(STEREOSPECIFIC_NUMBERS2)
    }

    pub fn stereospecific_number3(self) -> Expr {
        self.0.struct_().field_by_name(STEREOSPECIFIC_NUMBERS3)
    }

    pub fn stereospecific_numbers(self) -> Expr {
        self.0.struct_().field_by_name("*")
    }
}

impl TriacylglycerolExpr {
    pub fn sum(self) -> Expr {
        self.clone().stereospecific_number1()
            + self.clone().stereospecific_number2()
            + self.stereospecific_number3()
    }

    pub fn unsaturation(self) -> Expr {
        self.clone()
            .stereospecific_number1()
            .fatty_acid()
            .unsaturation()
            + self
                .clone()
                .stereospecific_number2()
                .fatty_acid()
                .unsaturation()
            + self.stereospecific_number3().fatty_acid().unsaturation()
    }

    pub fn map(self, f: impl Fn(Expr) -> Expr) -> Expr {
        as_struct(vec![
            f(self.clone().stereospecific_number1()).alias(STEREOSPECIFIC_NUMBERS1),
            f(self.clone().stereospecific_number2()).alias(STEREOSPECIFIC_NUMBERS2),
            f(self.stereospecific_number3()).alias(STEREOSPECIFIC_NUMBERS3),
        ])
        .alias(TRIACYLGLYCEROL)
    }

    pub fn map_expr(self, f: impl Fn(Expr) -> Expr) -> Expr {
        as_struct(vec![
            f(self.clone().stereospecific_number1()).alias(STEREOSPECIFIC_NUMBERS1),
            f(self.clone().stereospecific_number2()).alias(STEREOSPECIFIC_NUMBERS2),
            f(self.stereospecific_number3()).alias(STEREOSPECIFIC_NUMBERS3),
        ])
        .alias(TRIACYLGLYCEROL)
    }

    pub fn try_map_expr(self, f: impl Fn(Expr) -> PolarsResult<Expr>) -> PolarsResult<Expr> {
        Ok(as_struct(vec![
            f(self.clone().stereospecific_number1())?.alias(STEREOSPECIFIC_NUMBERS1),
            f(self.clone().stereospecific_number2())?.alias(STEREOSPECIFIC_NUMBERS2),
            f(self.stereospecific_number3())?.alias(STEREOSPECIFIC_NUMBERS3),
        ])
        .alias(TRIACYLGLYCEROL))
    }

    pub fn test_map_apply<F>(self, f: &'static F) -> Expr
    where
        F: Fn(Column) -> PolarsResult<Column> + Send + Sync,
    {
        as_struct(vec![
            self.clone()
                .stereospecific_number1()
                .map(f, |_, field| Ok(field.clone()))
                .alias(STEREOSPECIFIC_NUMBERS1),
            self.clone()
                .stereospecific_number2()
                .map(f, |_, field| Ok(field.clone()))
                .alias(STEREOSPECIFIC_NUMBERS2),
            self.clone()
                .stereospecific_number3()
                .map(f, |_, field| Ok(field.clone()))
                .alias(STEREOSPECIFIC_NUMBERS3),
        ])
    }
}

impl From<TriacylglycerolExpr> for Expr {
    fn from(value: TriacylglycerolExpr) -> Self {
        value.0
    }
}

#[cfg(feature = "atomic")]
impl Atomic for TriacylglycerolExpr {
    type Output = Expr;

    #[inline]
    fn carbon(self) -> Expr {
        // C3H8O3
        const GLYCEROL: u8 = 3;

        self.0
            .clone()
            .struct_()
            .field_by_name(STEREOSPECIFIC_NUMBERS1)
            .fatty_acid()
            .carbon()
            + self
                .0
                .clone()
                .struct_()
                .field_by_name(STEREOSPECIFIC_NUMBERS2)
                .fatty_acid()
                .carbon()
            + self
                .0
                .struct_()
                .field_by_name(STEREOSPECIFIC_NUMBERS3)
                .fatty_acid()
                .carbon()
            + lit(GLYCEROL)
    }

    #[inline]
    fn hydrogen(self) -> Expr {
        // C3H8O3
        const GLYCEROL: u8 = 8;

        self.0
            .clone()
            .struct_()
            .field_by_name(STEREOSPECIFIC_NUMBERS1)
            .fatty_acid()
            .hydrogen()
            + self
                .0
                .clone()
                .struct_()
                .field_by_name(STEREOSPECIFIC_NUMBERS2)
                .fatty_acid()
                .hydrogen()
            + self
                .0
                .struct_()
                .field_by_name(STEREOSPECIFIC_NUMBERS3)
                .fatty_acid()
                .hydrogen()
            + lit(GLYCEROL - 3)
    }

    #[inline]
    fn oxygen(self) -> Expr {
        // C3H8O3
        const GLYCEROL: u8 = 3;

        self.0
            .clone()
            .struct_()
            .field_by_name(STEREOSPECIFIC_NUMBERS1)
            .fatty_acid()
            .oxygen()
            + self
                .0
                .clone()
                .struct_()
                .field_by_name(STEREOSPECIFIC_NUMBERS2)
                .fatty_acid()
                .oxygen()
            + self
                .0
                .struct_()
                .field_by_name(STEREOSPECIFIC_NUMBERS3)
                .fatty_acid()
                .oxygen()
            + lit(GLYCEROL - 3)
    }
}

impl EquivalentCarbonNumber for TriacylglycerolExpr {
    type Output = Expr;

    fn equivalent_carbon_number(self) -> Expr {
        (self
            .clone()
            .stereospecific_number1()
            .fatty_acid()
            .equivalent_carbon_number()
            + self
                .clone()
                .stereospecific_number2()
                .fatty_acid()
                .equivalent_carbon_number()
            + self
                .stereospecific_number3()
                .fatty_acid()
                .equivalent_carbon_number())
        .alias("EquivalentCarbonNumber")
    }
}

pub mod permutation;

mod factors;
mod relative_atomic_mass;
