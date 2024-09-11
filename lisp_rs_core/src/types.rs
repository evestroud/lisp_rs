use std::fmt::Display;

use functions::{Builtin, Lambda};

pub(crate) mod functions;

#[derive(Default, PartialEq, Clone, Debug)]
pub enum Cell {
    #[default]
    Nil,
    Pair(Box<Cell>, Box<Cell>),
    Number(i32), // TODO Numeric tower
    Boolean(bool),
    Symbol(String),
    Quote(Box<Cell>),
    Builtin(Builtin),
    Lambda(Lambda), // TODO String(String)
}

impl Cell {
    pub fn new_pair() -> Self {
        Cell::Pair(Box::new(Cell::Nil), Box::new(Cell::Nil))
    }

    pub fn new_pair_from(car: Cell, cdr: Cell) -> Self {
        Cell::Pair(Box::new(car), Box::new(cdr))
    }

    pub fn iter(&self) -> IntoIter {
        IntoIter { next: self }
    }

    pub fn len(&self) -> usize {
        self.iter().count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Nil => "".to_string(),
                Cell::Pair(car, cdr) => {
                    let mut list = vec![car.to_string()];
                    let mut tail = cdr;
                    while let Cell::Pair(ref _car, ref _cdr) = **tail {
                        list.push(_car.to_string());
                        tail = _cdr;
                    }
                    if **tail != Cell::Nil {
                        list.push(".".to_string());
                        list.push(tail.to_string());
                    }
                    format!("({})", list.join(" "))
                }
                Cell::Number(n) => n.to_string(),
                Cell::Boolean(b) => b.to_string(),
                Cell::Symbol(s) => s.to_string(),
                Cell::Quote(q) => format!("'{}", q),
                Self::Builtin(b) => b.to_string(),
                Cell::Lambda(l) => format!("{:?}", l),
            }
        )
    }
}

pub struct IntoIter<'a> {
    next: &'a Cell,
}

impl<'a> Iterator for IntoIter<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            Cell::Nil => None,
            Cell::Pair(car, cdr) => {
                self.next = cdr;
                Some(car)
            }
            _ => {
                self.next = &Cell::Nil;
                Some(self.next)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.next.len();
        (len, Some(len))
    }
}

impl<'a> IntoIterator for &'a Cell {
    type Item = &'a Cell;

    type IntoIter = IntoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { next: self }
    }
}

pub struct Iter {
    next: Option<Cell>,
}

impl ExactSizeIterator for Iter {}

impl Iterator for Iter {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next.take() {
            Some(Cell::Pair(car, cdr)) => {
                self.next = Some(*cdr);
                Some(*car)
            }
            _ => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = match &self.next {
            Some(cell) => cell.len(),
            None => 0,
        };
        (len, Some(len))
    }
}
