Cwl is a front end web language implemented with Rust procedural macros.

To install, you will need:
1. [rustc/cargo](https://www.rust-lang.org/tools/install)
1. [node/npm](https://nodejs.org/en/download/)
1. [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

Then:
```bash
git clone https://github.com/thisminute/cascading-wasm-language.git
cd cascading-wasm-language
```

For windows users, run in the root directory:
```bash
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

# Understanding the Code

## Execution Steps

### Rust folder structure
"lib.rs" and "mod.rs" are special names that are entry points for the folders they are in. The top level is `lib.rs` because the whole project is a library, and every subsequent folder contains a private module included from there.

### src/lib.rs
The macro starts in one of 3 procedural macros exported from lib.rs. `cwl` is the main one, `cwl_dom` and `cwl_lib` are helpers for writing tests.

### Data flow
Data flows between data structures (in the data module) by way of transformations between those structures (in the transform module). We start with the cwl input tokens themselves, as lexed by Rust, in `src/lib.rs`. You can see some valid cwl in the `tests` directory - the stuff inside of the `cwl_dom! {}` blocks are the tokens.

So, starting with cwl tokens:
```
tokens    -> parse   ->
ast       -> analyze ->
semantics -> write   ->
compiled code!
```
The `write` transformation is defined in several parts, one for each of several outputs that don't resemble each other - HML, CSS, and Rust code that is compiled later into a Wasm binary.

In terms of file paths, this translates to:
1. `src/lib.rs` - tokens already provided as a TokenStream
1. `src/transform/parse.rs`
1. `src/data/ast.rs`- [Abstract Syntax Tree](https://en.wikipedia.org/wiki/Abstract_syntax_tree)
1. `src/transform/analyze.rs` - [semantic analysis](https://en.wikipedia.org/wiki/Semantic_analysis_(compilers))
1. `src/data/semantics.rs`
1. `src/transform/write/*.rs` - the order we write the outputs in shouldn't matter

This is the core of CWL (Reuse this if you want a custom syntax that compiles to Rust!). In `src/misc` are files outside of this core flow, such as the helper `context` which is used during semantic analysis.

## Integration tests
`./tests` has a collection of cwl examples that render different features. Currently, they just check to see that the examples compile. Run them with `wasm-pack test --headless --chrome`. `--firefox` works too, and you'll have to have whichever browser you're using installed.
