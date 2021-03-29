use crate::expressions::{Expression, Operation};
use crate::token::{Token, TokenWithContext};
use std::iter::Peekable;

pub fn parse(tokens: &[TokenWithContext]) -> Vec<Expression> {
    let mut target: Vec<Expression> = vec![];
    let mut peekable_tokens = tokens.iter().peekable();
    while let Some(expr) = addition(&mut peekable_tokens) {
        target.push(expr)
    }

    target
}

fn addition<'a, I>(tokens: &mut Peekable<I>) -> Option<Expression>
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    let left = multiplication(tokens)?;
    let mut get_addition_sign = || match tokens.peek().map(|token| &token.token) {
        Some(Token::Addition) => {
            let _ = tokens.next();
            Some(Operation::Addition)
        }
        Some(Token::Subtraction) => {
            let _ = tokens.next();
            Some(Operation::Subtraction)
        }
        _ => None,
    };

    if let Some(operator) = get_addition_sign() {
        let right = multiplication(tokens)?;
        return Some(Expression::Binary(
            Box::new(left),
            operator,
            Box::new(right),
        ));
    }

    Some(left)
}

fn multiplication<'a, I>(tokens: &mut Peekable<I>) -> Option<Expression>
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    let left = primary(tokens)?;

    let mut get_multiplication_sign = || match tokens.peek().map(|token| &token.token) {
        Some(Token::Multiply) => {
            let _ = tokens.next();
            Some(Operation::Multiply)
        }
        Some(Token::Division) => {
            let _ = tokens.next();
            Some(Operation::Division)
        }
        _ => None,
    };
    if let Some(operator) = get_multiplication_sign() {
        let right = primary(tokens)?;
        return Some(Expression::Binary(
            Box::new(left),
            operator,
            Box::new(right),
        ));
    };
    Some(left)
}

fn primary<'a, I>(tokens: &mut Peekable<I>) -> Option<Expression>
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    let current_token = tokens.next()?;
    match &current_token.token {
        Token::DigitLiteral(num) => Some(Expression::Literal(num.to_string())),
        _ => None,
    }
}
