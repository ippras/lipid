use super::FattyAcidExpr;
use polars::prelude::*;
use std::sync::LazyLock;

macro_rules! fatty_acid_expr {
    ($(#[$outer:meta])* C $c:literal U $u:literal $($i:tt)*) => {
        paste::paste! {
            $(#[$outer])*
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

fatty_acid_expr!(#[doc = "Butyric acid"] C 4 U 0);

fatty_acid_expr!(#[doc = "Valeric acid"] C 5 U 0);

fatty_acid_expr!(#[doc = "Caproic acid"] C 6 U 0);

fatty_acid_expr!(#[doc = "Enanthic acid"] C 7 U 0);

fatty_acid_expr!(#[doc = "Caprylic acid"] C 8 U 0);

fatty_acid_expr!(#[doc = "Pelargonic acid"] C 9 U 0);

fatty_acid_expr!(#[doc = "Capric acid"] C 10 U 0);

fatty_acid_expr!(#[doc = "Undecylic acid"] C 11 U 0);

fatty_acid_expr!(#[doc = "Lauric acid"] C 12 U 0);

fatty_acid_expr!(#[doc = "Tridecylic acid"] C 13 U 0);

fatty_acid_expr!(#[doc = "Myristic acid"] C 14 U 0);

fatty_acid_expr!(#[doc = "Pentadecylic acid"] C 15 U 0);

fatty_acid_expr!(#[doc = "Palmitic acid"] C 16 U 0);

fatty_acid_expr!(#[doc = "Palmitoleic acid (n-7)"] C 16 U 1 Z 9);

fatty_acid_expr!(#[doc = "Palmitelaidic acid (n-7)"] C 16 U 1 E 9);

fatty_acid_expr!(#[doc = "Margaric acid"] C 17 U 0);

fatty_acid_expr!(#[doc = "Stearic acid"] C 18 U 0);

fatty_acid_expr!(#[doc = "Oleic acid (n-9)"] C 18 U 1 Z 9);

fatty_acid_expr!(#[doc = "Elaidic acid (n-9)"] C 18 U 1 E 9);

fatty_acid_expr!(#[doc = "Linoleic acid (ω-6)"] C 18 U 2 Z 9 Z 12);

fatty_acid_expr!(#[doc = "α-Linolenic acid (ω-3)"] C 18 U 3 Z 9 Z 12 Z 15);

fatty_acid_expr!(#[doc = "γ-Linolenic acid (ω-6), GLA"] C 18 U 3);

fatty_acid_expr!(#[doc = "α-Eleostearic acid"] C 18 U 3 Z 9 E 11 E 13);

fatty_acid_expr!(#[doc = "β-Eleostearic acid"] C 18 U 3 E 9 E 11 E 13);

fatty_acid_expr!(#[doc = "Jacaric acid"] C 18 U 3 Z 8 E 10 Z 12);

fatty_acid_expr!(#[doc = "Catalpic acid"] C 18 U 3 E 9 E 11 Z 13);

fatty_acid_expr!(#[doc = "Stearidonic acid (ω-3)"] C 18 U 4 Z 6 Z 9 Z 12 Z 15);

fatty_acid_expr!(#[doc = "Nonadecylic acid"] C 19 U 0);

fatty_acid_expr!(#[doc = "Arachidic acid"] C 20 U 0);

fatty_acid_expr!(#[doc = "Gadoleic acid (n-11)"] C 20 U 1 Z 9);

fatty_acid_expr!(#[doc = "Gondoic acid (n-9)"] C 20 U 1 Z 11);

fatty_acid_expr!(#[doc = "DihomoLinoleic acid (ω-6)"] C 20 U 2 Z 11 Z 14);

fatty_acid_expr!(#[doc = "Bis-homo-α-Linolenic acid (ω-3), ETA"] C 20 U 3 Z 11 Z 14 Z 17);

fatty_acid_expr!(#[doc = "Bis-homo-γ-Linolenic acid (ω-6)"] C 20 U 3 Z 8 Z 11 Z 14);

fatty_acid_expr!(#[doc = "Mead Acid"] C 20 U 3 Z 5 Z 8 Z 11);

fatty_acid_expr!(#[doc = "Arachidonic acid (ω-6)"] C 20 U 4 Z 5 Z 8 Z 11 Z 14);

fatty_acid_expr!(#[doc = "Eicosatetraenoic acid (ω-3)"] C 20 U 4 Z 8 Z 11 Z 14 Z 17);

fatty_acid_expr!(#[doc = "EPA (ω-3)"] C 20 U 5 Z 5 Z 8 Z 11 Z 14 Z 17);

fatty_acid_expr!(#[doc = "Heneicosylic acid"] C 21 U 0);

fatty_acid_expr!(#[doc = "Behenic acid"] C 22 U 0);

fatty_acid_expr!(#[doc = "Erucic acid (n-9)"] C 22 U 1 Z 13);

fatty_acid_expr!(#[doc = "Docosadienoic acid (ω-6)"] C 22 U 2 Z 13 Z 16);

fatty_acid_expr!(#[doc = "Eranthic acid (ω-6)"] C 22 U 3 Z 5 Z 13 Z 16);

fatty_acid_expr!(#[doc = "Adrenic acid (ω-6)"] C 22 U 4 Z 7 Z 10 Z 13 Z 16);

fatty_acid_expr!(#[doc = "DPA (ω-3)"] C 22 U 5 Z 7 Z 10 Z 13 Z 16 Z 19);

fatty_acid_expr!(#[doc = "DHA (ω-3)"] C 22 U 6 Z 4 Z 7 Z 10 Z 13 Z 16 Z 19);

fatty_acid_expr!(#[doc = "Tricosylic acid"] C 23 U 0);

fatty_acid_expr!(#[doc = "Lignoceric acid"] C 24 U 0);

fatty_acid_expr!(#[doc = "Nervonic acid (n-9)"] C 24 U 1 Z 15);

fatty_acid_expr!(#[doc = "Tetracosadienoic acid (ω-6)"] C 24 U 2 Z 15 Z 18);

fatty_acid_expr!(#[doc = "Tetracosatrienylic acid"] C 24 U 3 Z 12 Z 15 Z 18);

fatty_acid_expr!(#[doc = "Tetracosatetraenylic acid"] C 24 U 4 Z 9 Z 12 Z 15 Z 18);

fatty_acid_expr!(#[doc = "Tetracosapentaenylic acid"] C 24 U 5 Z 6 Z 9 Z 12 Z 15 Z 18);

fatty_acid_expr!(#[doc = "Tetracosahexaenylic acid (ω-3)"] C 24 U 6 Z 6 Z 9 Z 12 Z 15 Z 18 Z 21);

fatty_acid_expr!(#[doc = "Hyenic acid"] C 25 U 0);

fatty_acid_expr!(#[doc = "Cerotic acid"] C 26 U 0);

fatty_acid_expr!(#[doc = "Ximenic acid"] C 26 U 1 Z 17);

fatty_acid_expr!(#[doc = "Hexacosadienylic acid"] C 26 U 2);

fatty_acid_expr!(#[doc = "Hexacosatrienylic acid"] C 26 U 3);

fatty_acid_expr!(#[doc = "Hexacosatetraenylic acid"] C 26 U 4);

fatty_acid_expr!(#[doc = "Hexacosapentaenylic acid"] C 26 U 5);

fatty_acid_expr!(#[doc = "Hexacosahexaenylic acid"] C 26 U 6);

fatty_acid_expr!(#[doc = "Carboceric acid"] C 27 U 0);

fatty_acid_expr!(#[doc = "Montanic acid"] C 28 U 0);

fatty_acid_expr!(#[doc = "Octacosenylic acid"] C 28 U 1);

fatty_acid_expr!(#[doc = ""] C 28 U 2);

fatty_acid_expr!(#[doc = "Nonacosylic acid"] C 29 U 0);

fatty_acid_expr!(#[doc = "Melissic acid"] C 30 U 0);

fatty_acid_expr!(#[doc = "Lumequeic acid"] C 30 U 1 Z 21);

fatty_acid_expr!(#[doc = ""] C 30 U 2);

fatty_acid_expr!(#[doc = "Henatriacontylic acid"] C 31 U 0);

fatty_acid_expr!(#[doc = "Lacceroic acid"] C 32 U 0);

fatty_acid_expr!(#[doc = "Dotriacontenylic acid"] C 32 U 1);

fatty_acid_expr!(#[doc = ""] C 32 U 2);

fatty_acid_expr!(#[doc = "Psyllic acid"] C 33 U 0);

fatty_acid_expr!(#[doc = "Gheddic acid"] C 34 U 0);

fatty_acid_expr!(#[doc = "Tetratriacontenylic acid"] C 34 U 1);

fatty_acid_expr!(#[doc = ""] C 34 U 2);

fatty_acid_expr!(#[doc = "Ceroplastic acid"] C 35 U 0);

fatty_acid_expr!(#[doc = "Hexatriacontylic acid"] C 36 U 0);

fatty_acid_expr!(#[doc = "Hexatriacontenylic acid"] C 36 U 1);

fatty_acid_expr!(#[doc = ""] C 36 U 2);

static EMPTY_LIST: LazyLock<Expr> = LazyLock::new(|| {
    lit(Scalar::new(
        DataType::List(Box::new(DataType::Null)),
        AnyValue::List(Series::new_empty(PlSmallStr::EMPTY, &DataType::Null)),
    ))
});
