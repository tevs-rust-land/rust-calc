## Rust-Calc
![image](https://user-images.githubusercontent.com/12128153/117509590-32119b00-af93-11eb-91fc-8560eeaf5fa6.png)


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

#### Installing via npm
Install the wasm wrapper from https://www.npmjs.com/package/wasm-calc/v/0.3.3


### The UI

The UI is just a 1 page app built with Next that takes the expression from an input & returns the result or the error incase a bad expression was encountered.
Happy path
![image](https://user-images.githubusercontent.com/12128153/117509590-32119b00-af93-11eb-91fc-8560eeaf5fa6.png)

Unexpected character
![image](https://user-images.githubusercontent.com/12128153/117509696-61280c80-af93-11eb-8439-e0eaa5da5ba1.png)

Unterminated expression
![image](https://user-images.githubusercontent.com/12128153/117509800-887ed980-af93-11eb-9b18-a9d203529c25.png)


This project has been made possible by Rust & WebAssembly 🦀 + 🕸

