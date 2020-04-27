Based on the https://github.com/rustwasm/wasm-pack-template.git

This project uses webpack-dev-server to locally serve a wasm binary compiled from a custom syntax called CWF. The language is implemented with Rust macros and gets compiled by Rust, and this environment exists to test and develop the language. The language implementation currently sits in `src/app/cwf.rs`, with example/WIP templates in `src/app.rs`.

To install, you will need cargo/rustc and npm/node. Then:

```bash
git clone https://github.com/thisminute/cwf.git
cd cwf/www    # npm stuff is in the www directory
npm run clean # will run wasm-pack and npm install
npm start     # opens a new browser tab in watch mode for the binary!
```

Once started, try editing src/app.rs. Example:

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
