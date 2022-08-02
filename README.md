# Minesweeper

Let's try rust & webassembly by creating Minesweeper.
Based on: <https://youtu.be/0ywizYLPV00>

## Prerequisites

* rust & cargo
* wasm-pack

## Process

1. Create a new rust lib (inside existing directory) with `cargo init --lib`
2. Let's build the logic with rust first, without thinking about webassembly
3. Link wasm module <-> javascript with [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)
4. Enjoy!

### Step 3: Link wasm module with javascript using wasm-bindgen

Step 1: `cargo add wasm-bindgen`

Step 2: add to Cargo.toml file:

```toml
[lib]
crate-type = ["cdylib"]
```

Step 3: add the prelude `use wasm_bindgen::prelude::*;`

Step 4: bind javascript and rust functions with `#[wasm_bindgen]`

Step 5: compile the project into wasm with `wasm-pack build --target web`
(it creates `pkg/`)

Step 6: import in the html file (in the `<script>`), the functions needed

```javascript
import init, {greet} from "./pkg/minesweeper.js";

async function main() {
    await init(); // init wasm module

    greet("Bob");
}

main();
```

### A note on the rand library

<https://github.com/rust-random/rand#wasm-support>

> the WASM target `wasm-unknown-unknown` is not automatically supported by `rand`
> or `getrandom`. To solve this, either use a different target such as
> `wasm32-wasi` or add a direct dependency on `getrandom` with the js feature
> (if the target supports JavaScript). See getrandom#WebAssembly support.

the target `wasm-unknown-unknown` is the one used by `wasm-pack`.

<https://docs.rs/getrandom/latest/getrandom/#webassembly-support>

> [...] is not automatically supported since, from the target name alone, we
> cannot deduce which JavaScript interface is in use (of if JavaScript is
> available at all).

<https://docs.rs/getrandom/latest/getrandom/#indirect-dependencies>

to use `rand` we need to add `getrandom` with the feature `["js"]`

```toml
[dependencies]
getrandom = { version = "0.2.7", features: ["js"]}
```