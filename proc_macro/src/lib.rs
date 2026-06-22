use crate::error::Error::*;
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

    // let c_start = c.range.start.base10_parse::<u8>().expect("parse C start");
    // let c_end = c.range.end.base10_parse::<u8>().expect("parse C end");
    // let u_start = u.range.start.base10_parse::<u8>().expect("parse U start");
    // let u_end = u.range.end.base10_parse::<u8>().expect("parse U end");

    // let c_struct = quote_spanned! {c => {
    //     polars::datatypes::AnyValue::StructOwned(Box::new((
    //         vec![AnyValue::UInt8(#c_start), AnyValue::UInt8(#c_stop)],
    //         vec![::fatty_acid_proc_macro::field!(START), ::fatty_acid_proc_macro::field!(STOP)],
    //     )));
    // }};
}

fn try_input(input: Input) -> Result<TokenStream> {
    let Input { c, u } = input;

    let length = i.terminated.len();
    let mut index = vec![];
    let mut triple = vec![];
    let mut parity = vec![];

    for Index { offset, kind, .. } in i.terminated {
        // ВАЖНО: Для омега-нотации (отрицательные смещения) я использую `c_end`.
        // Если логика требует использования `c_start`, измените переменную ниже.
        let delta = match offset {
            omega @ ..0 => c_end - omega.unsigned_abs(),
            delta @ 0.. => delta as u8,
        };

        if delta != 0 {
            index.push(quote!(Some(#delta)));
        } else {
            index.push(quote!(None));
        }

        match kind.name_display() {
            "C" => {
                triple.push(quote!(Some(false)));
                parity.push(quote!(Some(false)));
            }
            "T" => {
                triple.push(quote!(Some(false)));
                parity.push(quote!(Some(true)));
            }
            "O" => {
                triple.push(quote!(Some(false)));
                parity.push(quote!(None));
            }
            "A" => {
                triple.push(quote!(Some(true)));
                parity.push(quote!(None));
            }
            "U" => {
                triple.push(quote!(None));
                parity.push(quote!(None));
            }
            kind => panic!("unexpected entry kind {kind}"),
        }
    }

    let tokens = quote! {{
        (|| -> PolarsResult<_> {
            let index = {
                let mut builder = PrimitiveChunkedBuilder::<UInt8Type>::new(INDEX.into(), #length);
                for index in [#(#index),*] {
                    builder.append_option(index);
                }
                builder.finish()
            };
            let triple = {
                let mut builder = BooleanChunkedBuilder::new(TRIPLE.into(), #length);
                for triple in [#(#triple),*] {
                    builder.append_option(triple);
                }
                builder.finish()
            };
            let parity = {
                let mut builder = BooleanChunkedBuilder::new(PARITY.into(), #length);
                for parity in [#(#parity),*] {
                    builder.append_option(parity);
                }
                builder.finish()
            };
            let item = StructChunked::from_series(
                PlSmallStr::EMPTY,
                #length,
                [
                    index.into_series(),
                    triple.into_series(),
                    parity.into_series(),
                ]
                .iter(),
            )?;

            Ok(AnyValue::StructOwned(Box::new((
                vec![
                    #c,
                    #u,
                    AnyValue::List(item.into_series()),
                ],
                vec![
                    field!(C_START),
                    field!(C_END),
                    field!(U_START),
                    field!(U_END),
                    field!(::lipid::r#const::INDICES)
                ],
            ))))
        })()
    }};

    Ok(tokens)
}
