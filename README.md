Cwf is a front end web language implemented with Rust procedural macros.

To install, you will need:
1. [rustc/cargo](https://www.rust-lang.org/tools/install)
1. [node/npm](https://nodejs.org/en/download/)
1. [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

Then:
```bash
git clone --recurse-submodules https://github.com/thisminute/cascading-wasm-framework.git
cd cascading-wasm-framework
```

For windows users, run in the root directory:
```bash
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

# Understanding the Code

## Execution Steps
The code is divided at a high level into 4 parts:
1. Parse
2. Macro
3. Page Initialization
4. Execution

1. First, the cwf syntax is parsed into data structures that Rust can work with. Both the data structures and the rules for parsing are in `src/tokens.rs`.
2. Next, the data structures are turned into Rust code by the `proc_macro` defined in `src/lib.rs`. This logic is the code OUTside of `quote! {}` blocks, and the code INside of those blocks eventually runs in a browser.
3. Some of the code in the quote! {} blocks belongs to the 3rd group, and sets up the components and data bindings on a page when it is launched.
4. Though not entirely distinct from the initialization step, the rest of the code can be thought of as running as the page is in use, especially code triggered by event listeners.

## Integration Tests
`./tests` has a collection of cwf examples that render different features. Currently, they just check to see that the examples compile.
