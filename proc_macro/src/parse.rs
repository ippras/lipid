use self::keywords::{A, C, O, T, U};
use crate::error::{
    Error::{IndicesLengthGreaterThanRange, StartGreaterThanStop},
    error,
};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote, quote_spanned};
use syn::{
    LitInt, Token, braced,
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    spanned::Spanned,
};

mod keywords {
    // proc_easy::easy_token!(I);
    proc_easy::easy_token!(A);
    proc_easy::easy_token!(C);
    proc_easy::easy_token!(O);
    proc_easy::easy_token!(T);
    proc_easy::easy_token!(U);
}

proc_easy::easy_argument_group! {
    enum UnsaturatedBound {
        Acetylenic(A),
        Cis(C),
        Olefinic(O),
        Trans(T),
        Unsaturated(U),
    }
}

// Range `{ start, end }`
pub(super) struct Range {
    pub(super) start: u8,
    pub(super) end: u8,
}

impl Parse for Range {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _brace = braced!(content in input);

        let start: LitInt = content.parse()?;
        let _comma = content.parse::<Token![,]>()?;
        let end: LitInt = content.parse()?;

        let start = start.base10_parse::<u8>()?;
        let end = end.base10_parse::<u8>()?;

        if start > end {
            return error!(StartGreaterThanStop: content);
        }

        Ok(Self { start, end })
    }
}

impl ToTokens for Range {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { start, end } = self;
        quote! {
            ::polars::datatypes::AnyValue::StructOwned(Box::new((
                vec![::polars::datatypes::AnyValue::UInt8(#start), ::polars::datatypes::AnyValue::UInt8(#end)],
                vec![::lipid::field!(START), ::lipid::field!(STOP)],
            )))
        }
        .to_tokens(tokens)
    }
}

/// Carbons
pub(super) struct Carbons {
    pub(super) ident: C,
    pub(super) range: Range,
}

impl Parse for Carbons {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse()?;
        let range = input.parse()?;
        Ok(Self { ident, range })
    }
}

// impl ToTokens for Carbons {
//     fn to_tokens(&self, tokens: &mut TokenStream) {
//         let Self { ident, range } = self;
//         quote_spanned! {
//             ident.span() => ::polars::datatypes::AnyValue::StructOwned(Box::new((
//                 vec![#range],
//                 vec![::lipid::field!(C)],
//             )))
//         }
//         .to_tokens(tokens)
//     }
// }

/// Unsaturated
pub(super) struct Unsaturated {
    pub(super) ident: U,
    pub(super) range: Range,
    pub(super) indices: Indices,
}

impl Parse for Unsaturated {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse()?;
        let range: Range = input.parse()?;
        let indices: Indices = input.parse()?;
        if indices.0.len() > range.end as _ {
            return error!(IndicesLengthGreaterThanRange: range);
        }
        Ok(Self {
            ident,
            range,
            indices,
        })
    }
}

impl ToTokens for Unsaturated {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            ident,
            range,
            indices,
        } = self;
        quote_spanned! {
            ident.span() => ::polars::datatypes::AnyValue::StructOwned(Box::new((
                vec![#range, #indices],
                vec![::lipid::field!(LENGTH), ::lipid::field!(INDICES)],
            )))
        }
        .to_tokens(tokens)
    }
}

/// Indices
pub(super) struct Indices(pub(super) Punctuated<Index, Token![,]>);

impl Parse for Indices {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        braced!(content in input);
        Ok(Self(content.parse_terminated(Index::parse, Token![,])?))
    }
}

impl ToTokens for Indices {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let length = self.0.len();
        // let c_stop = self.c_stop;

        let mut index_tokens = vec![];
        let mut triple_tokens = vec![];
        let mut parity_tokens = vec![];

        for index in &self.0 {
            let offset = index.offset;

            if delta != 0 {
                index_tokens.push(quote!(Some(#offset)));
            } else {
                index_tokens.push(quote!(None::<u8>));
            }

            match &index.kind {
                UnsaturatedBound::Acetylenic(_) => {
                    triple_tokens.push(quote!(Some(true)));
                    parity_tokens.push(quote!(None::<bool>));
                }
                UnsaturatedBound::Cis(_) => {
                    triple_tokens.push(quote!(Some(false)));
                    parity_tokens.push(quote!(Some(false)));
                }
                UnsaturatedBound::Olefinic(_) => {
                    triple_tokens.push(quote!(Some(false)));
                    parity_tokens.push(quote!(None::<bool>));
                }
                UnsaturatedBound::Trans(_) => {
                    triple_tokens.push(quote!(Some(false)));
                    parity_tokens.push(quote!(Some(true)));
                }
                UnsaturatedBound::Unsaturated(_) => {
                    triple_tokens.push(quote!(None::<bool>));
                    parity_tokens.push(quote!(None::<bool>));
                }
            }
        }

        quote! {
            ::polars::datatypes::AnyValue::List({
                let index = {
                    let mut builder = ::polars::chunked_array::builder::PrimitiveChunkedBuilder::<::polars::datatypes::UInt8Type>::new(::lipid::field!(INDEX).into(), #length);
                    for val in [#(#index_tokens),*] {
                        builder.append_option(val);
                    }
                    builder.finish()
                };
                let triple = {
                    let mut builder = ::polars::chunked_array::builder::BooleanChunkedBuilder::new(::lipid::field!(TRIPLE).into(), #length);
                    for val in [#(#triple_tokens),*] {
                        builder.append_option(val);
                    }
                    builder.finish()
                };
                let parity = {
                    let mut builder = ::polars::chunked_array::builder::BooleanChunkedBuilder::new(::lipid::field!(PARITY).into(), #length);
                    for val in [#(#parity_tokens),*] {
                        builder.append_option(val);
                    }
                    builder.finish()
                };
                let item = ::polars::datatypes::StructChunked::from_series(
                    ::polars::datatypes::PlSmallStr::EMPTY,
                    #length,
                    [
                        index.into_series(),
                        triple.into_series(),
                        parity.into_series(),
                    ]
                    .iter(),
                ).unwrap();
                item.into_series()
            })
        }.to_tokens(tokens);
    }
}

pub(super) struct Index {
    pub(super) offset: i8,
    pub(super) kind: UnsaturatedBound,
}

impl Parse for Index {
    fn parse(input: ParseStream) -> Result<Self> {
        let offset: LitInt = input.parse()?;
        Ok(Index {
            offset: offset.base10_parse::<i8>()?,
            kind: input.parse()?,
        })
    }
}

/// Input
pub(super) struct Input {
    pub(super) c: Carbons,
    pub(super) u: Unsaturated,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            c: input.parse()?,
            u: input.parse()?,
        })
    }
}
