use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    Division,
    Addition,
    Multiply,
    Subtraction,
}
#[derive(Debug, PartialEq)]

pub enum Expression {
    Literal(f64),
    Binary(Box<Expression>, Operation, Box<Expression>),
    Grouping(Box<Expression>),
    Error(ExpressionErrors),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExpressionErrors {
    UnexpectedElement(String),
}

impl fmt::Display for ExpressionErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpressionErrors::UnexpectedElement(message) => {
                write!(f, "{}", message)
            }
        }
    }
}

impl Expression {
    pub fn execute(&self) -> f64 {
        match self {
            Expression::Literal(num) => *num,
            Expression::Binary(left, operation, right) => {
                let left = Expression::execute(left);
                let right = Expression::execute(right);
                match &operation {
                    Operation::Addition => left + right,
                    Operation::Division => left / right,
                    Operation::Multiply => left * right,
                    Operation::Subtraction => left - right,
                }
            }
            Expression::Grouping(expr) => Expression::execute(expr),
            Expression::Error(_) => {
                unreachable!()
            }
        }
    }
}
