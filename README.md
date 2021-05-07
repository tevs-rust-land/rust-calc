## Rust-Calc

This Project has 3 parts

1. Engine -> /calc_engine
2. Wasm wrapper -> wasm
3. The small UI -> www

### The Engine

The Engine has been built in rust with 0 dependencies. The scanner & parser are written by hand. Im using top-down recursive descent to parse & evaluate the expression given.

Check out the `examples` folder in the `calc_engine` directory to see how it can be used.

Crate [Link](https://crates.io/crates/calc_engine)

### Wasm Wrapper

The wasm wrapper is really just a wrapper for the engine which makes it accessible to browser clients.
It really just has 1 function. `calculate` which passes the input to the engine. The wrapper has been built with [wasm-pack](https://github.com/rustwasm/wasm-pack)

### The UI
