CUI is a front end web language implemented with Rust procedural macros. Code compiled from CUI runs in a browser, and consists of an HTML component and a Webassembly binary.

To install, you will need:

1. [rustc/cargo](https://www.rust-lang.org/tools/install)
1. [node/npm](https://nodejs.org/en/download/)
1. [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

Then:

```bash
git clone https://github.com/thisminute/cascading-ui.git
cd cascading-ui
git submodule init
git submodule update
cd create-cui-app/www
npm install
npm start
```

Windows users may need to run the following in the root directory before `npm start`:

```bash
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

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

## Transformations in detail

Transformations are not entirely conceptually distinct from one another - some of the tasks that a transformation does could have been placed in a different transformation to achieve the same result. For example, different kinds of blocks (class, element, or event) are written to different arrays during parsing, but the same thing could be achieved by writing to a single `blocks` array during parsing and then determining what kind of block each block is later, during analysis.

A general principle is to place logic as early as it can happen without losing information that we need later. In our block parsing example, by determining whether a block is a class, element, or event in parsing, we lose the ability to tell whether a particular class came before or after another element. For example:

```cui
// 1
.some_rule {}
some_rule {}

// 2
some_rule {}
.some_rule {}
```

Both of these bits of code parse into exactly the same AST, because both create a `classes` array with one item and an `elements` array with one item, and it is impossible to tell after parsing which order they were in originally. This is in fact desired behavior! CUI syntax should not require us to place classes before elements for the rules to apply, and as there is currently no planned distinction between putting an element and a class in either order (note: the order of any element relative to other _elements_ on the other hand is very important, and preserved in the `elements` array), we are okay with completely eliminating that information from the pipeline in the very first transformation, and it saves us from having to worry about that information affecting something in a later transformation.

### Parse

Parsing takes tokens derived from some CUI input and provided to us by the `syn` crate, and it transforms them into an AST. The AST is a minimum representation of the input - it is close to 1:1 with the original code, but unlike the input code it is in a tree rather than a sequence of tokens. The AST should represent the minimum information necessary to reproduce a CUI program. For example:

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

To be very clear: The functions in `initialize/register.rs` and `initialize/render.rs` run at build time, whereas those in `runtime/register.rs` and `runtime/render.rs` will be called in a browser by the runtime. Note what's inside the `quote!` macros in each file.

## Tests

`./tests` has a collection of CUI examples that render different features, and then checks the DOM to see that it rendered as expected! Run them with `wasm-pack test --headless --firefox`. `--chrome` works too. See `./tests/README.md` for details.

## Debugging

We use `create-cui-app` to provide a server and some debugging tools to work on CUI. Use `git submodule update` to pull down the contents of the directory. `cd create-cui-app/www` and you will have access to some npm commands. Run `npm install` first. `npm start` will compile the test app, and `npm run debug` will run some steps that generate the macro output at `create-cui-app/target/cui_macro_output_formatted.rs` and also try to compile it. This is helpful when there is a build error in a bit of CUI code, but the library compiles, because it happens in code generated by the macro and the error messages just points at the macro, to see the error message in the generated code.
