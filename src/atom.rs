use crate::{
    builtin::Builtin,
    environment::Env,
    evaluator::eval_all,
    lib::{validate_num_args, SchemeError},
};
use std::{
    cell::RefCell,
    fmt::{self, Debug, Display},
    rc::Rc,
};

pub mod rational;

impl Atom {
    pub(crate) fn is_number(&self) -> bool {
        match self {
            Self::Number(_) => true,
            _ => false,
        }
    }

    pub(crate) fn is_symbol(&self) -> bool {
        match self {
            Self::Symbol(_) => true,
            _ => false,
        }
    }
    pub(crate) fn is_nil(&self) -> bool {
        match self {
            Self::Nil => true,
            _ => false,
        }
    }
    pub(crate) fn is_builtin(&self) -> bool {
        match self {
            Self::Builtin(_) => true,
            _ => false,
        }
    }
    pub(crate) fn is_special_form(&self) -> bool {
        match self {
            Self::SpecialForm(_) => true,
            _ => false,
        }
    }
    pub(crate) fn is_quote(&self) -> bool {
        match self {
            Self::Quote(_) => true,
            _ => false,
        }
    }
    pub(crate) fn is_lambda(&self) -> bool {
        match self {
            Self::Lambda(_) => true,
            _ => false,
        }
    }
    pub(crate) fn is_boolean(&self) -> bool {
        match self {
            Self::Boolean(_) => true,
            _ => false,
        }
    }
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Atom::Number(n) => n.to_string(),
            Atom::Symbol(s) => s.to_string(),
            Atom::Nil => "Nil".to_string(),
            Atom::Builtin(b) => b.to_string(),
            Atom::SpecialForm(f) => f.to_string(),
            Atom::Quote(exp) => format!("'{}", exp),
            Atom::Lambda(l) => l.to_string(),
            Atom::Boolean(b) => b.to_string(),
        };
        write!(f, "{}", val)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Lambda {
    pub(crate) params: Vec<String>,
    pub(crate) body: Vec<SchemeExp>,
    pub(crate) env: Rc<RefCell<Env>>,
}

impl Lambda {
    pub(crate) fn eval(&mut self, args: &[Atom]) -> Result<Atom, SchemeError> {
        validate_num_args(&args, self.params.len(), self.params.len())?;
        for (name, val) in self.params.iter().zip(args) {
            self.env.borrow_mut().set(name, &val);
        }

        eval_all(&self.body, &mut self.env)
    }
}

impl Display for Lambda {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = format!(
            "(lambda ({}) {})",
            self.params.join(" "),
            self.body
                .iter()
                .map(|exp| exp.to_string())
                .reduce(|p, c| p + " " + &c)
                .unwrap_or("".to_string())
        );
        write!(f, "{}", string)
    }
}
