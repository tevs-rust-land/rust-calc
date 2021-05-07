## Rust-Calc

![image](https://user-images.githubusercontent.com/12128153/117473350-894b4780-af62-11eb-8590-0db8bd07bf81.png)

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

#### Running the build (from wasm)

Install wasm-pack, assuming you have rust installed.

```sh
cargo install --git https://github.com/rustwasm/wasm-pack.git

cd wasm

wasm-pack build

```

### The UI

The UI is just a 1 page app built with Next that takes the expression from an input & returns the result or the error incase a bad expression was encountered.
