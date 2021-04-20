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

impl Expression {
    pub fn execute(&self) -> f64 {
        match self {
            Expression::Literal(str) => str
                .to_owned()
                .parse::<f64>()
                .expect("Failed to parse number"),
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
