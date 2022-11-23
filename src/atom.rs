#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Atom {
    Int(i32),
    Float(f32),
    Symbol(String),
    Nil,
}
