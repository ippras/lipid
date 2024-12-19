use super::FattyAcidExpr;
use polars::prelude::*;
use std::sync::LazyLock;

/// Lauric acid
pub static C12U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(12).alias("Carbons"),
        EMPTY_LIST.clone().alias("Unsaturated"),
    ]))
});

/// Myristic acid
pub static C14U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(14).alias("Carbons"),
        EMPTY_LIST.clone().alias("Unsaturated"),
    ]))
});

/// Palmitic acid
pub static C16U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(16).alias("Carbons"),
        EMPTY_LIST.clone().alias("Unsaturated"),
    ]))
});

/// Stearic acid
pub static C18U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(18).alias("Carbons"),
        EMPTY_LIST.clone().alias("Unsaturated"),
    ]))
});

/// Oleic acid
pub static C18U1Z9: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(18).alias("Carbons"),
        concat_list([as_struct(vec![
            lit(9).alias("Index"),
            lit(1).alias("Isomerism"),
            lit(1).alias("Unsaturation"),
        ])])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Linoleic acid
pub static C18U2Z9Z12: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(18).alias("Carbons"),
        concat_list([
            as_struct(vec![
                lit(0).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(0).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(0).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(0).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(0).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(0).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
        ])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// α-Linolenic acid
pub static C18U3Z9Z12Z15: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(18).alias("Carbons"),
        concat_list([
            as_struct(vec![
                lit(9).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(12).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(15).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
        ])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Eicosapentaenoic acid (EPA)
pub static C20U5Z5Z8Z11Z14Z17: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(20).alias("Carbons"),
        concat_list([
            as_struct(vec![
                lit(5).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(8).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(11).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(14).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(17).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
        ])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Docosahexaenoic acid (DHA)
pub static C22U6Z4Z7Z10Z13Z16Z19: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(22).alias("Carbons"),
        concat_list([
            as_struct(vec![
                lit(4).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(7).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(10).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(13).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(16).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(19).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
        ])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

static EMPTY_LIST: LazyLock<Expr> = LazyLock::new(|| {
    lit(Scalar::new(
        DataType::List(Box::new(DataType::Null)),
        AnyValue::List(Series::new_empty(PlSmallStr::EMPTY, &DataType::Null)),
    ))
});
