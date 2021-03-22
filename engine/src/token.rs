#[derive(Debug)]
pub enum Token {
    Division,
    Addition,
    Multiply,
    Subtraction,
    WhiteSpace,
    DigitLiteral(String),
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
