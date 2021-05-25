use crate::token::{self, Position, Token, TokenWithContext};
use auto_correct_n_suggest;
use std::collections::HashMap;
use std::fmt;
use std::iter::Peekable;
use std::str;

#[derive(Debug, PartialEq, Eq)]
pub enum ScannerError {
    UnexpectedCharacter(char),
    NumberParsingError(String),
    DidYouMean(String),
    UnknownKeyWord(String),
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScannerError::DidYouMean(suggestion) => write!(f, "Did you mean {} ?", suggestion),
            ScannerError::NumberParsingError(number) => {
                write!(f, "Unrecognized number {}", number)
            }
            ScannerError::UnexpectedCharacter(c) => write!(f, "Unexpected character {}", c),
            ScannerError::UnknownKeyWord(keyword) => write!(f, "Unknown keyword {}", keyword),
        }
    }
}

struct Scanner<'a> {
    keywords: HashMap<String, token::Token>,
    dictionary: auto_correct_n_suggest::Dictionary,
    current_lexeme: String,
    current_position: Position,
    source: Peekable<str::Chars<'a>>,
}

impl<'a> Scanner<'a> {
    fn initialize(source: &'a str) -> Scanner {
        let mut keywords = HashMap::new();
        let mut dictionary = auto_correct_n_suggest::Dictionary::new();
        Scanner::add_keywords_to_hashmap(&mut keywords);
        Scanner::insert_keywords_to_dictionary(&keywords, &mut dictionary);
        Scanner {
            dictionary,
            keywords,
            current_lexeme: "".into(),
            current_position: Position::initial(),
            source: source.chars().into_iter().peekable(),
        }
    }

    fn add_keywords_to_hashmap(keywords: &mut HashMap<String, Token>) {
        keywords.insert("plus".to_string(), token::Token::Addition);
        keywords.insert("minus".to_string(), token::Token::Subtraction);
        keywords.insert("multiplication".to_string(), token::Token::Multiply);
        keywords.insert("division".to_string(), token::Token::Division);
    }

    fn insert_keywords_to_dictionary(
        keywords: &HashMap<String, Token>,
        dictionary: &mut auto_correct_n_suggest::Dictionary,
    ) {
        for keyword in keywords.keys() {
            dictionary.insert(keyword.to_string())
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
            c if token::is_alpha(c) => self.keyword(),
            c => Err(ScannerError::UnexpectedCharacter(c)),
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

    fn keyword(&mut self) -> Result<token::Token, ScannerError> {
        self.advance_while(&|c| token::is_alpha(c));
        let literal_length = self.current_lexeme.len();
        let mut keyword: String = self.current_lexeme.chars().take(literal_length).collect();
        keyword.make_ascii_lowercase();
        match self.keywords.get(&keyword) {
            Some(token) => Ok(*token),
            None => Err(self.attempt_to_suggest_word(&keyword)),
        }
    }
    fn attempt_to_suggest_word(&mut self, keyword: &str) -> ScannerError {
        let auto_suggested_word = self
            .dictionary
            .auto_correct(keyword.to_string())
            .and_then(|suggestions| suggestions.first().cloned());
        match auto_suggested_word {
            Some(word) => ScannerError::DidYouMean(word),
            None => ScannerError::UnknownKeyWord(keyword.to_string()),
        }
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

pub fn scan(source: &str) -> Result<Vec<TokenWithContext>, Vec<String>> {
    let mut tokens = Vec::new();
    let mut errors = Vec::new();

    for result in scan_into_iterator(source) {
        match result {
            Ok(token_with_context) => match token_with_context.token {
                Token::WhiteSpace => {}
                _ => tokens.push(token_with_context),
            },
            Err(error) => errors.push(format!("{}", error)),
        }
    }
    if errors.is_empty() {
        Ok(tokens)
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_can_scan_addition_expression() {
        let source = r#"1+1"#;
        let scanned_tokens = scan(source).expect("1 + 1 was scanned with an error");
        assert_eq!(scanned_tokens.len(), 3);
    }

    #[test]
    fn test_can_scan_with_keywords() {
        let source = r#"1 plus 1"#;
        let scanned_tokens = scan(source).expect("1 plus 1 was scanned with an error");
        assert_eq!(scanned_tokens.len(), 3);
    }

    #[test]
    fn test_scanner_can_recognize_auto_capitalized_keywords() {
        let source = r#"1 PLUS 1"#;
        let scanned_tokens = scan(source).expect("1 PLUS 1 was scanned with an error");
        assert_eq!(scanned_tokens.len(), 3);
    }

    #[test]
    fn test_scanner_can_auto_suggest_keyword() {
        let source = r#"1 plux 1"#;
        let scanned_tokens = scan(source);
        assert!(scanned_tokens.is_err());
        let err = scanned_tokens.unwrap_err();
        assert_eq!(vec!["Did you mean plus ?".to_string()], err)
    }
}
