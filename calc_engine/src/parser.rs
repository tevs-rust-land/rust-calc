use crate::expressions::{Expression, ExpressionErrors};
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

pub fn parse(tokens: &[TokenWithContext]) -> Result<Vec<Expression>, Vec<String>> {
    let mut target: Vec<Expression> = vec![];
    let mut errors: Vec<String> = vec![];
    let mut peekable_tokens = tokens.iter().peekable();
    while let Some(expr) = addition(&mut peekable_tokens) {
        match expr {
            Expression::Error(error) => errors.push(format!("{}", error)),
            expr => target.push(expr),
        }
    }

    if errors.is_empty() {
        Ok(target)
    } else {
        Err(errors)
    }
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
        Token::DigitLiteral(num) => Some(Expression::Literal(*num)),
        Token::OpeningBracket => {
            let expression = addition(tokens)?;
            let element = tokens.next()?;
            match &element.token {
                Token::ClosingBracket => Some(Expression::Grouping(Box::new(expression))),
                e => {
                    let error_message = format!("Expected ( but got {}", e);
                    Some(Expression::Error(ExpressionErrors::UnexpectedElement(
                        error_message,
                    )))
                }
            }
        }
        _ => addition(tokens),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expressions::{Expression, Operation};
    use crate::scanner;
    #[test]
    fn test_can_parse_addition_expression() {
        let source = r#"1+1"#;
        let scanned_tokens = scanner::scan(source).expect("1 + 1 was scanned with an error");
        let parsed_expression = parse(&scanned_tokens).expect("1 + 1 should parse correctly");
        assert_eq!(
            vec![Expression::Binary(
                Box::new(Expression::Literal(1.0)),
                Operation::Addition,
                Box::new(Expression::Literal(1.0))
            )],
            parsed_expression
        )
    }

    #[test]
    fn test_can_parse_subtraction_expression() {
        let source = r#"5-2"#;
        let scanned_tokens = scanner::scan(source).expect("5 - 2 was scanned with an error");
        let parsed_expression = parse(&&scanned_tokens).expect("5 - 2 should parse correctly");
        assert_eq!(
            vec![Expression::Binary(
                Box::new(Expression::Literal(5.0)),
                Operation::Subtraction,
                Box::new(Expression::Literal(2.0))
            )],
            parsed_expression
        )
    }
    #[test]
    fn test_can_parse_grouped_expression() {
        let source = r#"(5-2)"#;
        let scanned_tokens = scanner::scan(source).expect("(5-2) was scanned with an error");
        let parsed_expression = parse(&scanned_tokens).expect("(5-2) should parse correctly");
        assert_eq!(
            vec![Expression::Grouping(Box::new(Expression::Binary(
                Box::new(Expression::Literal(5.0)),
                Operation::Subtraction,
                Box::new(Expression::Literal(2.0))
            )))],
            parsed_expression
        )
    }

    #[test]
    fn test_detect_error_from_unclosed_grouped_expression() {
        let source = r#"(5-2+"#;
        let scanned_tokens = scanner::scan(source).expect("(5-2+ was scanned with an error");

        let parse_result = parse(&scanned_tokens);
        match parse_result {
            Ok(_) => unreachable!(),
            Err(errors) => {
                let error_message = "Expected ( but got +".to_string();
                assert_eq!(vec![error_message], errors)
            }
        }
    }
}
