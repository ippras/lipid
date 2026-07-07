use self::keywords::{A, C, O, T, U};
use crate::error::{Error::IndicesLengthGreaterThanUnsaturatedCount, error};
use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{ToTokens, quote, quote_spanned};
use syn::{
    LitInt, Token, braced, parenthesized,
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
    pub(super) enum UnsaturatedBound {
        Acetylenic(A),
        Cis(C),
        Olefinic(O),
        Trans(T),
        Unsaturated(U),
    }
}

/// Carbons
pub(super) struct Carbons {
    pub(super) ident: C,
    pub(super) value: u8,
}

impl Parse for Carbons {
    fn parse(input: ParseStream) -> Result<Self> {
        // input.step(|cursor| {
        //     if let Some((tt, next)) = cursor.token_tree() {
        //         if let TokenTree::Punct(punct) = tt {
        //             // punct.as_char() возвращает char
        //             // Возвращаем символ и новый курсор (next), что сдвигает поток
        //             return Ok((punct.as_char(), next));
        //         }
        //     }
        //     Err(cursor.error("ожидался символ пунктуации"))
        // });
        let ident = input.parse()?;
        let value = input.parse::<LitInt>()?;
        Ok(Self {
            ident,
            value: value.base10_parse::<u8>()?,
        })
    }
}

impl ToTokens for Carbons {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { ident, value } = self;
        quote_spanned! {
            ident.span() => ::polars::datatypes::AnyValue::UInt8(#value)
        }
        .to_tokens(tokens)
    }
}

/// Unsaturated
pub(super) struct Unsaturated {
    pub(super) ident: U,
    pub(super) value: u8,
}

impl Parse for Unsaturated {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse()?;
        let value = input.parse::<LitInt>()?;
        Ok(Self {
            ident,
            value: value.base10_parse::<u8>()?,
        })
    }
}

impl ToTokens for Unsaturated {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { ident, value } = self;
        quote_spanned! {
            ident.span() => ::polars::datatypes::AnyValue::UInt8(#value)
        }
        .to_tokens(tokens)
    }
}

/// Indices
pub(super) struct Indices {
    pub(super) span: Span,
    pub(super) values: Punctuated<Index, Token![,]>,
}

impl Parse for Indices {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        braced!(content in input);
        Ok(Self {
            span: content.span(),
            values: content.parse_terminated(Index::parse, Token![,])?,
        })
    }
}

impl ToTokens for Indices {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let length = self.values.len();

        let mut index_tokens = vec![];
        let mut triple_tokens = vec![];
        let mut parity_tokens = vec![];

        for index in &self.values {
            let offset = index.offset;

            if offset != 0 {
                index_tokens.push(quote!(Some(#offset)));
            } else {
                index_tokens.push(quote!(None::<u8>));
            }

            match &index.unsaturated_bound {
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
                    for optional_value in [#(#index_tokens),*] {
                        builder.append_option(optional_value);
                    }
                    builder.finish()
                };
                let triple = {
                    let mut builder = ::polars::chunked_array::builder::BooleanChunkedBuilder::new(::lipid::field!(TRIPLE).into(), #length);
                    for optional_value in [#(#triple_tokens),*] {
                        builder.append_option(optional_value);
                    }
                    builder.finish()
                };
                let parity = {
                    let mut builder = ::polars::chunked_array::builder::BooleanChunkedBuilder::new(::lipid::field!(PARITY).into(), #length);
                    for optional_value in [#(#parity_tokens),*] {
                        builder.append_option(optional_value);
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
    pub(super) unsaturated_bound: UnsaturatedBound,
}

impl Parse for Index {
    fn parse(input: ParseStream) -> Result<Self> {
        let offset: LitInt = input.parse()?;
        let _: syn::Token![:] = input.parse()?;
        Ok(Index {
            offset: offset.base10_parse::<i8>()?,
            unsaturated_bound: input.parse()?,
        })
    }
}

/// Input
pub(super) struct Input {
    pub(super) carbons: Carbons,
    pub(super) unsaturated: Unsaturated,
    pub(super) indices: Indices,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            carbons: input.parse()?,
            unsaturated: input.parse()?,
            indices: input.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;
    use syn::parse_str;

    #[test]
    fn test_parse_carbons() {
        let c: Carbons = parse_str("C 18").unwrap();
        assert_eq!(c.value, 18);
    }

    #[test]
    fn test_parse_unsaturated() {
        let u: Unsaturated = parse_str("U 1").unwrap();
        assert_eq!(u.value, 1);
    }

    #[test]
    fn test_parse_index_delta_cis() {
        let index: Index = parse_str("9: C").unwrap();
        assert_eq!(index.offset, 9);
        assert!(matches!(index.unsaturated_bound, UnsaturatedBound::Cis(_)));
    }

    #[test]
    fn test_parse_index_omega() {
        // Проверка отрицательного смещения (i8)
        let index: Index = parse_str("-3: U").unwrap();
        assert_eq!(index.offset, -3);
        assert!(matches!(
            index.unsaturated_bound,
            UnsaturatedBound::Unsaturated(_)
        ));
    }

    #[test]
    fn test_parse_full_input() {
        let input: Input =
            parse_str("C 18 U 3 { 9: C, 12: C, 15: T }").expect("Failed to parse full input");

        assert_eq!(input.carbons.value, 18);
        assert_eq!(input.unsaturated.value, 3);
        assert_eq!(input.indices.values.len(), 3);

        let first_index = &input.indices.values[0];
        assert_eq!(first_index.offset, 9);
        assert!(matches!(
            first_index.unsaturated_bound,
            UnsaturatedBound::Cis(_)
        ));
    }

    // #[test]
    // fn test_parse_unsaturated_invalid_length() {
    //     // Максимальное количество связей (end) = 1, но передано 2 индекса
    //     let result: Result<Unsaturated> = parse_str("U{ 1, 1 }{ 9 => C, 12 => C }");
    //     assert!(
    //         matches!(result, Err(error) if error.to_string() == Error::IndicesLengthGreaterThanRange.message())
    //     );
    // }

    // #[test]
    // fn test_parse_full_input() {
    //     let input: Input = parse_str("C16..18U3..3{ 9 => C, 12 => C, 15 => T }")
    //         .expect("Failed to parse full input");

    //     assert_eq!(input.carbons.value.start, 16);
    //     assert_eq!(input.carbons.value.end, 18);

    //     assert_eq!(input.unsaturated.range.start, 3);
    //     assert_eq!(input.unsaturated.range.end, 3);
    //     assert_eq!(input.unsaturated.indices.0.len(), 3);

    //     let first_index = input.unsaturated.indices.0.first().unwrap();
    //     assert_eq!(first_index.offset, 9);
    //     assert!(matches!(
    //         first_index.unsaturated_bound,
    //         UnsaturatedBound::Cis(_)
    //     ));
    // }
}
