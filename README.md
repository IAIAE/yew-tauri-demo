# Tauri + Yew Demo

练手项目，包含yew -> js -> tauri 的异步调用过程。同时包含tauri -> js -> yew的通信过程。

## Installation

```shell
rustup target add wasm32-unknown-unknown
cargo install trunk
cargo install wasm-bindgen-cli
cargo install tauri-cli --version ^1.0.0
```

## Dev Server 

After installing the above, you should be able to run it with

```shell
cargo tauri dev
```

## Building the app

You can do a release build with

```shell
cargo tauri build
```

This should create an installer in src-tauri/target/release/bundle/

## Further reading

Tauri: https://tauri.studio/en/docs/get-started/intro

Yew: https://yew.rs/docs/getting-started/introduction

wasm-bindgen-usage: https://rustwasm.github.io/wasm-bindgen/reference/passing-rust-closures-to-js.html

trunk: https://trunkrs.dev/

