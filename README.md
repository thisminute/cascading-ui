Cwl is a front end web language implemented with Rust procedural macros.

To install, you will need:

1. [rustc/cargo](https://www.rust-lang.org/tools/install)
1. [node/npm](https://nodejs.org/en/download/)
1. [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

Then:

```bash
git clone https://github.com/thisminute/cascading-wasm-language.git
cd cascading-wasm-language
git submodule init
git submodule update
cd create-cwl-app/www
npm install
npm start
```

For windows users, run in the root directory before `npm start`:

```bash
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

# Understanding the Code

## Execution Steps

### Rust folder structure

"lib.rs" and "mod.rs" are special names that are entry points for the folders they are in. The top level is `./src/lib.rs`. The whole project is a library, and every subsequent directory contains a private module included from there.

### src/lib.rs

Execution of the code starts in one of 3 procedural macros exported from lib.rs. `cwl` is the main one, `cwl_document` and `cwl_header` are helpers meant to be used for writing tests.

### Data flow

This diagram is helpful for remembering the order that the steps happen in, reference it to help with this section and if you get lost while navigating the code!

```
cwl       -> lex     ->
tokens    -> parse*  ->
ast       -> analyze ->
semantics -> render  ->
dom       -> write   ->
compiled code!
```

\* this library starts at the parse step (`src/transform/parse.rs`), because we are using some libraries to lex out the tokens that we need

In words:
When a piece of CWL code is compiled, it is first lexed into tokens, which are then parsed into an [Abstract Syntax Tree](https://en.wikipedia.org/wiki/Abstract_syntax_tree) (AST). [Semantic analysis](<https://en.wikipedia.org/wiki/Semantic_analysis_(compilers)>) is performed on the AST to generate an object representing the meaning of the code. This "Semantics" object is rendered into an abstract representation of an HTML DOM, which is finally compiled into text strings containing HTML and CSS, and executables containing Webassembly.

Data flows between data structures (the `data` module, in the left column of the diagram) by way of transformations between those structures (the `transform` module, in the right column of the diagram). All of the code sits in the `src` directory, and starts in `lib.rs`, then passes to `transform/parse.rs` to create an object which is described in `data/ast.rs`, which are then passed to `transform/analyze.rs` to create a semantics object defined in `data/semantics.rs`, and so on through the diagram. The `write` transformation is defined in several parts, one for each of several outputs that don't resemble each other - HTML, CSS, and Rust code that is then compiled into a Wasm binary. Each output is generated with a different trait implemented on the Semantics struct.

This is the core of CWL (And could be fairly easy to adapt to other syntaxes!). In `src/misc` are files outside of this core flow, such as the helper `context` which is used during semantic analysis.

## Integration tests

`./tests` has a collection of CWL examples that render different features. Currently, they just check to see that the examples compile. Run them with `wasm-pack test --headless --chrome`. `--firefox` works too, and you'll have to have installed the browser you choose.

## Debugging

We use `create-cwl-app` to provide a server and some debugging tools to work on cwl. Use `git submodule update` to pull down the contents of the directory. `cd create-cwl-app/www` and you will have access to some npm commands. Run `npm install` first. `npm start` will compile the test app, and `npm run debug` will compile it twice, first compiling the macro to rust, which can be viewed in `create-cwl-app/www/
