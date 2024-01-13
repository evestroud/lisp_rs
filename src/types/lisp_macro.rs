use super::Exp;

pub(crate) struct Macro {
    pub(crate) params: Vec<String>,
    pub(crate) body: Vec<Exp>,
}
