Based on the https://github.com/rustwasm/wasm-pack-template.git

This project uses webpack-dev-server to locally serve a wasm binary compiled from a custom syntax called CWF. The language is implemented with Rust macros and gets compiled by Rust, and this environment exists to test and develop the language. The language implementation currently sits in `src/app/cwf.rs`, with example/WIP templates in `src/app.rs`.

To install, you will need:
1. [rustc/cargo](https://www.rust-lang.org/tools/install)
1. [node/npm](https://nodejs.org/en/download/)
1. [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

Then:
```
git clone https://github.com/thisminute/cascading-wasm-framework.git
cd cascading-wasm-framework
```

For windows users, run in the root directory:
```
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

Then:
```
cd www    # npm stuff is in the www directory
npm run clean # will run wasm-pack and npm install
npm start     # opens a new browser tab in watch mode for the binary!
```

Once started, try editing src/lib.rs. Example:

```cwf
div {
   text: hello;
}
span {
   text: world;
}
```
represents
```html
<div>
   hello
</div>
<span>
   world
</span>
```

Try editing the tag types or content of the elements, or adding more!
