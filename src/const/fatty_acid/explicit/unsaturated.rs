#![rustfmt::skip]

use crate::prelude::*;
use fatty_acid_proc_macro::fatty_acid;
use polars::prelude::*;
use std::sync::LazyLock;

/// [Palmitoleic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C16U1C9: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 16 U 1 { 9: C }).unwrap());

/// [Palmitelaidic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C16U1T9: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 16 U 1 { 9: T }).unwrap());

/// [Oleic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18U1C9: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 18 U 1 { 9: C }).unwrap());

/// [Elaidic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18U1T9: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 18 U 1 { 9: T }).unwrap());

/// [Linoleic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18U2C9C12: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 18 U 2 { 9: C, 12: C }).unwrap());

/// [α-Linolenic acid; ALA](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18U3C9C12C15: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 18 U 3 { 9: C, 12: C, 15: C }).unwrap());

/// [γ-Linolenic acid; GLA](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18U3C6C9C12: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 18 U 3 { 6: C, 9: C, 12: C }).unwrap());

/// [Jacaric acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18U3C8T10C12: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 18 U 3 { 8: C, 10: T, 12: C }).unwrap());

/// [α-Eleostearic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18U3C9T11T13: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 18 U 3 { 9: C, 11: T, 13: T }).unwrap());

/// [β-Eleostearic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18U3T9T11T13: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 18 U 3 { 9: T, 11: T, 13: T }).unwrap());

/// [Catalpic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18U3T9T11C13: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 18 U 3 { 9: T, 11: T, 13: C }).unwrap());

/// [Stearidonic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18U4C6C9C12C15: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 18 U 4 { 6: C, 9: C, 12: C, 15: C }).unwrap());

/// [Gadoleic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20U1C9: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 20 U 1 { 9: C }).unwrap());

/// [Gondoic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20U1C11: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 20 U 1 { 11: C }).unwrap());

/// [DihomoLinoleic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20U2C11C14: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 20 U 2 { 11: C, 14: C }).unwrap());

/// [Bis-homo-α-Linolenic acid; TTA](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20U3C11C14C17: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 20 U 3 { 11: C, 14: C, 17: C }).unwrap());

/// [Bis-homo-γ-Linolenic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20U3C8C11C14: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 20 U 3 { 8: C, 11: C, 14: C }).unwrap());

/// [Mead acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20U3C5C8C11: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 20 U 3 { 5: C, 8: C, 11: C }).unwrap());

/// [Arachidonic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20U4C5C8C11C14: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 20 U 4 { 5: C, 8: C, 11: C, 14: C }).unwrap());

/// [Eicosatetraenoic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20U4C8C11C14C17: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 20 U 4 { 8: C, 11: C, 14: C, 17: C }).unwrap());

/// [Eicosapentaenoic acid; EPA](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20U5C5C8C11C14C17: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 20 U 5 { 5: C, 8: C, 11: C, 14: C, 17: C }).unwrap());

/// [Erucic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C22U1C13: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 22 U 1 { 13: C }).unwrap());

/// [Docosadienoic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C22U2C13C16: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 22 U 2 { 13: C, 16: C }).unwrap());

/// [Eranthic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C22U3C5C13C16: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 22 U 3 { 5: C, 13: C, 16: C }).unwrap());

/// [Adrenic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C22U4C7C10C13C16: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 22 U 4 { 7: C, 10: C, 13: C, 16: C }).unwrap());

/// [Docosapentaenoic acid; DPA](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C22U5C7C10C13C16C19: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 22 U 5 { 7: C, 10: C, 13: C, 16: C, 19: C }).unwrap());

/// [Docosahexaenoic acid; DHA](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C22U6C4C7C10C13C16C19: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 22 U 6 { 4: C, 7: C, 10: C, 13: C, 16: C, 19: C }).unwrap());

/// [Nervonic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C24U1C15: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 24 U 1 { 15: C }).unwrap());

/// [Tetracosadienoic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C24U2C15C18: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 24 U 2 { 15: C, 18: C }).unwrap());

/// [Tetracosatrienylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C24U3C12C15C18: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 24 U 3 { 12: C, 15: C, 18: C }).unwrap());

/// [Tetracosatetraenylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C24U4C9C12C15C18: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 24 U 4 { 9: C, 12: C, 15: C, 18: C }).unwrap());

/// [Tetracosapentaenylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C24U5C6C9C12C15C18: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 24 U 5 { 6: C, 9: C, 12: C, 15: C, 18: C }).unwrap());

/// [Tetracosahexaenylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C24U6C6C9C12C15C18C21: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 24 U 6 { 6: C, 9: C, 12: C, 15: C, 18: C, 21: C }).unwrap());

/// [Ximenic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C26U1C17: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 26 U 1 { 17: C }).unwrap());

/// [Lumequeic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C30U1C21: LazyLock<AnyValue> = LazyLock::new(|| fatty_acid!(C 30 U 1 { 21: C }).unwrap());