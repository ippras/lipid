#![rustfmt::skip]

use crate::prelude::*;

/// [Palmitoleic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C16DC9: [&str; 15] = [S, S, S, S, S, S, S, S, DC, S, S, S, S, S, S];

/// [Palmitelaidic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C16DT9: [&str; 15] = [S, S, S, S, S, S, S, S, DT, S, S, S, S, S, S];

/// [Oleic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18DC9: [&str; 17] = [S, S, S, S, S, S, S, S, DC, S, S, S, S, S, S, S, S];

/// [Elaidic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18DT9: [&str; 17] = [S, S, S, S, S, S, S, S, DT, S, S, S, S, S, S, S, S];

/// [Linoleic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18DC9DC12: [&str; 17] = [S, S, S, S, S, S, S, S, DC, S, S, DC, S, S, S, S, S];

/// [α-Linolenic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18DC9DC12DC15: [&str; 17] = [S, S, S, S, S, S, S, S, DC, S, S, DC, S, S, DC, S, S];

/// [γ-Linolenic acid, GLA](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18DC6DC9DC12: [&str; 17] = [S, S, S, S, S, DC, S, S, DC, S, S, DC, S, S, S, S, S];

/// [Jacaric acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18DC8DT10DC12: [&str; 17] = [S, S, S, S, S, S, S, DC, S, DT, S, DC, S, S, S, S, S];

/// [α-Eleostearic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18DC9DT11DT13: [&str; 17] = [S, S, S, S, S, S, S, S, DC, S, DT, S, DT, S, S, S, S];

/// [β-Eleostearic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18DT9DT11DT13: [&str; 17] = [S, S, S, S, S, S, S, S, DT, S, DT, S, DT, S, S, S, S];

/// [Catalpic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18DT9DT11DC13: [&str; 17] = [S, S, S, S, S, S, S, S, DT, S, DT, S, DC, S, S, S, S];

/// [Stearidonic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C18DC6DC9DC12DC15: [&str; 17] = [S, S, S, S, S, DC, S, S, DC, S, S, DC, S, S, DC, S, S];

/// [Gadoleic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20DC9: [&str; 19] = [S, S, S, S, S, S, S, S, DC, S, S, S, S, S, S, S, S, S, S];

/// [Gondoic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20DC11: [&str; 19] = [S, S, S, S, S, S, S, S, S, S, DC, S, S, S, S, S, S, S, S];

/// [DihomoLinoleic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20DC11DC14: [&str; 19] = [S, S, S, S, S, S, S, S, S, S, DC, S, S, DC, S, S, S, S, S];

/// [Bis-homo-α-Linolenic acid, DTTA](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20DC11DC14DC17: [&str; 19] = [S, S, S, S, S, S, S, S, S, S, DC, S, S, DC, S, S, DC, S, S];

/// [Bis-homo-γ-Linolenic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20DC8DC11DC14: [&str; 19] = [S, S, S, S, S, S, S, DC, S, S, DC, S, S, DC, S, S, S, S, S];

/// [Mead Acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20DC5DC8DC11: [&str; 19] = [S, S, S, S, DC, S, S, DC, S, S, DC, S, S, S, S, S, S, S, S];

/// [Arachidonic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20DC5DC8DC11DC14: [&str; 19] = [S, S, S, S, DC, S, S, DC, S, S, DC, S, S, DC, S, S, S, S, S];

/// [Eicosatetraenoic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20DC8DC11DC14DC17: [&str; 19] = [S, S, S, S, S, S, S, DC, S, S, DC, S, S, DC, S, S, DC, S, S];

/// [Eicosapentaenoic EPA](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C20DC5DC8DC11DC14DC17: [&str; 19] = [S, S, S, S, DC, S, S, DC, S, S, DC, S, S, DC, S, S, DC, S, S];

/// [Erucic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C22DC13: [&str; 21] = [S, S, S, S, S, S, S, S, S, S, S, S, DC, S, S, S, S, S, S, S, S];

/// [Docosadienoic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C22DC13DC16: [&str; 21] = [S, S, S, S, S, S, S, S, S, S, S, S, DC, S, S, DC, S, S, S, S, S];

/// [Eranthic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C22DC5DC13DC16: [&str; 21] = [S, S, S, S, DC, S, S, S, S, S, S, S, DC, S, S, DC, S, S, S, S, S];

/// [Adrenic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C22DC7DC10DC13DC16: [&str; 21] = [S, S, S, S, S, S, DC, S, S, DC, S, S, DC, S, S, DC, S, S, S, S, S];

/// [DPA](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C22DC7DC10DC13DC16DC19: [&str; 21] = [S, S, S, S, S, S, DC, S, S, DC, S, S, DC, S, S, DC, S, S, DC, S, S];

/// [DHA](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C22DC4DC7DC10DC13DC16DC19: [&str; 21] = [S, S, S, DC, S, S, DC, S, S, DC, S, S, DC, S, S, DC, S, S, DC, S, S];

/// [Nervonic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C24DC15: [&str; 23] = [S, S, S, S, S, S, S, S, S, S, S, S, S, S, DC, S, S, S, S, S, S, S, S];

/// [Tetracosadienoic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C24DC15DC18: [&str; 23] = [S, S, S, S, S, S, S, S, S, S, S, S, S, S, DC, S, S, DC, S, S, S, S, S];

/// [Tetracosatrienylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C24DC12DC15DC18: [&str; 23] = [S, S, S, S, S, S, S, S, S, S, S, DC, S, S, DC, S, S, DC, S, S, S, S, S];

/// [Tetracosatetraenylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C24DC9DC12DC15DC18: [&str; 23] = [S, S, S, S, S, S, S, S, DC, S, S, DC, S, S, DC, S, S, DC, S, S, S, S, S];

/// [Tetracosapentaenylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C24DC6DC9DC12DC15DC18: [&str; 23] = [S, S, S, S, S, DC, S, S, DC, S, S, DC, S, S, DC, S, S, DC, S, S, S, S, S];

/// [Tetracosahexaenylic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C24DC6DC9DC12DC15DC18DC21: [&str; 23] = [S, S, S, S, S, DC, S, S, DC, S, S, DC, S, S, DC, S, S, DC, S, S, DC, S, S];

/// [Ximenic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C26DC17: [&str; 25] = [S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, DC, S, S, S, S, S, S, S, S];

/// [Lumequeic acid](https://byrdwell.com/Triacylglycerols/FattyAcids.htm)
pub static C30DC21: [&str; 29] = [S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, DC, S, S, S, S, S, S, S, S];
