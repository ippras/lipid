#![rustfmt::skip]

use crate::prelude::*;
use fatty_acid_macro::fatty_acid;
use polars::prelude::*;
use std::sync::LazyLock;

/// [Palmitoleic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C16C9: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C16 { 9 => C }).unwrap());

/// [Palmitelaidic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C16T9: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C16 { 9 => T }).unwrap());

/// [Oleic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18C9: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C18 { 9 => C }).unwrap());

/// [Elaidic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18T9: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C18 { 9 => T }).unwrap());

/// [Linoleic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18C9C12: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C18 { 9 => C, 12 => C }).unwrap());

/// [α-Linolenic acid; ALA](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18C9C12C15: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C18 { 9 => C, 12 => C, 15 => C }).unwrap());

/// [γ-Linolenic acid; GLA](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18C6C9C12: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C18 { 6 => C, 9 => C, 12 => C }).unwrap());

/// [Jacaric acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18C8T10C12: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C18 { 8 => C, 10 => T, 12 => C }).unwrap());

/// [α-Eleostearic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18C9T11T13: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C18 { 9 => C, 11 => T, 13 => T }).unwrap());

/// [β-Eleostearic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18T9T11T13: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C18 { 9 => T, 11 => T, 13 => T }).unwrap());

/// [Catalpic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18T9T11C13: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C18 { 9 => T, 11 => T, 13 => C }).unwrap());

/// [Stearidonic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18C6C9C12C15: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C18 { 6 => C, 9 => C, 12 => C, 15 => C }).unwrap());

/// [Gadoleic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20C9: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C20 { 9 => C }).unwrap());

/// [Gondoic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20C11: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C20 { 11 => C }).unwrap());

/// [DihomoLinoleic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20C11C14: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C20 { 11 => C, 14 => C }).unwrap());

/// [Bis-homo-α-Linolenic acid; TTA](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20C11C14C17: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C20 { 11 => C, 14 => C, 17 => C }).unwrap());

/// [Bis-homo-γ-Linolenic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20C8C11C14: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C20 { 8 => C, 11 => C, 14 => C }).unwrap());

/// [Mead acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20C5C8C11: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C20 { 5 => C, 8 => C, 11 => C }).unwrap());

/// [Arachidonic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20C5C8C11C14: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C20 { 5 => C, 8 => C, 11 => C, 14 => C }).unwrap());

/// [Eicosatetraenoic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20C8C11C14C17: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C20 { 8 => C, 11 => C, 14 => C, 17 => C }).unwrap());

/// [Eicosapentaenoic acid; EPA](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20C5C8C11C14C17: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C20 { 5 => C, 8 => C, 11 => C, 14 => C, 17 => C }).unwrap());

/// [Erucic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C22C13: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C22 { 13 => C }).unwrap());

/// [Docosadienoic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C22C13C16: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C22 { 13 => C, 16 => C }).unwrap());

/// [Eranthic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C22C5C13C16: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C22 { 5 => C, 13 => C, 16 => C }).unwrap());

/// [Adrenic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C22C7C10C13C16: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C22 { 7 => C, 10 => C, 13 => C, 16 => C }).unwrap());

/// [Docosapentaenoic acid; DPA](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C22C7C10C13C16C19: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C22 { 7 => C, 10 => C, 13 => C, 16 => C, 19 => C }).unwrap());

/// [Docosahexaenoic acid; DHA](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C22C4C7C10C13C16C19: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C22 { 4 => C, 7 => C, 10 => C, 13 => C, 16 => C, 19 => C }).unwrap());

/// [Nervonic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C24C15: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C24 { 15 => C }).unwrap());

/// [Tetracosadienoic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C24C15C18: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C24 { 15 => C, 18 => C }).unwrap());

/// [Tetracosatrienylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C24C12C15C18: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C24 { 12 => C, 15 => C, 18 => C }).unwrap());

/// [Tetracosatetraenylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C24C9C12C15C18: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C24 { 9 => C, 12 => C, 15 => C, 18 => C }).unwrap());

/// [Tetracosapentaenylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C24C6C9C12C15C18: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C24 { 6 => C, 9 => C, 12 => C, 15 => C, 18 => C }).unwrap());

/// [Tetracosahexaenylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C24C6C9C12C15C18C21: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C24 { 6 => C, 9 => C, 12 => C, 15 => C, 18 => C, 21 => C }).unwrap());

/// [Ximenic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C26C17: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C26 { 17 => C }).unwrap());

/// [Lumequeic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C30C21: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C30 { 21 => C }).unwrap());
