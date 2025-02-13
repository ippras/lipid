use super::TriacylglycerolExpr;
use polars::prelude::*;

/// Permutation
pub trait Permutation {
    /// Non-stereospecific permutation
    fn non_stereospecific(self, f: impl Fn(Expr) -> Expr, options: Options) -> PolarsResult<Expr>;

    /// Positional permutation
    fn positional(self, f: impl Fn(Expr) -> Expr, options: Options) -> Expr;
}

impl Permutation for TriacylglycerolExpr {
    fn non_stereospecific(self, f: impl Fn(Expr) -> Expr, options: Options) -> PolarsResult<Expr> {
        Ok(if options.map {
            concat_list([f(self.clone().sn1()), f(self.clone().sn2()), f(self.sn3())])?
                .list()
                .sort(Default::default())
                .list()
                .to_struct(ListToStructArgs::FixedWidth(
                    [
                        "StereospecificNumber1".into(),
                        "StereospecificNumber2".into(),
                        "StereospecificNumber3".into(),
                    ]
                    .into(),
                ))
        } else {
            concat_list([self.clone().sn1(), self.clone().sn2(), self.sn3()])?
                .list()
                .eval(col("").sort_by([f(col(""))], Default::default()), true)
                .list()
                .to_struct(ListToStructArgs::FixedWidth(
                    [
                        "StereospecificNumber1".into(),
                        "StereospecificNumber2".into(),
                        "StereospecificNumber3".into(),
                    ]
                    .into(),
                ))
        })
    }

    fn positional(self, f: impl Fn(Expr) -> Expr, options: Options) -> Expr {
        if options.map {
            let sn1 = f(self.clone().sn1());
            let sn2 = f(self.clone().sn2());
            let sn3 = f(self.clone().sn3());
            let predicate = sn1.clone().lt_eq(sn3.clone());
            let (sn1, sn3) = (
                ternary_expr(predicate.clone(), sn1.clone(), sn3.clone()),
                ternary_expr(predicate, sn3, sn1),
            );
            as_struct(vec![
                sn1.alias("StereospecificNumber1"),
                sn2.alias("StereospecificNumber2"),
                sn3.alias("StereospecificNumber3"),
            ])
        } else {
            let sn1 = self.clone().sn1();
            let sn2 = self.clone().sn2();
            let sn3 = self.clone().sn3();
            let predicate = f(self.clone().sn1()).lt_eq(f(self.clone().sn3()));
            let (sn1, sn3) = (
                ternary_expr(predicate.clone(), sn1.clone(), sn3.clone()),
                ternary_expr(predicate, sn3, sn1),
            );
            as_struct(vec![
                sn1.alias("StereospecificNumber1"),
                sn2.alias("StereospecificNumber2"),
                sn3.alias("StereospecificNumber3"),
            ])
        }
    }
}

/// Permutation options
#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub map: bool,
}

impl Options {
    pub const fn new() -> Self {
        Self { map: false }
    }

    pub fn map(self, map: bool) -> Self {
        Self { map }
    }
}
