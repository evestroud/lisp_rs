pub mod buffer;
pub mod error;
pub mod lexer;
pub mod parser;

pub mod types {
    use std::fmt::Display;

    #[derive(PartialEq, Eq, Clone, Debug)]
    pub enum Cell {
        Nil,
        Pair(Box<Cell>, Box<Cell>),
        Number(i32), // TODO Numeric tower
        Boolean(bool),
        Symbol(String),
        Quote(Box<Cell>),
        // TODO String(String)
    }

    impl Cell {
        pub fn new_pair() -> Self {
            Cell::Pair(Box::new(Cell::Nil), Box::new(Cell::Nil))
        }
    }

    impl Default for Cell {
        fn default() -> Self {
            Cell::Nil
        }
    }

    impl Display for Cell {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Cell::Nil => "".to_owned(),
                    Cell::Pair(car, cdr) => {
                        let list = vec![car.to_string()];
                        let tail = cdr;
                        while let Cell::Pair(ref _car, ref _cdr) = **tail {
                            // TODO add cars to list
                        }
                        // TODO if remaining cdr is not Nil, add a dot and final value
                        format!("({})", list.join(" "))
                    }
                    Cell::Number(n) => format!("{}", n),
                    Cell::Boolean(b) => format!("{}", b),
                    Cell::Symbol(s) => format!("{}", s),
                    Cell::Quote(q) => todo!(),
                }
            )
        }
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn it_works() {}
    }
}
