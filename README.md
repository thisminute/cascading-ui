CUI is a language for building UIs, and is implemented in this repository using Rust procedural macros. Code compiled from CUI runs in a browser, and consists of an HTML component and a Webassembly binary.

This repository is primarily for people interested in the development of the CUI language itself! If you are interested in building apps with CUI, take a look at the README for the [cui-tools repository](https://github.com/thisminute/cui-tools.git).

This repository includes cui-tools as a git submodule to make testing the library easier.

# Installation

You will need rustc and wasm-pack to build the library and run tests:

1. [rustc/cargo](https://www.rust-lang.org/tools/install)
1. [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

Then:

```bash
git clone https://github.com/thisminute/cascading-ui.git
cd cascading-ui

# you only need to do this submodule step if you plan to use cui-tools
git submodule update --init --recursive
```

Windows users may also need to run the following in the root directory:

```bash
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

Once everything is set up, there are two ways to test the library:

1. Use the included cui-tools to build an app that uses the library. Edit files in `cui-tools/app/src` to make changes to the app, then `cargo run` in the cui-tools directory.
1. Use the tests in the `tests` directory. Edit or add tests, then use `wasm-pack test --headless --firefox` (or `--chrome`, or `--safari`) to run them.

For more information about each workflow, take a look at the README files in `cui-tools` and `tests` respectively.

# Understanding the Code

## Execution Steps

### Rust folder structure

"lib.rs" and "mod.rs" are special names that are entry points for the folders they are in. The top level is `./src/lib.rs`. The whole project is a library, and every subsequent directory contains a private module included from there.

### src/lib.rs

Execution of the code starts in one of 3 procedural macros exported from lib.rs. `cui` is the main one, `test_setup` and `test_header` are helpers used for writing tests.

### Data flow

This diagram is helpful for remembering the order that the steps happen in, reference it to help with this section and if you get lost while navigating the code!

```
cui       -> lex     ->
tokens    -> parse*  ->
ast       -> analyze ->
semantics -> render  ->
semantics -> compile ->
html/wasm
```

\* this library starts at the parse step (`src/transform/parse.rs`), because lexing is taken care of us by some libraries, so we have some tokens to start

In words:
When a piece of CUI code is compiled, it is first lexed into tokens, which are then parsed into an [Abstract Syntax Tree](https://en.wikipedia.org/wiki/Abstract_syntax_tree) (AST). [Semantic analysis](<https://en.wikipedia.org/wiki/Semantic_analysis_(compilers)>) is performed on the AST to generate an object representing the meaning of the code. This "Semantics" object is rendered into an abstract representation of an HTML DOM, which is finally compiled into text strings containing HTML and CSS, and executables containing Webassembly.

Data flows between data structures (the `data` module, in the left column of the diagram) by way of transformations between those structures (the `transform` module, in the right column of the diagram). All of the code sits in the `src` directory, and starts in `lib.rs`, then passes to `transform/parse.rs` to create an object which is described in `data/ast.rs`, which are then passed to `transform/analyze.rs` to create a semantics object defined in `data/semantics.rs`, and so on through the diagram. The `write` transformation is defined in several parts, one for each of several outputs that don't resemble each other - HTML, CSS, and Rust code that is then compiled into a Wasm binary. Each output is generated with a different trait implemented on the Semantics struct.

This is the core of CUI (And could be fairly easy to adapt to other syntaxes!). In `src/misc` are files outside of this core flow, such as the helper `context` which is used during semantic analysis.

## Transformations in Detail

### Parse

Parsing takes tokens derived from some CUI input (which have been lexed for us by the `syn` crate) and it transforms them into an AST. The AST is a minimum representation of the input - it is close to 1:1 with the original code, but unlike the input code it is in a tree rather than a sequence of tokens. The AST should represent the minimum information necessary to reproduce a CUI program. For example:

```cui
.box {
     content {}
}
box {}
box {}
```

The root of the AST parsed from this code would contain a class block with an element block inside of it, and then 2 separate element blocks, like this:

```
// AST (made of blocks)
     / .box - content
page - box
     \ box
```

### Analyze and Render

Analyzing an AST generates a different tree structure which we call "semantics", which represents the meaning of the code in a way that continues to reflect the structure of the input, but which is filled out with information that will later let us transform the tree into a different shape, as well as any other information we need to gather along the way. To illustrate why the tree must be transformed, we can look at the tree structure of the desired output for the above code:

```
// DOM (made of elements)
page - box - content
     \ box - content
```

Analysis walks the AST (which we say consists of blocks) and creates a tree modeled after it (which we say consists of groups), and then runs steps we call rendering, which gather and fill out information in the groups, creating two-way links between element and class nodes in addition to the normal tree structure. In this case, these allow us to walk from either `box` group to the `.box` group, and from there to the `content` element group:

```
// semantics (made of groups, looks like AST)
     / .box - content
page - box
     \ box

// also has paths to be walked in this order
page - box - .box - content
     \ box /
```

Only one group node exists for the `.box` class, and only one for the `content` elements, but these elements can be reached from two places. This semantics tree can now be used to generate a DOM during rendering.

## Compile

The semantics tree compiles to an HTML document and a TokenStream of Rust code to be compiled into Webassembly. We create the HTML document in `compile/html.rs`. This step describes how to recursively generate a single string containing an HTML document that can then get written to a file, leveraging `compile/css.rs` to complete the contents of style tags and attributes. In a separate step, the TokenStream is generated by a complicated process in `compile/wasm`, and finally compiled by Rust where the macro invocation was. The HTML and CSS static content compilation steps are fairly simple compared to the Webassembly compilation step.

The assembled binary needs to know about the 3 kinds of groups - elements, classes, and listeners - and to be aware of 3 things we can do to each group - nothing, render it to the DOM, or register it to happen when an element matching some class is created later. We may do nothing to some groups if everything that needed to happen already happened during rendering and is captured in the static content.

The last complication we'll cover here is that the binary performs only limited operations for groups that have already been rendered, during something like an initialization step for the page. This code is generated by the "initialize" register and render functions, which run during build and produce simple procedural code, as opposed to the "runtime" register and render commands, which are whole functions that are called at runtime.

To be as clear as possible: The functions in `initialize/register.rs` and `initialize/render.rs` run at build time, whereas those in `runtime/register.rs` and `runtime/render.rs` will be called in a browser by the runtime. Note what's inside the `quote!` macros in each file.

## Tests

`./tests` has a collection of CUI examples that render different features, and then checks the DOM to see that it rendered as expected! Run them with `wasm-pack test --headless --firefox --chrome` (or just one browser; chrome happens to run faster). See `./tests/README.md` for details.

## Debugging

We use `cui-tools` to provide a server and some debugging tools to work on CUI. Use `git submodule update --init --recursive` to pull the contents of the directory if it is empty. Then, you can `cd cui-tools` and `./run.sh` to run the dev server.
