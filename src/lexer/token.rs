use crate::number::Number;

pub enum Token {
    Number(Number),
    Operator(Operator),
    Prefix(Prefix),
    Suffix(Suffix),
    Parenthesis(Parenthesis),
    Identifier(String),
    Comma,
    SemiColon,
    End,
}

pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}

impl Operator {
    pub fn binding_power(self) -> (i8, i8) {
        match self {
            Operator::Add => (2, 3),
            Operator::Sub => (2, 3),
            Operator::Mul => (4, 5),
            Operator::Div => (4, 5),
            Operator::Mod => (4, 5),
            Operator::Pow => (7, 6),
        }
    }
}

pub enum Prefix {
    Plus,
    Minus,
}

impl Prefix {
}

pub enum Suffix {
    Thousand,
    Million,
    Billion,
    Trillion,
    Factorial,
}

impl Suffix {
}

pub enum Parenthesis {
    Open,
    Close,
}

impl Parenthesis {
}
