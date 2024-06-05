pub mod buffer;
pub mod error;
pub mod lexer;
pub mod parser;

pub mod types {
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct Cell {
        pub car: Value,
        pub cdr: Value,
    }

    impl Default for Cell {
        fn default() -> Self {
            Cell {
                car: Value::Nil,
                cdr: Value::Nil,
            }
        }
    }

    #[derive(PartialEq, Eq, Clone, Debug)]
    pub enum Value {
        Nil,
        List(Box<Cell>),
        Number(i32), // TODO Numeric tower
        Boolean(bool),
        Symbol(String),
        Quote(Box<Value>),
        // TODO String(String)
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn it_works() {}
    }
}
