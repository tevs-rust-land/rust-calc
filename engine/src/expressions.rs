pub struct Literal(pub String);

#[derive(Debug)]
pub enum Operation {
    Minus,
    Division,
    Addition,
    Multiply,
    Subtraction,
}
#[derive(Debug)]

pub enum Expression {
    Literal(String),
    Binary(Box<Expression>, Operation, Box<Expression>),
}
