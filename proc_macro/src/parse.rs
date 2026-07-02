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
    pub(super) enum UnsaturatedBound {
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
        let start: LitInt = input.parse()?;
        let _dot = input.parse::<Token![.]>()?;
        let _dot = input.parse::<Token![.]>()?;
        let _eq = input.parse::<Token![=]>()?;
        let end: LitInt = input.parse()?;

        let start = start.base10_parse::<u8>()?;
        let end = end.base10_parse::<u8>()?;

        if start > end {
            return error!(StartGreaterThanStop: start);
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
                vec![::lipid::field!(START), ::lipid::field!(END)],
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

impl ToTokens for Carbons {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { ident, range } = self;
        quote_spanned! {
            ident.span() => ::polars::datatypes::AnyValue::StructOwned(Box::new((
                vec![#range],
                vec![::lipid::field!(CARBON)],
            )))
        }
        .to_tokens(tokens)
    }
}

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
        if indices.0.len() > range.start as _ {
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
                vec![::lipid::field!(RANGE), ::lipid::field!(INDICES)],
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

        let mut index_tokens = vec![];
        let mut triple_tokens = vec![];
        let mut parity_tokens = vec![];

        for index in &self.0 {
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
        let _: syn::Token![=>] = input.parse()?;
        Ok(Index {
            offset: offset.base10_parse::<i8>()?,
            unsaturated_bound: input.parse()?,
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

#[cfg(test)]
mod tests {
    use crate::error::Error;

    use super::*;
    use syn::parse_str;

    #[test]
    fn test_parse_range_valid() {
        let range: Range = parse_str("16..=18").expect("Failed to parse valid range");
        assert_eq!(range.start, 16);
        assert_eq!(range.end, 18);
    }

    #[test]
    fn test_parse_range_invalid_start_greater_than_stop() {
        let result: Result<Range> = parse_str("20..=18");
        assert!(result.is_err());
        assert!(
            matches!(result, Err(error) if error.to_string() == Error::StartGreaterThanStop.message())
        );
    }

    #[test]
    fn test_parse_index_cis() {
        let index: Index = parse_str("9 => C").unwrap();
        assert_eq!(index.offset, 9);
        assert!(matches!(index.unsaturated_bound, UnsaturatedBound::Cis(_)));
    }

    #[test]
    fn test_parse_index_omega() {
        // Проверка отрицательного смещения (i8)
        let index: Index = parse_str("-3 => U").unwrap();
        assert_eq!(index.offset, -3);
        assert!(matches!(
            index.unsaturated_bound,
            UnsaturatedBound::Unsaturated(_)
        ));
    }

    #[test]
    fn test_parse_unsaturated_valid() {
        let u: Unsaturated = parse_str("U 2..=3{ 9 => C, 12 => C }").unwrap();
        assert_eq!(u.range.start, 2);
        assert_eq!(u.range.end, 3);
        assert_eq!(u.indices.0.len(), 2);
    }

    #[test]
    fn test_parse_unsaturated_invalid_length() {
        // Максимальное количество связей (end) = 1, но передано 2 индекса
        let result: Result<Unsaturated> = parse_str("U{ 1, 1 }{ 9 => C, 12 => C }");
        assert!(
            matches!(result, Err(error) if error.to_string() == Error::IndicesLengthGreaterThanRange.message())
        );
    }

    #[test]
    fn test_parse_full_input() {
        let input: Input = parse_str("C16..18U3..3{ 9 => C, 12 => C, 15 => T }")
            .expect("Failed to parse full input");

        assert_eq!(input.c.range.start, 16);
        assert_eq!(input.c.range.end, 18);

        assert_eq!(input.u.range.start, 3);
        assert_eq!(input.u.range.end, 3);
        assert_eq!(input.u.indices.0.len(), 3);

        let first_index = input.u.indices.0.first().unwrap();
        assert_eq!(first_index.offset, 9);
        assert!(matches!(
            first_index.unsaturated_bound,
            UnsaturatedBound::Cis(_)
        ));
    }
}
