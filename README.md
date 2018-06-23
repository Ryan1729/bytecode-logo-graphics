# Stack-based WASM Shenzhen solitaire

This is a fork of [another repo of mine](https://github.com/Ryan1729/stack-based-wasm-shenzhen-solitaire) which is being made largely on a lark. 

What follows is the README for the repo linked above.

# Stack-based WASM Shenzhen solitaire

This is a fork of [my own non-stack-based version of the same game](https://github.com/Ryan1729/wasm_shenzhen_solitaire). You are most likely to be more interested in the other one since this one exists just for my own practice at expressing solitaire games in a stack-based style since I plan to do so in another project.

What follows is the README for the version linked above, so the playable version link is for that version. Note though the licensing that the license applies to both versions.

# WASM Shenzhen solitaire

This is a port of [Hunter X](https://www.lexaloffle.com/bbs/?uid=26640)'s [pico-8 version](https://www.lexaloffle.com/bbs/?pid=46634&tid=30310) of [Zachtronic](https://www.zachtronics.com)'s [Shenzhen Solitaire](http://store.steampowered.com/app/570490/SHENZHEN_SOLITAIRE/).

## Playable version

[Right here](https://ryan1729.github.io/wasm_shenzhen_solitaire/).

Use z, x, and the arrow keys to play. If you win, you can press Enter to deal another game.

### Building (using Rust's native WebAssembly backend)

1. Install newest nightly Rust:

       $ curl https://sh.rustup.rs -sSf | sh

2. Install WebAssembly target:

       $ rustup target add wasm32-unknown-unknown

3. Install [cargo-web]:

       $ cargo install -f cargo-web

4. Build it:

       $ cargo web start --target wasm32-unknown-unknown --release

5. Visit `http://localhost:8000` with your browser.

[cargo-web]: https://github.com/koute/cargo-web

### Building for other backends

Replace `--target-webasm` with `--target wasm32-unknown-emscripten` or `--target asmjs-unknown-emscripten`
if you want to build it using another backend. You will also have to install the
corresponding targets with `rustup` - `wasm32-unknown-emscripten` and `asmjs-unknown-emscripten`
respectively.

___

licensed under Apache, MIT and CC BY-NC-SA 4.0.
