pub mod buffer;
pub mod error;
pub mod lexer;

pub mod types {
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct Cell {
        car: Value,
        cdr: Value,
    }

    #[derive(PartialEq, Eq, Clone, Debug)]
    pub enum Value {
        Nil,
        List(Box<Cell>),
        Number(i32), // TODO Numeric tower
        Boolean(bool),
        Symbol(String),
        // TODO String(String)
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn it_works() {}
    }
}
