pub mod expressions;
pub mod parser;
pub mod scanner;
pub mod token;
pub fn calculate(val: &str) -> Result<f64, String> {
    let math_tokens = scanner::scan(val).map_err(|errors| format!("{:?}", errors))?;

    let parsed_results = parser::parse(&math_tokens).map_err(|errors| format!("{:?}", errors))?;

    let result: f64 = parsed_results
        .into_iter()
        .map(|expression| expression.execute())
        .sum();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_calculate_simple_addition() {
        let error_margin = f64::EPSILON;
        let result = calculate("1 + 1").expect("This basic calculation should be parsed correctly");
        let result = result;

        assert!((result - 2.0).abs() < error_margin)
    }
    #[test]
    fn it_should_calculate_complex_math() {
        let error_margin = f64::EPSILON;

        let expr = "1 + (5-1) + 3 + (4 * 5)";
        let result = calculate(expr).expect("This basic calculation should be parsed correctly");
        assert!((result - 28.0).abs() < error_margin)
    }
}
