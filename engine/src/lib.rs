pub mod scanner;
pub mod token;
pub fn calculate(val: &str) -> String {
    let (math_tokens, _errors) = scanner::scan(val);
    format!("{}", math_tokens.len())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
