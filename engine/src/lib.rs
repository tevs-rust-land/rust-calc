pub mod expressions;
pub mod parser;
pub mod scanner;
pub mod token;
pub fn calculate(val: &str) -> String {
    let (math_tokens, errors) = scanner::scan(val);
    if !errors.is_empty() {
        return format!("{}", math_tokens.len());
    }
    let parsed_result = parser::parse(&math_tokens);
    println!("{:?}", parsed_result);
    format!("{}", math_tokens.len())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
