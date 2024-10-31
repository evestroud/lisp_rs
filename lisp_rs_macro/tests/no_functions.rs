use lisp_rs_macro::load_builtins;

#[load_builtins]
mod test_compiles {
    fn do_nothing() {}
}

fn main() {}
