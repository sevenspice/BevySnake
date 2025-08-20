[![Netlify Status](https://api.netlify.com/api/v1/badges/e2572855-11b7-4b3c-a640-60e1278a7da8/deploy-status)](https://app.netlify.com/projects/roaring-haupia-17e1e3/deploys)

# 概要

- [Bevyでスネークゲームを作成する](https://xn--p8j7a4jn57j.com/entry/20250804/1754280831)

# ビルド方法
``` bash
cargo build
```

## 実行
```bash
cargo run
```

# ビルド方法(WASM)

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

## 実行

``` bash
cargo +nightly install miniserve
miniserve ./ --index index.html
# ブラウザで http://127.0.0.1:8080 へアクセス
```
