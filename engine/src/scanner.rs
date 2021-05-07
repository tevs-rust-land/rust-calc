use crate::token::{self, Position, Token, TokenWithContext};
use std::iter::Peekable;
use std::str;

#[derive(Debug)]
pub enum ScannerError {
    UnexpecredCharacter(char),
    NumberParsingError(String),
}

struct Scanner<'a> {
    current_lexeme: String,
    current_position: Position,
    source: Peekable<str::Chars<'a>>,
}

impl<'a> Scanner<'a> {
    fn initialize(source: &'a str) -> Scanner {
        Scanner {
            current_lexeme: "".into(),
            current_position: Position::initial(),
            source: source.chars().into_iter().peekable(),
        }
    }

    fn advance(&mut self) -> Option<char> {
        let next = self.source.next();
        if let Some(c) = next {
            self.current_lexeme.push(c);
            self.current_position.increment_column();
        }
        next
    }

    fn add_context(&mut self, token: Token, initial_position: Position) -> TokenWithContext {
        TokenWithContext {
            token,
            lexeme: self.current_lexeme.clone(),
            position: initial_position,
        }
    }

    fn scan_next(&mut self) -> Option<Result<TokenWithContext, ScannerError>> {
        let initial_position = self.current_position;
        self.current_lexeme.clear();
        let next_char = match self.advance() {
            Some(c) => c,
            None => return None,
        };

        let result = match next_char {
            '*' => Ok(Token::Multiply),
            '-' => Ok(Token::Subtraction),
            '+' => Ok(Token::Addition),
            '/' => Ok(Token::Division),
            '(' => Ok(Token::OpeningBracket),
            ')' => Ok(Token::ClosingBracket),
            c if token::is_whitespace(c) => Ok(Token::WhiteSpace),
            c if token::is_digit(c) => self.digit(),
            c => Err(ScannerError::UnexpecredCharacter(c)),
        };

        Some(result.map(|token| self.add_context(token, initial_position)))
    }
    fn peek_check(&mut self, check: &dyn Fn(char) -> bool) -> bool {
        match self.source.peek() {
            Some(&c) => check(c),
            None => false,
        }
    }

    fn advance_while(&mut self, condition: &dyn Fn(char) -> bool) {
        while self.peek_check(condition) {
            self.advance();
        }
    }

    fn digit(&mut self) -> Result<token::Token, ScannerError> {
        self.advance_while(&|c| token::is_digit(c));
        let literal_length = self.current_lexeme.len();
        let num: String = self.current_lexeme.chars().take(literal_length).collect();
        let num = num
            .parse::<f64>()
            .map_err(|_| ScannerError::NumberParsingError(num))?;
        Ok(Token::DigitLiteral(num))
    }
}

struct TokensIterator<'a> {
    scanner: Scanner<'a>,
}

impl<'a> Iterator for TokensIterator<'a> {
    type Item = Result<TokenWithContext, ScannerError>;
    fn next(&mut self) -> Option<Self::Item> {
        self.scanner.scan_next()
    }
}

pub fn scan_into_iterator<'a>(
    source: &'a str,
) -> impl Iterator<Item = Result<TokenWithContext, ScannerError>> + 'a {
    TokensIterator {
        scanner: Scanner::initialize(source),
    }
}

pub fn scan(source: &str) -> (Vec<TokenWithContext>, Vec<ScannerError>) {
    let mut tokens = Vec::new();
    let mut errors = Vec::new();

    for result in scan_into_iterator(source) {
        match result {
            Ok(token_with_context) => match token_with_context.token {
                Token::WhiteSpace => {}
                _ => tokens.push(token_with_context),
            },
            Err(error) => errors.push(error),
        }
    }
    (tokens, errors)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_can_scan_addition_expression() {
        let source = r#"1+1"#;
        let (scanned_tokens, err) = scan(source);
        assert_eq!(scanned_tokens.len(), 3);
        assert_eq!(err.len(), 0);
    }
}
