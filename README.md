# 概要

- []()

# ビルド方法

## 参考

- [Browser (WebAssembly)](https://bevy-cheatbook.github.io/platforms/wasm.html)
- [Create a Custom Web Page](https://bevy-cheatbook.github.io/platforms/wasm/webpage.html)

## 手順

``` bash
rustup target install wasm32-unknown-unknown
```
``` bash
cargo install wasm-bindgen-cli
```
``` bash
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web --out-dir ./ --out-name "snake" ./target/wasm32-unknown-unknown/release/bevy_snake.wasm
```
``` bash
cargo +nightly install miniserve
miniserve ./ --index index.html
# ブラウザで http://127.0.0.1:8080 へアクセス
```
