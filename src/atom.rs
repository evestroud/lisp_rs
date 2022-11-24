use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Atom {
    Int(i32),
    Float(f32),
    Symbol(String),
    Nil,
    Builtin(Builtin),
}

pub(crate) struct Builtin(pub(crate) &'static dyn Fn(Vec<Atom>) -> Atom);

impl Debug for Builtin {
    // https://stackoverflow.com/questions/38088067/equivalent-of-func-or-function-in-rust
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Builtin function")
    }
}

impl Clone for Builtin {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl PartialEq for Builtin {
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }

    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}
