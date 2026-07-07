use crate::{
    error::Error::*,
    parse::{Indices, Unsaturated},
};
use error::error;
use parse::{Index, Input};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse::Result, parse_macro_input, spanned::Spanned};

mod error;
mod parse;

#[proc_macro]
pub fn fatty_acid(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as Input);

    match try_input(input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn try_input(input: Input) -> Result<TokenStream> {
    let Input {
        carbons,
        unsaturated,
        indices,
    } = input;

    let carbons_count = carbons.value as usize;
    let unsaturated_count = unsaturated.value as usize;
    if carbons_count <= unsaturated_count {
        return Err(UnsaturatedCountGreaterOrEqualThanCarbonsCount {
            span: carbons.ident.span(),
            carbons_count,
            unsaturated_count,
        })?;
    }
    let indices_length = indices.values.len();
    if indices_length > unsaturated_count {
        return Err(IndicesLengthGreaterThanUnsaturatedCount {
            span: indices.span,
            unsaturated_count,
            indices_length,
        })?;
    }

    let tokens = quote! {{
        (|| -> PolarsResult<_> {
            // let index = {
            //     let mut builder = PrimitiveChunkedBuilder::<UInt8Type>::new(INDEX.into(), #length);
            //     for index in [#(#index),*] {
            //         builder.append_option(index);
            //     }
            //     builder.finish()
            // };
            // let triple = {
            //     let mut builder = BooleanChunkedBuilder::new(TRIPLE.into(), #length);
            //     for triple in [#(#triple),*] {
            //         builder.append_option(triple);
            //     }
            //     builder.finish()
            // };
            // let parity = {
            //     let mut builder = BooleanChunkedBuilder::new(PARITY.into(), #length);
            //     for parity in [#(#parity),*] {
            //         builder.append_option(parity);
            //     }
            //     builder.finish()
            // };
            // let item = StructChunked::from_series(
            //     PlSmallStr::EMPTY,
            //     #length,
            //     [
            //         index.into_series(),
            //         triple.into_series(),
            //         parity.into_series(),
            //     ]
            //     .iter(),
            // )?;

            Ok(::polars::datatypes::AnyValue::StructOwned(Box::new((
                vec![#carbons, #unsaturated, #indices],
                vec![
                    ::lipid::field!(CARBON),
                    ::lipid::field!(UNSATURATED),
                    ::lipid::field!(INDICES),
                ],
            ))));
        })()
    }};

    Ok(tokens)
}
