use crate::prelude::*;
use fatty_acid_proc_macro::fatty_acid;
use polars::prelude::*;
use std::sync::LazyLock;

/// [Butyric acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C4U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 4 U 0 {}).unwrap());

/// [Valeric acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C5U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 5 U 0 {}).unwrap());

/// [Caproic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C6U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 6 U 0 {}).unwrap());

/// [Enanthic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C7U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 7 U 0 {}).unwrap());

/// [Caprylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C8U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 8 U 0 {}).unwrap());

/// [Pelargonic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C9U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 9 U 0 {}).unwrap());

/// [Capric acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C10U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 10 U 0 {}).unwrap());

/// [Undecylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C11U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 11 U 0 {}).unwrap());

/// [Lauric acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C12U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 12 U 0 {}).unwrap());

/// [Tridecylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C13U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 13 U 0 {}).unwrap());

/// [Myristic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C14U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 14 U 0 {}).unwrap());

/// [Pentadecylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C15U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 15 U 0 {}).unwrap());

/// [Palmitic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C16U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 16 U 0 {}).unwrap());

/// [Margaric acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C17U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 17 U 0 {}).unwrap());

/// [Stearic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 18 U 0 {}).unwrap());

/// [Nonadecylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C19U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 19 U 0 {}).unwrap());

/// [Arachidic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 20 U 0 {}).unwrap());

/// [Heneicosylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C21U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 21 U 0 {}).unwrap());

/// [Behenic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C22U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 22 U 0 {}).unwrap());

/// [Tricosylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C23U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 23 U 0 {}).unwrap());

/// [Lignoceric acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C24U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 24 U 0 {}).unwrap());

/// [Hyenic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C25U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 25 U 0 {}).unwrap());

/// [Cerotic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C26U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 26 U 0 {}).unwrap());

/// [Carboceric acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C27U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 27 U 0 {}).unwrap());

/// [Montanic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C28U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 28 U 0 {}).unwrap());

/// [Nonacosylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C29U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 29 U 0 {}).unwrap());

/// [Melissic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C30U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 30 U 0 {}).unwrap());

/// [Henatriacontylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C31U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 31 U 0 {}).unwrap());

/// [Lacceroic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C32U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 32 U 0 {}).unwrap());

/// [Psyllic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C33U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 33 U 0 {}).unwrap());

/// [Gheddic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C34U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 34 U 0 {}).unwrap());

/// [Ceroplastic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C35U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 35 U 0 {}).unwrap());

/// [Hexatriacontylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C36U0: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 36 U 0 {}).unwrap());
