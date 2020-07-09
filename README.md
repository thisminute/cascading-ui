CWF is a front end web language implemented with Rust procedural macros.

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

## Procedural Macro
`./src/lib.rs` is the entry point for the procedural macro that generates Rust code from Cwf file input.
`./src/tokens.rs` is the definition file for the tokens that the language consists of. Rule and List are the most important tokens.

## Integration Tests
`./tests` has a collection of cwf examples that render different features. Currently, they just check to see that the examples compile.

## `create-cwf-app`
`./create-cwf-app` creates a node/webpack server that is used to test and deploy the language. See its README for usage details.
