use super::FattyAcidExpr;
use polars::prelude::*;
use std::sync::LazyLock;

macro_rules! fatty_acid_expr {
    (C $c:literal U $u:literal $($i:tt)*) => {
        paste::paste! {
            pub static [<C $c U $u $($i)*>]: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
                let mut fields = vec![lit($c).alias("Carbons")];
                if $u == 0 {
                    fields.push(EMPTY_LIST.clone().alias("Unsaturated"));
                }
                fatty_acid_expr!(fields, $($i)*)
            });
        }
    };
    ($fields:ident, Z $z:literal $($i:tt)*) => {{
        $fields.push(
            concat_list([as_struct(vec![
                lit($z).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ])])
            .unwrap()
            .alias("Unsaturated"),
        );
        fatty_acid_expr!($fields, $($i)*)
    }};
    ($fields:ident, E $e:literal $($i:tt)*) => {{
        $fields.push(
            concat_list([as_struct(vec![
                lit($e).alias("Index"),
                lit(-1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ])])
            .unwrap()
            .alias("Unsaturated"),
        );
        fatty_acid_expr!($fields, $($i)*)
    }};
    ($fields:ident,) => {
        FattyAcidExpr(as_struct($fields))
    };
}

// fn t() {
//     fields.push(
//         concat_list([as_struct(vec![
//             lit($z).alias("Index"),
//             lit(-1).alias("Isomerism"),
//             lit(1).alias("Unsaturation"),
//         ])])
//         .unwrap()
//         .alias("Unsaturated"),
//     )
// }

fatty_acid_expr!(C 2 U 2 Z 9 E 11);

/// Butyric acid
pub static C4U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(4).alias("Carbons"),
        EMPTY_LIST.clone().alias("Unsaturated"),
    ]))
});

/// Valeric acid
pub static C5U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(5).alias("Carbons"),
        EMPTY_LIST.clone().alias("Unsaturated"),
    ]))
});

/// Caproic acid
pub static C6U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(6).alias("Carbons"),
        EMPTY_LIST.clone().alias("Unsaturated"),
    ]))
});

/// Enanthic acid
pub static C7U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(7).alias("Carbons"),
        EMPTY_LIST.clone().alias("Unsaturated"),
    ]))
});

/// Caprylic acid
pub static C8U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(8).alias("Carbons"),
        EMPTY_LIST.clone().alias("Unsaturated"),
    ]))
});

/// Pelargonic acid
pub static C9U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(9).alias("Carbons"),
        EMPTY_LIST.clone().alias("Unsaturated"),
    ]))
});

/// Capric acid
pub static C10U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(10).alias("Carbons"),
        EMPTY_LIST.clone().alias("Unsaturated"),
    ]))
});

/// Undecylic acid
pub static C11U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(11).alias("Carbons"),
        EMPTY_LIST.clone().alias("Unsaturated"),
    ]))
});

/// Lauric acid
pub static C12U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(12).alias("Carbons"),
        EMPTY_LIST.clone().alias("Unsaturated"),
    ]))
});

/// Tridecylic acid
pub static C13U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(13).alias("Carbons"),
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

/// Pentadecylic acid
pub static C15U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(15).alias("Carbons"),
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

/// Palmitoleic acid (n-7)
pub static C16U1Z9: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(16).alias("Carbons"),
        concat_list([as_struct(vec![
            lit(9).alias("Index"),
            lit(1).alias("Isomerism"),
            lit(1).alias("Unsaturation"),
        ])])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Palmitelaidic acid (n-7)
pub static C16U1E9: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(16).alias("Carbons"),
        concat_list([as_struct(vec![
            lit(9).alias("Index"),
            lit(-1).alias("Isomerism"),
            lit(1).alias("Unsaturation"),
        ])])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Margaric acid
pub static C17U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(17).alias("Carbons"),
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

/// Oleic acid (n-9)
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

/// Elaidic acid (n-9)
pub static C18U1E9: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(18).alias("Carbons"),
        concat_list([as_struct(vec![
            lit(9).alias("Index"),
            lit(-1).alias("Isomerism"),
            lit(1).alias("Unsaturation"),
        ])])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Linoleic acid (ω-6)
pub static C18U2Z9Z12: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
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
        ])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// α-Linolenic acid (ω-3)
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

/// γ-Linolenic acid (ω-6), GLA
pub static C18U3: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(18).alias("Carbons"),
        concat_list([
            as_struct(vec![
                lit(6).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
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
        ])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// α-Eleostearic acid
pub static C18U3Z9E11E13: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(18).alias("Carbons"),
        concat_list([
            as_struct(vec![
                lit(9).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(11).alias("Index"),
                lit(-1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(13).alias("Index"),
                lit(-1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
        ])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// β-Eleostearic acid
pub static C18U3E9E11E13: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(18).alias("Carbons"),
        concat_list([
            as_struct(vec![
                lit(9).alias("Index"),
                lit(-1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(11).alias("Index"),
                lit(-1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(13).alias("Index"),
                lit(-1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
        ])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Jacaric acid
pub static C18U3Z8E10Z12: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(18).alias("Carbons"),
        concat_list([
            as_struct(vec![
                lit(8).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(10).alias("Index"),
                lit(-1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(12).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
        ])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Catalpic acid
pub static C18U3E9E11Z13: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(18).alias("Carbons"),
        concat_list([
            as_struct(vec![
                lit(9).alias("Index"),
                lit(-1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(11).alias("Index"),
                lit(-1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(13).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
        ])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Stearidonic acid (ω-3)
pub static C18U4Z6Z9Z12Z15: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(18).alias("Carbons"),
        concat_list([
            as_struct(vec![
                lit(6).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
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

/// Nonadecylic acid
pub static C19U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(19).alias("Carbons"),
        EMPTY_LIST.clone().alias("Unsaturated"),
    ]))
});

/// Arachidic acid
pub static C20U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(20).alias("Carbons"),
        EMPTY_LIST.clone().alias("Unsaturated"),
    ]))
});

/// Gadoleic acid (n-11)
pub static C20U1Z9: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(20).alias("Carbons"),
        concat_list([as_struct(vec![
            lit(9).alias("Index"),
            lit(1).alias("Isomerism"),
            lit(1).alias("Unsaturation"),
        ])])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Gondoic acid (n-9)
pub static C20U1Z11: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(20).alias("Carbons"),
        concat_list([as_struct(vec![
            lit(11).alias("Index"),
            lit(1).alias("Isomerism"),
            lit(1).alias("Unsaturation"),
        ])])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// DihomoLinoleic acid (ω-6)
pub static C20U2Z11Z14: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(20).alias("Carbons"),
        concat_list([
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
        ])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Bis-homo-α-Linolenic acid (ω-3), ETA
pub static C20U3Z11Z14Z17: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(20).alias("Carbons"),
        concat_list([
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

/// Bis-homo-γ-Linolenic acid (ω-6)
pub static C20U3Z8Z11Z14: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(20).alias("Carbons"),
        concat_list([
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
        ])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Mead Acid
pub static C20U3Z5Z8Z11: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
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
        ])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Arachidonic acid (ω-6)
pub static C20U4Z5Z8Z11Z14: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
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
        ])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Eicosatetraenoic acid (ω-3)
pub static C20U4Z8Z11Z14Z17: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(20).alias("Carbons"),
        concat_list([
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

/// EPA (ω-3)
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

/// Heneicosylic acid
pub static C21U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(21).alias("Carbons"),
        EMPTY_LIST.clone().alias("Unsaturated"),
    ]))
});

/// Behenic acid
pub static C22U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(22).alias("Carbons"),
        EMPTY_LIST.clone().alias("Unsaturated"),
    ]))
});

/// Erucic acid (n-9)
pub static C22U1Z13: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(22).alias("Carbons"),
        concat_list([as_struct(vec![
            lit(13).alias("Index"),
            lit(1).alias("Isomerism"),
            lit(1).alias("Unsaturation"),
        ])])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Docosadienoic acid (ω-6)
pub static C22U2Z13Z16: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(22).alias("Carbons"),
        concat_list([
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
        ])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Eranthic acid (ω-6)
pub static C22U3Z5Z13Z16: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(22).alias("Carbons"),
        concat_list([
            as_struct(vec![
                lit(5).alias("Index"),
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
        ])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Adrenic acid (ω-6)
pub static C22U4Z7Z10Z13Z16: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(22).alias("Carbons"),
        concat_list([
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
        ])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// DPA (ω-3)
pub static C22U5Z7Z10Z13Z16Z19: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(22).alias("Carbons"),
        concat_list([
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

/// DHA (ω-3)
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

/// Tricosylic acid
pub static C23U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(23).alias("Carbons"),
        EMPTY_LIST.clone().alias("Unsaturated"),
    ]))
});

/// Lignoceric acid
pub static C24U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(24).alias("Carbons"),
        EMPTY_LIST.clone().alias("Unsaturated"),
    ]))
});

/// Nervonic acid (n-9)
pub static C24U1Z15: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(24).alias("Carbons"),
        concat_list([as_struct(vec![
            lit(15).alias("Index"),
            lit(1).alias("Isomerism"),
            lit(1).alias("Unsaturation"),
        ])])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Tetracosadienoic acid (ω-6)
pub static C24U2Z15Z18: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(24).alias("Carbons"),
        concat_list([
            as_struct(vec![
                lit(15).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(18).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
        ])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Tetracosatrienylic acid
pub static C24U3Z12Z15Z18: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(24).alias("Carbons"),
        concat_list([
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
            as_struct(vec![
                lit(18).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
        ])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Tetracosatetraenylic acid
pub static C24U4Z9Z12Z15Z18: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(24).alias("Carbons"),
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
            as_struct(vec![
                lit(18).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
        ])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Tetracosapentaenylic acid
pub static C24U5Z6Z9Z12Z15Z18: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(24).alias("Carbons"),
        concat_list([
            as_struct(vec![
                lit(6).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
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
            as_struct(vec![
                lit(18).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
        ])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Tetracosahexaenylic acid (ω-3)
pub static C24U6Z6Z9Z12Z15Z18Z21: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(24).alias("Carbons"),
        concat_list([
            as_struct(vec![
                lit(6).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
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
            as_struct(vec![
                lit(18).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(21).alias("Index"),
                lit(1).alias("Isomerism"),
                lit(1).alias("Unsaturation"),
            ]),
        ])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Hyenic acid
pub static C25U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(25).alias("Carbons"),
        EMPTY_LIST.clone().alias("Unsaturated"),
    ]))
});

/// Cerotic acid
pub static C26U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(26).alias("Carbons"),
        EMPTY_LIST.clone().alias("Unsaturated"),
    ]))
});

/// Ximenic acid
pub static C26U1Z17: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(26).alias("Carbons"),
        concat_list([as_struct(vec![
            lit(17).alias("Index"),
            lit(1).alias("Isomerism"),
            lit(1).alias("Unsaturation"),
        ])])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Hexacosadienylic acid
pub static C26U2: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(26).alias("Carbons"),
        concat_list([
            as_struct(vec![
                lit(0).alias("Index"),
                lit(0).alias("Isomerism"),
                lit(0).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(0).alias("Index"),
                lit(0).alias("Isomerism"),
                lit(0).alias("Unsaturation"),
            ]),
        ])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Hexacosatrienylic acid
pub static C26U3: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(26).alias("Carbons"),
        concat_list([
            as_struct(vec![
                lit(0).alias("Index"),
                lit(0).alias("Isomerism"),
                lit(0).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(0).alias("Index"),
                lit(0).alias("Isomerism"),
                lit(0).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(0).alias("Index"),
                lit(0).alias("Isomerism"),
                lit(0).alias("Unsaturation"),
            ]),
        ])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Hexacosatetraenylic acid
pub static C26U4: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(26).alias("Carbons"),
        concat_list([
            as_struct(vec![
                lit(0).alias("Index"),
                lit(0).alias("Isomerism"),
                lit(0).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(0).alias("Index"),
                lit(0).alias("Isomerism"),
                lit(0).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(0).alias("Index"),
                lit(0).alias("Isomerism"),
                lit(0).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(0).alias("Index"),
                lit(0).alias("Isomerism"),
                lit(0).alias("Unsaturation"),
            ]),
        ])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Hexacosapentaenylic acid
pub static C26U5: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(26).alias("Carbons"),
        concat_list([
            as_struct(vec![
                lit(0).alias("Index"),
                lit(0).alias("Isomerism"),
                lit(0).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(0).alias("Index"),
                lit(0).alias("Isomerism"),
                lit(0).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(0).alias("Index"),
                lit(0).alias("Isomerism"),
                lit(0).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(0).alias("Index"),
                lit(0).alias("Isomerism"),
                lit(0).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(0).alias("Index"),
                lit(0).alias("Isomerism"),
                lit(0).alias("Unsaturation"),
            ]),
        ])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Hexacosahexaenylic acid
pub static C26U6: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(26).alias("Carbons"),
        concat_list([
            as_struct(vec![
                lit(0).alias("Index"),
                lit(0).alias("Isomerism"),
                lit(0).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(0).alias("Index"),
                lit(0).alias("Isomerism"),
                lit(0).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(0).alias("Index"),
                lit(0).alias("Isomerism"),
                lit(0).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(0).alias("Index"),
                lit(0).alias("Isomerism"),
                lit(0).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(0).alias("Index"),
                lit(0).alias("Isomerism"),
                lit(0).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(0).alias("Index"),
                lit(0).alias("Isomerism"),
                lit(0).alias("Unsaturation"),
            ]),
        ])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Carboceric acid
pub static C27U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(27).alias("Carbons"),
        EMPTY_LIST.clone().alias("Unsaturated"),
    ]))
});

/// Montanic acid
pub static C28U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(28).alias("Carbons"),
        EMPTY_LIST.clone().alias("Unsaturated"),
    ]))
});

/// Octacosenylic acid
pub static C28U1: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(28).alias("Carbons"),
        concat_list([as_struct(vec![
            lit(0).alias("Index"),
            lit(1).alias("Isomerism"),
            lit(1).alias("Unsaturation"),
        ])])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// --
pub static C28U2: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(28).alias("Carbons"),
        concat_list([
            as_struct(vec![
                lit(0).alias("Index"),
                lit(0).alias("Isomerism"),
                lit(0).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(0).alias("Index"),
                lit(0).alias("Isomerism"),
                lit(0).alias("Unsaturation"),
            ]),
        ])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Nonacosylic acid
pub static C29U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(29).alias("Carbons"),
        EMPTY_LIST.clone().alias("Unsaturated"),
    ]))
});

/// Melissic acid
pub static C30U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(30).alias("Carbons"),
        EMPTY_LIST.clone().alias("Unsaturated"),
    ]))
});

/// Lumequeic acid
pub static C30U1Z21: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(30).alias("Carbons"),
        concat_list([as_struct(vec![
            lit(0).alias("Index"),
            lit(1).alias("Isomerism"),
            lit(1).alias("Unsaturation"),
        ])])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

///
pub static C30U2: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(30).alias("Carbons"),
        concat_list([
            as_struct(vec![
                lit(0).alias("Index"),
                lit(0).alias("Isomerism"),
                lit(0).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(0).alias("Index"),
                lit(0).alias("Isomerism"),
                lit(0).alias("Unsaturation"),
            ]),
        ])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Henatriacontylic acid
pub static C31U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(31).alias("Carbons"),
        EMPTY_LIST.clone().alias("Unsaturated"),
    ]))
});

/// Lacceroic acid
pub static C32U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(32).alias("Carbons"),
        EMPTY_LIST.clone().alias("Unsaturated"),
    ]))
});

/// Dotriacontenylic acid
pub static C32U1: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(32).alias("Carbons"),
        concat_list([as_struct(vec![
            lit(0).alias("Index"),
            lit(1).alias("Isomerism"),
            lit(1).alias("Unsaturation"),
        ])])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// --
pub static C32U2: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(32).alias("Carbons"),
        concat_list([
            as_struct(vec![
                lit(0).alias("Index"),
                lit(0).alias("Isomerism"),
                lit(0).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(0).alias("Index"),
                lit(0).alias("Isomerism"),
                lit(0).alias("Unsaturation"),
            ]),
        ])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Psyllic acid
pub static C33U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(33).alias("Carbons"),
        EMPTY_LIST.clone().alias("Unsaturated"),
    ]))
});

/// Gheddic acid
pub static C34U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(34).alias("Carbons"),
        EMPTY_LIST.clone().alias("Unsaturated"),
    ]))
});

/// Tetratriacontenylic acid
pub static C34U1: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(34).alias("Carbons"),
        concat_list([as_struct(vec![
            lit(0).alias("Index"),
            lit(1).alias("Isomerism"),
            lit(1).alias("Unsaturation"),
        ])])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// --
pub static C34U2: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(34).alias("Carbons"),
        concat_list([
            as_struct(vec![
                lit(0).alias("Index"),
                lit(0).alias("Isomerism"),
                lit(0).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(0).alias("Index"),
                lit(0).alias("Isomerism"),
                lit(0).alias("Unsaturation"),
            ]),
        ])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// Ceroplastic acid
pub static C35U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(35).alias("Carbons"),
        EMPTY_LIST.clone().alias("Unsaturated"),
    ]))
});

/// Hexatriacontylic acid
pub static C36U0: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(36).alias("Carbons"),
        EMPTY_LIST.clone().alias("Unsaturated"),
    ]))
});

/// Hexatriacontenylic acid
pub static C36U1: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(36).alias("Carbons"),
        concat_list([as_struct(vec![
            lit(0).alias("Index"),
            lit(1).alias("Isomerism"),
            lit(1).alias("Unsaturation"),
        ])])
        .unwrap()
        .alias("Unsaturated"),
    ]))
});

/// --
pub static C36U2: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
    FattyAcidExpr(as_struct(vec![
        lit(36).alias("Carbons"),
        concat_list([
            as_struct(vec![
                lit(0).alias("Index"),
                lit(0).alias("Isomerism"),
                lit(0).alias("Unsaturation"),
            ]),
            as_struct(vec![
                lit(0).alias("Index"),
                lit(0).alias("Isomerism"),
                lit(0).alias("Unsaturation"),
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
