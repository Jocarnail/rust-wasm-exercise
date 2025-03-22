# Setup and notes

Installation and setup steps followed.
Not worth making an `.sh` file since they are mostly run once.

## wasm-pack installation

Installing wasm-pack from cargo.

```bash
cargo install wasm-pack

```

## cargo-generate

```bash
cargo install cargo-generate
```

## update npm
```bash
npm install npm@latest -g
```

## clone from template
```bash
cargo generate --git https://github.com/rustwasm/wasm-pack-template
```

Renamed Template's README file to [README_TEMPLATE.md](README_TEMPLATE.md)
Moved all files of template to main directory.
Deleated original template directory.

## Note on wasm-bindgen
Looking into [wasm-bindgen guide](https://rustwasm.github.io/wasm-bindgen/introduction.html).
This is used to generate the binding between JS and Rust.

From the first example:

```rust
#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);
}
```

import the window.alert JS function.
`extern "C"` seems to be the definition to call a C library using ABI^[application binary interface].
I think that the working principle is the same in `C`, maybe I can look it up there as well.  
It seems that `"C"` is just the definition for a interface, in this case the C ABI.
Which one are there and how do I use them?
The [Reference](https://doc.rust-lang.org/reference/items/external-blocks.html) cites `"crate"`, `"stdcall` for window API, and `"system"` which is converted to `"C"` or `"stdcall"`.

I think that this is the same as writing: ??

```rust
#[wasm_bindgen]
extern "C" fn alert(s: &str);
```

Additional info from [Rust edition guide](https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-extern.html): all the functions called inside the block are linked to external libraries.
Are they just called by name? It seems to automaticaly find them in the standard C libraries of the system.
Can there be two fn with the same name?


Anyway moving on.
Then:

```rust
#[wasm_bindgen]
pub fn greet(name: &str){
  alert(&format!("Hello, {}!", name));
}
```

access the alert function and run it with the appropriate arguments.
Then is provided to JS which access it via:

```js
import { greet } from "./pkg";

greet("world")
```

Note: `"./pkg"` is the target directory for `wasm-pack` compilation.

I can call a specific namespace inside JS with the macro `#[wasm_bindgen(js_namespace="xxxxx")]`.
I can call a function in JS named X using a different name Y in Rust with the macro:

```rust
#[wasm_bindgen]
extern "C"{
  #[wasm_bindgen(js_name = X)]
  fn Y(...);
}
```

Note that the same JS function can be bound to different Rust functions.

## First compilation
Warning on `src/util.rs` because function is not yet used. Expected.
