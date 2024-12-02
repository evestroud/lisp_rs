use lisp_rs_macro::load_builtins;

#[load_builtins(test)]
mod goof {
    use lisp_rs_core::{error::SchemeError, types::Cell};

    #[builtin]
    pub fn goof(args: &Cell) -> Result<Cell, SchemeError> {
        Ok(Cell::Nil)
    }
}

fn main() {
    println!("Hello, world!");
}
