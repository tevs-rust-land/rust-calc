## Wasm-calc

This Project is a wraper around the [Engine](https://github.com/tevs-rust-land/rust-calc/calc_engine) which scans & parses a math expression from string format to the result of the expression.

## Usage

```js
import("wasm-calc").then((calc) => {
  try {
    const result = calc.calculate("1 + 1");
    // returns 3 :).. Just kidding, result =  2
  } catch (error) {}
});
```

### Return type

We get back a

```rs
Result<f64, JsValue>
```

which in JS translates to `number | Error(string)`, hence the need for a try catch.

## Improvements to be made & TODOS

- [ ] Shrink size of the wasm file.

This wasm module has been Built with 🦀🕸
