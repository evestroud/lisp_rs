use lisp_rs_macro::load_builtins;

#[load_builtins]
mod example_type {
    use lisp_rs_core::{error::SchemeError, types::Cell};

    #[builtin]
    pub fn example_fn(args: &Cell) -> Result<Cell, SchemeError> {
        Ok(Cell::Nil)
    }
}

fn main() {}
