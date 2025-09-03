use crate::prelude::*;
use polars::prelude::*;

/// Triacylglycerol
pub const TRIACYLGLYCEROL: &str = "Triacylglycerol";
/// Stereospecific number 1
pub const STEREOSPECIFIC_NUMBER1: &str = "StereospecificNumber1";
/// Stereospecific number 2
pub const STEREOSPECIFIC_NUMBER2: &str = "StereospecificNumber2";
/// Stereospecific number 3
pub const STEREOSPECIFIC_NUMBER3: &str = "StereospecificNumber3";
/// Stereospecific numbers (1 and 2) or (2 and 3)
pub const STEREOSPECIFIC_NUMBER12_23: &str = "StereospecificNumber12(23)";
/// Stereospecific numbers 1 and 3
pub const STEREOSPECIFIC_NUMBER13: &str = "StereospecificNumber13";
/// Stereospecific numbers 1 or 3
pub const STEREOSPECIFIC_NUMBER1_3: &str = "StereospecificNumber1(3)";
/// Stereospecific number 1 and 2 and 3
pub const STEREOSPECIFIC_NUMBER123: &str = "StereospecificNumber123";
/// Label
pub const LABEL: &str = "Label";

/// Triacylglycerol [`Expr`]
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct TriacylglycerolExpr(pub Expr);

impl TriacylglycerolExpr {
    pub fn stereospecific_number1(self) -> Expr {
        self.0.struct_().field_by_name(STEREOSPECIFIC_NUMBER1)
    }

    pub fn stereospecific_number2(self) -> Expr {
        self.0.struct_().field_by_name(STEREOSPECIFIC_NUMBER2)
    }

    pub fn stereospecific_number3(self) -> Expr {
        self.0.struct_().field_by_name(STEREOSPECIFIC_NUMBER3)
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

    pub fn map_expr(self, f: impl Fn(Expr) -> Expr) -> Expr {
        as_struct(vec![
            f(self.clone().stereospecific_number1()).alias(STEREOSPECIFIC_NUMBER1),
            f(self.clone().stereospecific_number2()).alias(STEREOSPECIFIC_NUMBER2),
            f(self.stereospecific_number3()).alias(STEREOSPECIFIC_NUMBER3),
        ])
        .alias("Triacylglycerol")
    }

    pub fn try_map_expr(self, f: impl Fn(Expr) -> PolarsResult<Expr>) -> PolarsResult<Expr> {
        Ok(as_struct(vec![
            f(self.clone().stereospecific_number1())?.alias(STEREOSPECIFIC_NUMBER1),
            f(self.clone().stereospecific_number2())?.alias(STEREOSPECIFIC_NUMBER2),
            f(self.stereospecific_number3())?.alias(STEREOSPECIFIC_NUMBER3),
        ])
        .alias("Triacylglycerol"))
    }

    pub fn test_map_apply<F>(self, f: &'static F) -> Expr
    where
        F: Fn(Column) -> PolarsResult<Option<Column>> + Send + Sync,
    {
        as_struct(vec![
            self.clone()
                .stereospecific_number1()
                .map(f, GetOutput::same_type())
                .alias(STEREOSPECIFIC_NUMBER1),
            self.clone()
                .stereospecific_number2()
                .map(f, GetOutput::same_type())
                .alias(STEREOSPECIFIC_NUMBER2),
            self.clone()
                .stereospecific_number3()
                .map(f, GetOutput::same_type())
                .alias(STEREOSPECIFIC_NUMBER3),
        ])
    }
}

impl From<TriacylglycerolExpr> for Expr {
    fn from(value: TriacylglycerolExpr) -> Self {
        value.0
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

mod mass;
