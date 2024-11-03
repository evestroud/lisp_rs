use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, visit_mut::VisitMut, Error, Ident, ItemMod};

#[proc_macro_attribute]
pub fn load_builtins(_: TokenStream, input: TokenStream) -> TokenStream {
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
                    lisp_rs_core::types::functions::Builtin {
                        name: #lisp_name,
                        func: &#fn_name
                    }
                }
            });

    if builtin_load_statements.len() == 0 {
        return Error::new_spanned(input, "no #[builtin] annotated functions found")
            .into_compile_error()
            .into();
    }

    let load_builtins_fn = quote! {
        use lisp_rs_core::environment::FrameRef;

        pub fn load_builtins(env: &mut lisp_rs_core::environment::FrameRef) {
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
