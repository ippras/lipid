use crate::{error::Error::*, parse::Unsaturated};
use error::error;
use parse::{Index, Input};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse::Result, parse_macro_input};

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
    let Input { c, u } = input;

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

            Ok(AnyValue::StructOwned(Box::new(vec![#c, #u])))
        })()
    }};

    Ok(tokens)
}
