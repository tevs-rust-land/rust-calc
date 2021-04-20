pub mod expressions;
pub mod parser;
pub mod scanner;
pub mod token;
pub fn calculate(val: &str) -> String {
    let (math_tokens, errors) = scanner::scan(val);
    if !errors.is_empty() {
        return format!("{:?}", errors);
    }
    let (parsed_result, _errors) = parser::parse(&math_tokens);
    let mut result: f64 = 0.0;
    for expression in parsed_result {
        result += expression.execute()
    }
    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_calculate_simple_addition() {
        let error_margin = f64::EPSILON;
        let result = calculate("1 + 1");
        let result = result.parse::<f64>().unwrap();

        assert!((result - 2.0).abs() < error_margin)
    }
}
