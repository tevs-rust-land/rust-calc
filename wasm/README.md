## Wasm-calc

This Project is a wraper of the [Engine](https://github.com/tevs-rust-land/rust-calc/calc_engine) built in Pure Rust using top-down recursive descent.

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

This wasm module has been Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a>
