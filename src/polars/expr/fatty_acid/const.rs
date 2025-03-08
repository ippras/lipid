pub use self::{saturated::*, unsaturated::*};

use super::FattyAcidExpr;
use crate::prelude::*;
use polars::prelude::*;
use std::sync::LazyLock;

// #[inline]
// pub fn fatty_acid<'a>(iter: impl IntoIterator<Item = &'a str>) -> PolarsResult<Expr> {
//     Ok(lit(Scalar::new(
//         DataType::List(Box::new(BOUND_DATA_TYPE.clone())),
//         AnyValue::List(Series::from_iter(iter).cast(&BOUND_DATA_TYPE)?),
//     )))
// }
macro_rules! fatty_acid_expr {
    ($id:ident) => {
        pub static $id: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
            FattyAcidExpr(lit(Scalar::new(
                DataType::List(Box::new(BOUND_DATA_TYPE.clone())),
                AnyValue::List(
                    Series::from_iter(crate::fatty_acid::r#const::$id)
                        .cast(&BOUND_DATA_TYPE)
                        .unwrap(),
                ),
            )))
        });
    };
}

mod saturated {
    use super::*;

    fatty_acid_expr!(C1U0);
    fatty_acid_expr!(C2U0);
    fatty_acid_expr!(C3U0);
    fatty_acid_expr!(C4U0);
    fatty_acid_expr!(C5U0);
    fatty_acid_expr!(C6U0);
    fatty_acid_expr!(C7U0);
    fatty_acid_expr!(C8U0);
    fatty_acid_expr!(C9U0);
    fatty_acid_expr!(C10U0);
    fatty_acid_expr!(C11U0);
    fatty_acid_expr!(C12U0);
    fatty_acid_expr!(C13U0);
    fatty_acid_expr!(C14U0);
    fatty_acid_expr!(C15U0);
    fatty_acid_expr!(C16U0);
    fatty_acid_expr!(C17U0);
    fatty_acid_expr!(C18U0);
    fatty_acid_expr!(C19U0);
    fatty_acid_expr!(C20U0);
    fatty_acid_expr!(C21U0);
    fatty_acid_expr!(C22U0);
    fatty_acid_expr!(C23U0);
    fatty_acid_expr!(C24U0);
    fatty_acid_expr!(C25U0);
    fatty_acid_expr!(C26U0);
    fatty_acid_expr!(C27U0);
    fatty_acid_expr!(C28U0);
    fatty_acid_expr!(C29U0);
    fatty_acid_expr!(C30U0);
    fatty_acid_expr!(C31U0);
    fatty_acid_expr!(C32U0);
    fatty_acid_expr!(C33U0);
    fatty_acid_expr!(C34U0);
    fatty_acid_expr!(C35U0);
    fatty_acid_expr!(C36U0);
    fatty_acid_expr!(C37U0);
    fatty_acid_expr!(C38U0);
    fatty_acid_expr!(C39U0);
    fatty_acid_expr!(C40U0);
    fatty_acid_expr!(C41U0);
    fatty_acid_expr!(C42U0);
    fatty_acid_expr!(C43U0);
    fatty_acid_expr!(C44U0);
    fatty_acid_expr!(C45U0);
    fatty_acid_expr!(C46U0);
    fatty_acid_expr!(C47U0);
    fatty_acid_expr!(C48U0);
    fatty_acid_expr!(C49U0);
    fatty_acid_expr!(C50U0);
    fatty_acid_expr!(C51U0);
    fatty_acid_expr!(C52U0);
    fatty_acid_expr!(C53U0);
    fatty_acid_expr!(C54U0);
    fatty_acid_expr!(C55U0);
    fatty_acid_expr!(C56U0);
    fatty_acid_expr!(C57U0);
    fatty_acid_expr!(C58U0);
    fatty_acid_expr!(C59U0);
    fatty_acid_expr!(C60U0);
    fatty_acid_expr!(C61U0);
    fatty_acid_expr!(C62U0);
    fatty_acid_expr!(C63U0);
    fatty_acid_expr!(C64U0);

    // pub static _C1U0: LazyLock<Expr> = LazyLock::new(|| fatty_acid_expr!(C1U0).unwrap());
}

mod unsaturated {
    use super::*;

    // [[byrdwell.com]](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
    fatty_acid_expr!(C16U1DC9);
    fatty_acid_expr!(C16U1DT9);
    fatty_acid_expr!(C18U1DC9);
    fatty_acid_expr!(C18U1DT9);
    fatty_acid_expr!(C18U2DC9DC12);
    fatty_acid_expr!(C18U3DC9DC12DC15);
    fatty_acid_expr!(C18U3DC6DC9DC12);
    fatty_acid_expr!(C18U3DC9DT11DT13);
    fatty_acid_expr!(C18U3DT9DT11DT13);
    fatty_acid_expr!(C18U3DC8DT10DC12);
    fatty_acid_expr!(C18U3DT9DT11DC13);
    fatty_acid_expr!(C18U4DC6DC9DC12DC15);
    fatty_acid_expr!(C20U1DC9);
    fatty_acid_expr!(C20U1DC11);
    fatty_acid_expr!(C20U2DC11DC14);
    fatty_acid_expr!(C20U3DC11DC14DC17);
    fatty_acid_expr!(C20U3DC8DC11DC14);
    fatty_acid_expr!(C20U3DC5DC8DC11);
    fatty_acid_expr!(C20U4DC5DC8DC11DC14);
    fatty_acid_expr!(C20U4DC8DC11DC14DC17);
    fatty_acid_expr!(C20U5DC5DC8DC11DC14DC17);
    fatty_acid_expr!(C22U1DC13);
    fatty_acid_expr!(C22U2DC13DC16);
    fatty_acid_expr!(C22U3DC5DC13DC16);
    fatty_acid_expr!(C22U4DC7DC10DC13DC16);
    fatty_acid_expr!(C22U5DC7DC10DC13DC16DC19);
    fatty_acid_expr!(C22U6DC4DC7DC10DC13DC16DC19);
    fatty_acid_expr!(C24U1DC15);
    fatty_acid_expr!(C24U2DC15DC18);
    fatty_acid_expr!(C24U3DC12DC15DC18);
    fatty_acid_expr!(C24U4DC9DC12DC15DC18);
    fatty_acid_expr!(C24U5DC6DC9DC12DC15DC18);
    fatty_acid_expr!(C24U6DC6DC9DC12DC15DC18DC21);
    fatty_acid_expr!(C26U1DC17);
    // fatty_acid_expr!(C26U2);
    // fatty_acid_expr!(C26U3);
    // fatty_acid_expr!(C26U4);
    // fatty_acid_expr!(C26U5);
    // fatty_acid_expr!(C26U6);
    // fatty_acid_expr!(C28U1);
    // fatty_acid_expr!(C28U2);
    fatty_acid_expr!(C30U1DC21);
    // fatty_acid_expr!(C30U2);
    // fatty_acid_expr!(C32U1);
    // fatty_acid_expr!(C32U2);
    // fatty_acid_expr!(C34U1);
    // fatty_acid_expr!(C34U2);
    // fatty_acid_expr!(C36U1);
    // fatty_acid_expr!(C36U2);
}
