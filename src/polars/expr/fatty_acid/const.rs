use super::FattyAcidExpr;
use crate::{fatty_acid::r#const::C18U2DC9DC12, prelude::*};
use polars::prelude::*;
use std::sync::LazyLock;

fn temp<'a>(iter: impl IntoIterator<Item = &'a str>) -> PolarsResult<Expr> {
    Ok(lit(Scalar::new(
        DataType::List(Box::new(BOUND_DATA_TYPE.clone())),
        AnyValue::List(Series::from_iter(iter).cast(&BOUND_DATA_TYPE)?),
    )))
}

macro_rules! fatty_acid_expr {
    ($id:ident) => {
        Ok(lit(Scalar::new(
            DataType::List(Box::new(BOUND_DATA_TYPE.clone())),
            AnyValue::List(Series::from_iter(iter).cast(&BOUND_DATA_TYPE)?),
        )))
    };
}

macro_rules! fatty_acid_expr {
    ($(#[$outer:meta])* C $c:literal U $u:literal $($i:tt)*) => {
        paste::paste! {
            $(#[$outer])*
            pub static [<C $c U $u $($i)*>]: LazyLock<FattyAcidExpr> = LazyLock::new(|| {
                FattyAcidExpr(as_struct(vec![
                    lit($c).alias("Carbons"),
                    if $u == 0 {
                        EMPTY_LIST.clone()
                    } else {
                        #[allow(unused_mut)]
                        let mut unsaturated: Vec<Expr> = Vec::with_capacity($u);
                        unsaturated_expr!(unsaturated, $($i)*)
                    }.alias("Unsaturated"),
                ]))
            });
        }
    };
}

macro_rules! unsaturated_expr {
    ($unsaturated:ident, Z $z:literal $($i:tt)*) => {{
        $unsaturated.push(as_struct(vec![
            lit($z).alias("Index"),
            lit(1).alias("Isomerism"),
            lit(1).alias("Unsaturation"),
        ]));
        unsaturated_expr!($unsaturated, $($i)*)
    }};
    ($unsaturated:ident, E $e:literal $($i:tt)*) => {{
        $unsaturated.push(as_struct(vec![
            lit($e).alias("Index"),
            lit(-1).alias("Isomerism"),
            lit(1).alias("Unsaturation"),
        ]));
        unsaturated_expr!($unsaturated, $($i)*)
    }};
    ($unsaturated:ident,) => {
        concat_list($unsaturated).unwrap()
    };
}

macro_rules! source {
    (byrdwell.com) => {
        "[[byrdwell.com]](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)"
    };
}

fatty_acid_expr!(
    #[doc = concat!("Butyric acid ", source!(byrdwell.com))]
    C 4 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Valeric acid ", source!(byrdwell.com))]
    C 5 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Caproic acid ", source!(byrdwell.com))]
    C 6 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Enanthic acid ", source!(byrdwell.com))]
    C 7 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Caprylic acid ", source!(byrdwell.com))]
    C 8 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Pelargonic acid ", source!(byrdwell.com))]
    C 9 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Capric acid ", source!(byrdwell.com))]
    C 10 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Undecylic acid ", source!(byrdwell.com))]
    C 11 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Lauric acid ", source!(byrdwell.com))]
    C 12 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Tridecylic acid ", source!(byrdwell.com))]
    C 13 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Myristic acid ", source!(byrdwell.com))]
    C 14 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Pentadecylic acid ", source!(byrdwell.com))]
    C 15 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Palmitic acid ", source!(byrdwell.com))]
    C 16 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Palmitoleic acid (n-7) ", source!(byrdwell.com))]
    C 16 U 1 Z 9
);

fatty_acid_expr!(
    #[doc = concat!("Palmitelaidic acid (n-7) ", source!(byrdwell.com))]
    C 16 U 1 E 9
);

fatty_acid_expr!(
    #[doc = concat!("Margaric acid ", source!(byrdwell.com))]
    C 17 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Stearic acid ", source!(byrdwell.com))]
    C 18 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Oleic acid (n-9) ", source!(byrdwell.com))]
    C 18 U 1 Z 9
);

fatty_acid_expr!(
    #[doc = concat!("Elaidic acid (n-9) ", source!(byrdwell.com))]
    C 18 U 1 E 9
);

fatty_acid_expr!(
    #[doc = concat!("Linoleic acid (ω-6) ", source!(byrdwell.com))]
    C 18 U 2 Z 9 Z 12
);

fatty_acid_expr!(
    #[doc = concat!("α-Linolenic acid (ω-3) ", source!(byrdwell.com))]
    C 18 U 3 Z 9 Z 12 Z 15
);

fatty_acid_expr!(
    #[doc = concat!("γ-Linolenic acid (ω-6), GLA ", source!(byrdwell.com))]
    C 18 U 3
);

fatty_acid_expr!(
    #[doc = concat!("α-Eleostearic acid ", source!(byrdwell.com))]
    C 18 U 3 Z 9 E 11 E 13
);

fatty_acid_expr!(
    #[doc = concat!("β-Eleostearic acid ", source!(byrdwell.com))]
    C 18 U 3 E 9 E 11 E 13
);

fatty_acid_expr!(
    #[doc = concat!("Jacaric acid ", source!(byrdwell.com))]
    C 18 U 3 Z 8 E 10 Z 12
);

fatty_acid_expr!(
    #[doc = concat!("Catalpic acid ", source!(byrdwell.com))]
    C 18 U 3 E 9 E 11 Z 13
);

fatty_acid_expr!(
    #[doc = concat!("Stearidonic acid (ω-3) ", source!(byrdwell.com))]
    C 18 U 4 Z 6 Z 9 Z 12 Z 15
);

fatty_acid_expr!(
    #[doc = concat!("Nonadecylic acid ", source!(byrdwell.com))]
    C 19 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Arachidic acid ", source!(byrdwell.com))]
    C 20 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Gadoleic acid (n-11) ", source!(byrdwell.com))]
    C 20 U 1 Z 9
);

fatty_acid_expr!(
    #[doc = concat!("Gondoic acid (n-9) ", source!(byrdwell.com))]
    C 20 U 1 Z 11
);

fatty_acid_expr!(
    #[doc = concat!("DihomoLinoleic acid (ω-6) ", source!(byrdwell.com))]
    C 20 U 2 Z 11 Z 14
);

fatty_acid_expr!(
    #[doc = concat!("Bis-homo-α-Linolenic acid (ω-3), ETA ", source!(byrdwell.com))]
    C 20 U 3 Z 11 Z 14 Z 17
);

fatty_acid_expr!(
    #[doc = concat!("Bis-homo-γ-Linolenic acid (ω-6) ", source!(byrdwell.com))]
    C 20 U 3 Z 8 Z 11 Z 14
);

fatty_acid_expr!(
    #[doc = concat!("Mead Acid ", source!(byrdwell.com))]
    C 20 U 3 Z 5 Z 8 Z 11
);

fatty_acid_expr!(
    #[doc = concat!("Arachidonic acid (ω-6) ", source!(byrdwell.com))]
    C 20 U 4 Z 5 Z 8 Z 11 Z 14
);

fatty_acid_expr!(
    #[doc = concat!("Eicosatetraenoic acid (ω-3) ", source!(byrdwell.com))]
    C 20 U 4 Z 8 Z 11 Z 14 Z 17
);

fatty_acid_expr!(
    #[doc = concat!("EPA (ω-3) ", source!(byrdwell.com))]
    C 20 U 5 Z 5 Z 8 Z 11 Z 14 Z 17
);

fatty_acid_expr!(
    #[doc = concat!("Heneicosylic acid ", source!(byrdwell.com))]
    C 21 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Behenic acid ", source!(byrdwell.com))]
    C 22 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Erucic acid (n-9) ", source!(byrdwell.com))]
    C 22 U 1 Z 13
);

fatty_acid_expr!(
    #[doc = concat!("Docosadienoic acid (ω-6) ", source!(byrdwell.com))]
    C 22 U 2 Z 13 Z 16
);

fatty_acid_expr!(
    #[doc = concat!("Eranthic acid (ω-6) ", source!(byrdwell.com))]
    C 22 U 3 Z 5 Z 13 Z 16
);

fatty_acid_expr!(
    #[doc = concat!("Adrenic acid (ω-6) ", source!(byrdwell.com))]
    C 22 U 4 Z 7 Z 10 Z 13 Z 16
);

fatty_acid_expr!(
    #[doc = concat!("DPA (ω-3) ", source!(byrdwell.com))]
    C 22 U 5 Z 7 Z 10 Z 13 Z 16 Z 19
);

fatty_acid_expr!(
    #[doc = concat!("DHA (ω-3) ", source!(byrdwell.com))]
    C 22 U 6 Z 4 Z 7 Z 10 Z 13 Z 16 Z 19
);

fatty_acid_expr!(
    #[doc = concat!("Tricosylic acid ", source!(byrdwell.com))]
    C 23 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Lignoceric acid ", source!(byrdwell.com))]
    C 24 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Nervonic acid (n-9) ", source!(byrdwell.com))]
    C 24 U 1 Z 15
);

fatty_acid_expr!(
    #[doc = concat!("Tetracosadienoic acid (ω-6) ", source!(byrdwell.com))]
    C 24 U 2 Z 15 Z 18
);

fatty_acid_expr!(
    #[doc = concat!("Tetracosatrienylic acid ", source!(byrdwell.com))]
    C 24 U 3 Z 12 Z 15 Z 18
);

fatty_acid_expr!(
    #[doc = concat!("Tetracosatetraenylic acid ", source!(byrdwell.com))]
    C 24 U 4 Z 9 Z 12 Z 15 Z 18
);

fatty_acid_expr!(
    #[doc = concat!("Tetracosapentaenylic acid ", source!(byrdwell.com))]
    C 24 U 5 Z 6 Z 9 Z 12 Z 15 Z 18
);

fatty_acid_expr!(
    #[doc = concat!("Tetracosahexaenylic acid (ω-3) ", source!(byrdwell.com))]
    C 24 U 6 Z 6 Z 9 Z 12 Z 15 Z 18 Z 21
);

fatty_acid_expr!(
    #[doc = concat!("Hyenic acid ", source!(byrdwell.com))]
    C 25 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Cerotic acid ", source!(byrdwell.com))]
    C 26 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Ximenic acid ", source!(byrdwell.com))]
    C 26 U 1 Z 17
);

fatty_acid_expr!(
    #[doc = concat!("Hexacosadienylic acid ", source!(byrdwell.com))]
    C 26 U 2
);

fatty_acid_expr!(
    #[doc = concat!("Hexacosatrienylic acid ", source!(byrdwell.com))]
    C 26 U 3
);

fatty_acid_expr!(
    #[doc = concat!("Hexacosatetraenylic acid ", source!(byrdwell.com))]
    C 26 U 4
);

fatty_acid_expr!(
    #[doc = concat!("Hexacosapentaenylic acid ", source!(byrdwell.com))]
    C 26 U 5
);

fatty_acid_expr!(
    #[doc = concat!("Hexacosahexaenylic acid ", source!(byrdwell.com))]
    C 26 U 6
);

fatty_acid_expr!(
    #[doc = concat!("Carboceric acid ", source!(byrdwell.com))]
    C 27 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Montanic acid ", source!(byrdwell.com))]
    C 28 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Octacosenylic acid ", source!(byrdwell.com))]
    C 28 U 1
);

fatty_acid_expr!(
    #[doc = concat!("", source!(byrdwell.com))]
    C 28 U 2
);

fatty_acid_expr!(
    #[doc = concat!("Nonacosylic acid ", source!(byrdwell.com))]
    C 29 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Melissic acid ", source!(byrdwell.com))]
    C 30 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Lumequeic acid ", source!(byrdwell.com))]
    C 30 U 1 Z 21
);

fatty_acid_expr!(
    #[doc = concat!("", source!(byrdwell.com))]
    C 30 U 2
);

fatty_acid_expr!(
    #[doc = concat!("Henatriacontylic acid ", source!(byrdwell.com))]
    C 31 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Lacceroic acid ", source!(byrdwell.com))]
    C 32 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Dotriacontenylic acid ", source!(byrdwell.com))]
    C 32 U 1
);

fatty_acid_expr!(
    #[doc = concat!("", source!(byrdwell.com))]
    C 32 U 2
);

fatty_acid_expr!(
    #[doc = concat!("Psyllic acid ", source!(byrdwell.com))]
    C 33 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Gheddic acid ", source!(byrdwell.com))]
    C 34 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Tetratriacontenylic acid ", source!(byrdwell.com))]
    C 34 U 1
);

fatty_acid_expr!(
    #[doc = concat!("", source!(byrdwell.com))]
    C 34 U 2
);

fatty_acid_expr!(
    #[doc = concat!("Ceroplastic acid ", source!(byrdwell.com))]
    C 35 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Hexatriacontylic acid ", source!(byrdwell.com))]
    C 36 U 0
);

fatty_acid_expr!(
    #[doc = concat!("Hexatriacontenylic acid ", source!(byrdwell.com))]
    C 36 U 1
);

fatty_acid_expr!(
    #[doc = concat!("", source!(byrdwell.com))]
    C 36 U 2
);

static EMPTY_LIST: LazyLock<Expr> = LazyLock::new(|| {
    lit(Scalar::new(
        DataType::List(Box::new(DataType::Null)),
        AnyValue::List(Series::new_empty(PlSmallStr::EMPTY, &DataType::Null)),
    ))
});
