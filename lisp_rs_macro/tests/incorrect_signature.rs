use lisp_rs_macro::load_builtins;

#[load_builtins]
mod test_compiles {
    #[builtin]
    fn do_nothing() {}
}

fn main() {}
