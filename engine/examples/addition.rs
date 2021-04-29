use engine::calculate;

fn main() -> Result<(), String> {
    let _res = calculate("1 + 1")?;
    let _res2 = calculate("20 - 10")?;
    let _res3 = calculate("30 + 20 * 10")?;

    Ok(())
}
