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
/// Label
pub const LABEL: &str = "Label";

/// Triacylglycerol [`Expr`]
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct TriacylglycerolExpr(pub Expr);

impl TriacylglycerolExpr {
    pub fn stereospecific_number1(self) -> Expr {
        self.0.struct_().field_by_name("StereospecificNumber1")
    }

    pub fn stereospecific_number2(self) -> Expr {
        self.0.struct_().field_by_name("StereospecificNumber2")
    }

    pub fn stereospecific_number3(self) -> Expr {
        self.0.struct_().field_by_name("StereospecificNumber3")
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
            f(self.clone().stereospecific_number1()).alias("StereospecificNumber1"),
            f(self.clone().stereospecific_number2()).alias("StereospecificNumber2"),
            f(self.stereospecific_number3()).alias("StereospecificNumber3"),
        ])
        .alias("Triacylglycerol")
    }

    pub fn test_map_apply<F>(self, f: &'static F) -> Expr
    where
        F: Fn(Column) -> PolarsResult<Option<Column>> + Send + Sync,
    {
        as_struct(vec![
            self.clone()
                .stereospecific_number1()
                .map(f, GetOutput::same_type())
                .alias("StereospecificNumber1"),
            self.clone()
                .stereospecific_number2()
                .map(f, GetOutput::same_type())
                .alias("StereospecificNumber2"),
            self.clone()
                .stereospecific_number3()
                .map(f, GetOutput::same_type())
                .alias("StereospecificNumber3"),
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
