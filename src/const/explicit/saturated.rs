use crate::prelude::*;
use fatty_acid_macro::fatty_acid;
use polars::prelude::*;
use std::sync::LazyLock;

/// [Butyric acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C4: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C4 {}).unwrap());

/// [Valeric acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C5: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C5 {}).unwrap());

/// [Caproic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C6: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C6 {}).unwrap());

/// [Enanthic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C7: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C7 {}).unwrap());

/// [Caprylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C8: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C8 {}).unwrap());

/// [Pelargonic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C9: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C9 {}).unwrap());

/// [Capric acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C10: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C10 {}).unwrap());

/// [Undecylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C11: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C11 {}).unwrap());

/// [Lauric acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C12: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C12 {}).unwrap());

/// [Tridecylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C13: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C13 {}).unwrap());

/// [Myristic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C14: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C14 {}).unwrap());

/// [Pentadecylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C15: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C15 {}).unwrap());

/// [Palmitic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C16: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C16 {}).unwrap());

/// [Margaric acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C17: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C17 {}).unwrap());

/// [Stearic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C18: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C18 {}).unwrap());

/// [Nonadecylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C19: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C19 {}).unwrap());

/// [Arachidic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C20: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C20 {}).unwrap());

/// [Heneicosylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C21: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C21 {}).unwrap());

/// [Behenic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C22: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C22 {}).unwrap());

/// [Tricosylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C23: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C23 {}).unwrap());

/// [Lignoceric acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C24: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C24 {}).unwrap());

/// [Hyenic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C25: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C25 {}).unwrap());

/// [Cerotic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C26: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C26 {}).unwrap());

/// [Carboceric acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C27: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C27 {}).unwrap());

/// [Montanic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C28: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C28 {}).unwrap());

/// [Nonacosylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C29: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C29 {}).unwrap());

/// [Melissic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C30: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C30 {}).unwrap());

/// [Henatriacontylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C31: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C31 {}).unwrap());

/// [Lacceroic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C32: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C32 {}).unwrap());

/// [Psyllic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C33: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C33 {}).unwrap());

/// [Gheddic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C34: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C34 {}).unwrap());

/// [Ceroplastic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C35: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C35 {}).unwrap());

/// [Hexatriacontylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub const C36: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C36 {}).unwrap());
