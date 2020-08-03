Cwl is a front end web language implemented with Rust procedural macros.

To install, you will need:
1. [rustc/cargo](https://www.rust-lang.org/tools/install)
1. [node/npm](https://nodejs.org/en/download/)
1. [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

Then:
```bash
git clone --recurse-submodules https://github.com/thisminute/cascading-wasm-language.git
cd cascading-wasm-language
```

For windows users, run in the root directory:
```bash
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

# Understanding the Code

## Execution Steps
1. lib.rs
   - The macro code is entered through one of 3 procedural macros exported from lib.rs. `cwl` is the main one, cwl_dom and cwl_lib are helpers for writing tests.
1. parse.rs/tokens.rs
   - The parse trait parses the input syntax into structs defined in tokens.rs, which can be thought of as an AST with a Document as the root, Blocks as branches, and Rules as leaves.
1. lex.rs/meta.rs
   - The lex trait walks through the AST and fills out a struct called Meta defined in meta.rs
1. html.rs
   - The html trait walks through Meta and fills the html minifier, which then writes to a file
1. quote.rs
   - The quote trait walks through Meta and generates Rust code, after which the compiler takes over to generate the final wasm target

1. First, the cwl syntax is parsed into data structures that Rust can work with. Both the data structures and the rules for parsing are in `src/tokens.rs`.
2. Next, the data structures are turned into Rust code by the `proc_macro` defined in `src/lib.rs`. This logic is the code OUTside of `quote! {}` blocks, and the code INside of those blocks eventually runs in a browser.
3. Some of the code in the quote! {} blocks belongs to the 3rd group, and sets up the components and data bindings on a page when it is launched.
4. Though not entirely distinct from the initialization step, the rest of the code can be thought of as running as the page is in use, especially code triggered by event listeners.

## Integration Tests
`./tests` has a collection of cwl examples that render different features. Currently, they just check to see that the examples compile.
