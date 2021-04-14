use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Division,
    Addition,
    Multiply,
    Subtraction,
    WhiteSpace,
    DigitLiteral(String),
    OpeningBracket,
    ClosingBracket,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Division => write!(f, "/"),
            Token::Addition => write!(f, "+"),
            Token::Multiply => write!(f, "*"),
            Token::Subtraction => write!(f, "-"),
            Token::WhiteSpace => write!(f, "Whitespace"),
            Token::DigitLiteral(literal) => write!(f, "Literal {}", literal),
            Token::OpeningBracket => write!(f, "("),
            Token::ClosingBracket => write!(f, ")"),
        }
    }
}

pub type Lexeme = String;
#[derive(Debug)]
pub struct TokenWithContext {
    pub token: Token,
    pub lexeme: Lexeme,
    pub position: Position,
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub column: usize,
}

impl Position {
    pub fn initial() -> Position {
        Position { column: 1 }
    }

    pub fn increment_column(&mut self) {
        self.column += 1;
    }
}

pub fn is_digit(c: char) -> bool {
    ('0'..='9').contains(&c) || c == '.'
}

pub fn is_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\r' | '\t')
}
