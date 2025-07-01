use super::*;

// type FattyAcidChunkedN: usize> = [(Option<Option<NonZeroI8>>, Option<&'static str>); N];

// const BU: (&str, FattyAcidChunked) = ("Bu", fatty_acid(C4).unwrap());
// const V: (&str, FattyAcidChunked) = ("V", fatty_acid(C5).unwrap());
// const CO: (&str, FattyAcidChunked) = ("Co", fatty_acid(C6).unwrap());
// const EN: (&str, FattyAcidChunked) = ("En", fatty_acid(C7).unwrap());
// const CY: (&str, FattyAcidChunked) = ("Cy", fatty_acid(C8).unwrap());
// const CA: (&str, FattyAcidChunked) = ("Ca", fatty_acid(C10).unwrap());
// const LA: (&str, FattyAcidChunked) = ("La", fatty_acid(C12).unwrap());
// const M: (&str, FattyAcidChunked) = ("M", fatty_acid(C14).unwrap());
// const P: (&str, FattyAcidChunked) = ("P", fatty_acid(C16).unwrap());
// const PO: (&str, FattyAcidChunked) = ("Po", fatty_acid(C16DC9).unwrap());
// const PE: (&str, FattyAcidChunked) = ("Pe", fatty_acid(C16DT9).unwrap());
// const S: (&str, FattyAcidChunked) = ("S", fatty_acid(C18).unwrap());
// const O: (&str, FattyAcidChunked) = ("O", fatty_acid(C18DC9).unwrap());
// const EL: (&str, FattyAcidChunked) = ("El", fatty_acid(C18DT9).unwrap());
// const L: (&str, FattyAcidChunked) = ("L", fatty_acid(C18DC9DC12).unwrap());
// const LN: (&str, FattyAcidChunked) = ("Ln", fatty_acid(C18DC6DC9DC12).unwrap());
// const GLN: (&str, FattyAcidChunked) = ("Gln", fatty_acid(C18DC6DC9DC12).unwrap());
// const EO: (&str, FattyAcidChunked) = ("Eo", fatty_acid(C18DC9DT11DT13).unwrap());
// const JA: (&str, FattyAcidChunked) = ("Ja", fatty_acid(C18DT9DT11DT13).unwrap());
// const CT: (&str, FattyAcidChunked) = ("Ct", fatty_acid(C18DT9DT11DC13).unwrap());
// const ST: (&str, FattyAcidChunked) = ("St", fatty_acid(C18DC6DC9DC12DC15).unwrap());
// const A: (&str, FattyAcidChunked) = ("A", fatty_acid(C20).unwrap());
// const G: (&str, FattyAcidChunked) = ("G", fatty_acid(C20DC9).unwrap());
// const GO: (&str, FattyAcidChunked) = ("Go", fatty_acid(C20DC11).unwrap());
// const AO: (&str, FattyAcidChunked) = ("Ao", fatty_acid(C20DC5DC8DC11DC14).unwrap());
// const EP: (&str, FattyAcidChunked) = ("Ep", fatty_acid(C20DC5DC8DC11DC14DC17).unwrap());
// const B: (&str, FattyAcidChunked) = ("B", fatty_acid(C22).unwrap());
// const E: (&str, FattyAcidChunked) = ("E", fatty_acid(C22DC13).unwrap());
// const DP: (&str, FattyAcidChunked) = ("Dp", fatty_acid(C22DC7DC10DC13DC16DC19).unwrap());
// const DH: (&str, FattyAcidChunked) = ("Dh", fatty_acid(C22DC4DC7DC10DC13DC16DC19).unwrap());
// const LG: (&str, FattyAcidChunked) = ("Lg", fatty_acid(C24).unwrap());
// const N: (&str, FattyAcidChunked) = ("N", fatty_acid(C24DC15).unwrap());

// fn bu() -> PolarsResult<(&'static str, FattyAcidChunked)> {
//     Ok(("Bu", FattyAcidChunked::try_from(C4)?))
// }

// fn v() -> PolarsResult<(&'static str, FattyAcidChunked)> {
//     Ok(("V", FattyAcidChunked::try_from(C5)?))
// }

// fn co() -> PolarsResult<(&'static str, FattyAcidChunked)> {
//     Ok(("Co", FattyAcidChunked::try_from(C6)?))
// }

// fn en() -> PolarsResult<(&'static str, FattyAcidChunked)> {
//     Ok(("En", FattyAcidChunked::try_from(C7)?))
// }

// fn cy() -> PolarsResult<(&'static str, FattyAcidChunked)> {
//     Ok(("Cy", FattyAcidChunked::try_from(C8)?))
// }

// fn ca() -> PolarsResult<(&'static str, FattyAcidChunked)> {
//     Ok(("Ca", FattyAcidChunked::try_from(C10)?))
// }

// fn la() -> PolarsResult<(&'static str, FattyAcidChunked)> {
//     Ok(("La", FattyAcidChunked::try_from(C12)?))
// }

// fn m() -> PolarsResult<(&'static str, FattyAcidChunked)> {
//     Ok(("M", FattyAcidChunked::try_from(C14)?))
// }

// fn p() -> PolarsResult<(&'static str, FattyAcidChunked)> {
//     Ok(("P", FattyAcidChunked::try_from(C16)?))
// }

// fn po() -> PolarsResult<(&'static str, FattyAcidChunked)> {
//     Ok(("Po", FattyAcidChunked::try_from(C16DC9)?))
// }

// fn pe() -> PolarsResult<(&'static str, FattyAcidChunked)> {
//     Ok(("Pe", FattyAcidChunked::try_from(C16DT9)?))
// }

// fn s() -> PolarsResult<(&'static str, FattyAcidChunked)> {
//     Ok(("S", FattyAcidChunked::try_from(C18)?))
// }

// fn o() -> PolarsResult<(&'static str, FattyAcidChunked)> {
//     Ok(("O", FattyAcidChunked::try_from(C18DC9)?))
// }

// fn el() -> PolarsResult<(&'static str, FattyAcidChunked)> {
//     Ok(("El", FattyAcidChunked::try_from(C18DT9)?))
// }

// fn l() -> PolarsResult<(&'static str, FattyAcidChunked)> {
//     Ok(("L", FattyAcidChunked::try_from(C18DC9DC12)?))
// }

// fn ln() -> PolarsResult<(&'static str, FattyAcidChunked)> {
//     Ok(("Ln", FattyAcidChunked::try_from(C18DC6DC9DC12)?))
// }

// fn gln() -> PolarsResult<(&'static str, FattyAcidChunked)> {
//     Ok(("Gln", FattyAcidChunked::try_from(C18DC6DC9DC12)?))
// }

// fn eo() -> PolarsResult<(&'static str, FattyAcidChunked)> {
//     Ok(("Eo", FattyAcidChunked::try_from(C18DC9DT11DT13)?))
// }

// fn ja() -> PolarsResult<(&'static str, FattyAcidChunked)> {
//     Ok(("Ja", FattyAcidChunked::try_from(C18DT9DT11DT13)?))
// }

// fn ct() -> PolarsResult<(&'static str, FattyAcidChunked)> {
//     Ok(("Ct", FattyAcidChunked::try_from(C18DT9DT11DC13)?))
// }

// fn st() -> PolarsResult<(&'static str, FattyAcidChunked)> {
//     Ok(("St", FattyAcidChunked::try_from(C18DC6DC9DC12DC15)?))
// }

// fn a() -> PolarsResult<(&'static str, FattyAcidChunked)> {
//     Ok(("A", FattyAcidChunked::try_from(C20)?))
// }

// fn g() -> PolarsResult<(&'static str, FattyAcidChunked)> {
//     Ok(("G", FattyAcidChunked::try_from(C20DC9)?))
// }

// fn go() -> PolarsResult<(&'static str, FattyAcidChunked)> {
//     Ok(("Go", FattyAcidChunked::try_from(C20DC11)?))
// }

// fn ao() -> PolarsResult<(&'static str, FattyAcidChunked)> {
//     Ok(("Ao", FattyAcidChunked::try_from(C20DC5DC8DC11DC14)?))
// }

// fn ep() -> PolarsResult<(&'static str, FattyAcidChunked)> {
//     Ok(("Ep", FattyAcidChunked::try_from(C20DC5DC8DC11DC14DC17)?))
// }

// fn b() -> PolarsResult<(&'static str, FattyAcidChunked)> {
//     Ok(("B", FattyAcidChunked::try_from(C22)?))
// }

// fn e() -> PolarsResult<(&'static str, FattyAcidChunked)> {
//     Ok(("E", FattyAcidChunked::try_from(C22DC13)?))
// }

// fn dp() -> PolarsResult<(&'static str, FattyAcidChunked)> {
//     Ok(("Dp", FattyAcidChunked::try_from(C22DC7DC10DC13DC16DC19)?))
// }

// fn dh() -> PolarsResult<(&'static str, FattyAcidChunked)> {
//     Ok(("Dh", FattyAcidChunked::try_from(C22DC4DC7DC10DC13DC16DC19)?))
// }

// fn lg() -> PolarsResult<(&'static str, FattyAcidChunked)> {
//     Ok(("Lg", FattyAcidChunked::try_from(C24)?))
// }

// fn n() -> PolarsResult<(&'static str, FattyAcidChunked)> {
//     Ok(("N", FattyAcidChunked::try_from(C24DC15)?))
// }

mod fatty_acid;
mod triacylglycerol;
