## calc_engine

This Engine transforms a string representation of a Math expression to the actual result of the expression.

I'm using the top-down recursive descent parsing techque. (Input is read from Left -> Right)

## How to use.

```rs

use calc_engine;

func main() {
    let res = calc_engine::calculate("1 + 1")?;
    let error_margin = f64::EPSILON;
    assert!((result - 2.0).abs() < error_margin);

    let res = calc_engine::calculate("(1 + 1) + 3")?;

}

```

## Still to be done

- [ ] Bug fixes
- [ ] Logarithmic expressions
- [ ] More tests..

Not in any particular order.
