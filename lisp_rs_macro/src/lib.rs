use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, visit_mut::VisitMut, Error, Ident, ItemMod,
    Path,
};

#[proc_macro_attribute]
pub fn load_builtins(args: TokenStream, input: TokenStream) -> TokenStream {
    // When using this macro in lisp_rs_core, the path for types used needs to
    // start with `crate`, but in tests that import types from lisp_rs_core the
    // path needs to be `lisp_rs_core`. This annotation argument changes the
    // path for tests.
    let mut crate_source: Path = parse_quote! {crate};
    let args_parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("test") {
            crate_source = parse_quote! { lisp_rs_core};
        };
        Ok(())
    });
    parse_macro_input!(args with args_parser);

    let mut input = parse_macro_input!(input as ItemMod);

    let mut visitor = ModVisitor {
        builtins: Vec::new(),
    };
    visitor.visit_item_mod_mut(&mut input);

    let builtin_load_statements =
        visitor
            .builtins
            .iter()
            .map(|BuiltinInfo { lisp_name, fn_name }| {
                quote! {
                    #crate_source::types::functions::Builtin {
                        name: #lisp_name,
                        func: &#fn_name
                    }
                }
            });

    if builtin_load_statements.len() == 0 {
        let e = Error::new(input.span(), "no #[builtin] annotated functions found")
            .into_compile_error();
        let input = input.into_token_stream();
        return quote! {
            #e
            #input
        }
        .into();
    }

    let load_builtins_fn = quote! {
        use #crate_source::environment::FrameRef;

        pub fn load_builtins(env: &mut #crate_source::environment::FrameRef) {
            env.borrow_mut().load_builtins(vec![
                #(#builtin_load_statements),*
            ])

        }
    };

    input
        .content
        .as_mut()
        .expect("Expect mod to have content")
        .1
        .push(syn::Item::Verbatim(load_builtins_fn));
    input.into_token_stream().into()
}

struct BuiltinInfo {
    lisp_name: String,
    fn_name: Ident,
}

struct ModVisitor {
    builtins: Vec<BuiltinInfo>,
}

impl VisitMut for ModVisitor {
    fn visit_item_fn_mut(&mut self, i: &mut syn::ItemFn) {
        let builtin_attr_pos = i
            .attrs
            .iter()
            .position(|attr| attr.meta.path().is_ident("builtin"));
        if let Some(pos) = builtin_attr_pos {
            // TODO add "name" field to the attribute and get it here
            i.attrs.remove(pos);

            let fn_name = i.sig.ident.clone();
            self.builtins.push(BuiltinInfo {
                lisp_name: fn_name.to_string(),
                fn_name,
            })
        }
    }
}
