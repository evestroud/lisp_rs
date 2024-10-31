use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, ItemMod};

#[proc_macro_attribute]
pub fn load_builtins(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemMod);
    eprintln!("{:?}", input.ident);
    input.into_token_stream().into()
}
