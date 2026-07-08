mod error;
mod parse;

use crate::parse::Input;
use proc_macro2::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn fatty_acid(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as Input);

    match TokenStream::try_from(input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}
