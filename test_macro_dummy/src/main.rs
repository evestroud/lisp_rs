use lisp_rs_macro::load_builtins;

#[load_builtins]
mod goof {
    use lisp_rs_macro::load_builtins;
}

fn main() {
    println!("Hello, world!");
}
