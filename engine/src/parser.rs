use crate::expressions::{Expression, Literal};
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

fn addition<'a, I>(tokens: &mut Peekable<I>)
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    let left = multiplication(tokens);
}

fn multiplication<'a, I>(tokens: &mut Peekable<I>)
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    let left = primary(tokens);
}

fn primary<'a, I>(tokens: &mut Peekable<I>) -> Option<Expression>
where
    I: Iterator<Item = &'a TokenWithContext>,
{
    match tokens.peek().map(|t| &t.token) {
        Some(&Token::DigitLiteral(num)) => Some(Expression::Literal(num)),
        None => None,
    }
}
