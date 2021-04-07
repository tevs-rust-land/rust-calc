use crate::expressions::Expression;
use crate::token::{Token, TokenWithContext};
use std::iter::Peekable;

mod operators {
    use crate::expressions::Operation;
    use crate::token::{Token, TokenWithContext};
    use std::iter::Peekable;
    pub fn additional_signs<'a, I>(tokens: &mut Peekable<I>) -> Option<Operation>
    where
        I: Iterator<Item = &'a TokenWithContext>,
    {
        match tokens.peek().map(|token| &token.token) {
            Some(Token::Addition) => {
                let _ = tokens.next();
                Some(Operation::Addition)
            }
            Some(Token::Subtraction) => {
                let _ = tokens.next();
                Some(Operation::Subtraction)
            }
            _ => None,
        }
    }

    pub fn multiplication_signs<'a, I>(tokens: &mut Peekable<I>) -> Option<Operation>
    where
        I: Iterator<Item = &'a TokenWithContext>,
    {
        match tokens.peek().map(|token| &token.token) {
            Some(Token::Multiply) => {
                let _ = tokens.next();
                Some(Operation::Multiply)
            }
            Some(Token::Division) => {
                let _ = tokens.next();
                Some(Operation::Division)
            }
            _ => None,
        }
    }
}

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

    if let Some(operator) = operators::additional_signs(tokens) {
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

    if let Some(operator) = operators::multiplication_signs(tokens) {
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

// TODO: Add ability for grouping with ()
// TODO: Add tests for the parser
