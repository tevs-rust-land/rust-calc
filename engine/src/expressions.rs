pub struct Literal(pub String);

#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    Division,
    Addition,
    Multiply,
    Subtraction,
}
#[derive(Debug, PartialEq, Eq)]

pub enum Expression {
    Literal(String),
    Binary(Box<Expression>, Operation, Box<Expression>),
    Grouping(Box<Expression>),
    Error(ExpressionErrors),
}

#[derive(Debug, PartialEq, Eq)]

pub enum ExpressionErrors {
    UnexpectedElement(String),
}
