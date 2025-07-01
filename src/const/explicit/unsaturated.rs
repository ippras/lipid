#![rustfmt::skip]

use crate::prelude::*;
use fatty_acid_macro::fatty_acid;
use polars::prelude::*;
use std::sync::LazyLock;

/// [Palmitoleic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C16DC9: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C16 { 9 => DC }).unwrap());

/// [Palmitelaidic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C16DT9: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C16 { 9 => DT }).unwrap());

/// [Oleic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18DC9: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C18 { 9 => DC }).unwrap());

/// [Elaidic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18DT9: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C18 { 9 => DT }).unwrap());

/// [Linoleic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18DC9DC12: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C18 { 9 => DC, 12 => DC }).unwrap());

/// [α-Linolenic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18DC9DC12DC15: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C18 { 9 => DC, 12 => DC, 15 => DC }).unwrap());

/// [γ-Linolenic acid, GLA](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18DC6DC9DC12: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C18 { 6 => DC, 9 => DC, 12 => DC }).unwrap());

/// [Jacaric acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18DC8DT10DC12: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C18 { 8 => DC, 10 => DT, 12 => DC }).unwrap());

/// [α-Eleostearic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18DC9DT11DT13: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C18 { 9 => DC, 11 => DT, 13 => DT }).unwrap());

/// [β-Eleostearic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18DT9DT11DT13: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C18 { 9 => DT, 11 => DT, 13 => DT }).unwrap());

/// [Catalpic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18DT9DT11DC13: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C18 { 9 => DT, 11 => DT, 13 => DC }).unwrap());

/// [Stearidonic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18DC6DC9DC12DC15: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C18 { 6 => DC, 9 => DC, 12 => DC, 15 => DC }).unwrap());

/// [Gadoleic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20DC9: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C20 { 9 => DC }).unwrap());

/// [Gondoic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20DC11: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C20 { 11 => DC }).unwrap());

/// [DihomoLinoleic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20DC11DC14: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C20 { 11 => DC, 14 => DC }).unwrap());

/// [Bis-homo-α-Linolenic acid, DTTA](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20DC11DC14DC17: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C20 { 11 => DC, 14 => DC, 17 => DC }).unwrap());

/// [Bis-homo-γ-Linolenic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20DC8DC11DC14: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C20 { 8 => DC, 11 => DC, 14 => DC }).unwrap());

/// [Mead Acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20DC5DC8DC11: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C20 { 5 => DC, 8 => DC, 11 => DC }).unwrap());

/// [Arachidonic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20DC5DC8DC11DC14: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C20 { 5 => DC, 8 => DC, 11 => DC, 14 => DC }).unwrap());

/// [Eicosatetraenoic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20DC8DC11DC14DC17: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C20 { 8 => DC, 11 => DC, 14 => DC, 17 => DC }).unwrap());

/// [Eicosapentaenoic EPA](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20DC5DC8DC11DC14DC17: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C20 { 5 => DC, 8 => DC, 11 => DC, 14 => DC, 17 => DC }).unwrap());

/// [Erucic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C22DC13: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C22 { 13 => DC }).unwrap());

/// [Docosadienoic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C22DC13DC16: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C22 { 13 => DC, 16 => DC }).unwrap());

/// [Eranthic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C22DC5DC13DC16: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C22 { 5 => DC, 13 => DC, 16 => DC }).unwrap());

/// [Adrenic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C22DC7DC10DC13DC16: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C22 { 7 => DC, 10 => DC, 13 => DC, 16 => DC }).unwrap());

/// [DPA](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C22DC7DC10DC13DC16DC19: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C22 { 7 => DC, 10 => DC, 13 => DC, 16 => DC, 19 => DC }).unwrap());

/// [DHA](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C22DC4DC7DC10DC13DC16DC19: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C22 { 4 => DC, 7 => DC, 10 => DC, 13 => DC, 16 => DC, 19 => DC }).unwrap());

/// [Nervonic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C24DC15: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C24 { 15 => DC }).unwrap());

/// [Tetracosadienoic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C24DC15DC18: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C24 { 15 => DC, 18 => DC }).unwrap());

/// [Tetracosatrienylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C24DC12DC15DC18: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C24 { 12 => DC, 15 => DC, 18 => DC }).unwrap());

/// [Tetracosatetraenylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C24DC9DC12DC15DC18: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C24 { 9 => DC, 12 => DC, 15 => DC, 18 => DC }).unwrap());

/// [Tetracosapentaenylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C24DC6DC9DC12DC15DC18: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C24 { 6 => DC, 9 => DC, 12 => DC, 15 => DC, 18 => DC }).unwrap());

/// [Tetracosahexaenylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C24DC6DC9DC12DC15DC18DC21: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C24 { 6 => DC, 9 => DC, 12 => DC, 15 => DC, 18 => DC, 21 => DC }).unwrap());

/// [Ximenic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C26DC17: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C26 { 17 => DC }).unwrap());

/// [Lumequeic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C30DC21: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C30 { 21 => DC }).unwrap());
