use super::TriacylglycerolExpr;
use crate::prelude::*;
use polars::prelude::*;

/// Permutation
pub trait Permutation {
    /// Permutate
    fn permutate(
        self,
        f: impl Fn(Expr) -> Expr,
        stereospecificity: Option<Stereospecificity>,
    ) -> Expr;
}

impl Permutation for TriacylglycerolExpr {
    fn permutate(
        self,
        f: impl Fn(Expr) -> Expr,
        stereospecificity: Option<Stereospecificity>,
    ) -> Expr {
        match stereospecificity {
            None => {
                let sn1 = self.clone().stereospecific_number1();
                let sn2 = self.clone().stereospecific_number2();
                let sn3 = self.clone().stereospecific_number3();
                // Sort sn1, sn2
                let predicate = f(sn1.clone()).gt(f(sn2.clone()));
                let (sn1, sn2) = (
                    ternary_expr(predicate.clone(), sn2.clone(), sn1.clone()),
                    ternary_expr(predicate, sn1.clone(), sn2.clone()),
                );
                // Sort sn1, sn3
                let predicate = f(sn1.clone()).gt(f(sn3.clone()));
                let (sn1, sn3) = (
                    ternary_expr(predicate.clone(), sn3.clone(), sn1.clone()),
                    ternary_expr(predicate, sn1.clone(), sn3.clone()),
                );
                // Sort sn2, sn3
                let predicate = f(sn2.clone()).gt(f(sn3.clone()));
                let (sn2, sn3) = (
                    ternary_expr(predicate.clone(), sn3.clone(), sn2.clone()),
                    ternary_expr(predicate, sn2.clone(), sn3.clone()),
                );
                as_struct(vec![
                    sn1.alias(STEREOSPECIFIC_NUMBERS1),
                    sn2.alias(STEREOSPECIFIC_NUMBERS2),
                    sn3.alias(STEREOSPECIFIC_NUMBERS3),
                ])
            }
            Some(Stereospecificity::Positional) => {
                let sn1 = self.clone().stereospecific_number1();
                let sn2 = self.clone().stereospecific_number2();
                let sn3 = self.clone().stereospecific_number3();
                // Sort sn1, sn3
                let predicate = f(sn1.clone()).lt_eq(f(sn3.clone()));
                let (sn1, sn3) = (
                    ternary_expr(predicate.clone(), sn1.clone(), sn3.clone()),
                    ternary_expr(predicate, sn3, sn1),
                );
                as_struct(vec![
                    sn1.alias(STEREOSPECIFIC_NUMBERS1),
                    sn2.alias(STEREOSPECIFIC_NUMBERS2),
                    sn3.alias(STEREOSPECIFIC_NUMBERS3),
                ])
            }
            Some(Stereospecificity::Stereo) => self.0,
        }
    }
}
