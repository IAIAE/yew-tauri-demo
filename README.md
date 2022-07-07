# Tauri + Yew Demo

a demo(practice) project. has `yew -> js -> tauri`  async call. as well as `tauri -> js -> yew` message sending。

## pre-installed

```shell
rustup target add wasm32-unknown-unknown
cargo install trunk
cargo install wasm-bindgen-cli
cargo install tauri-cli --version ^1.0.0
```

## Dev

After installing the above, you should be able to run it with

```shell
cargo tauri dev
```

## Build

You can do a release build with

```shell
cargo tauri build --verbose
```

and if you want debug in released app. use `--debug`:

```shell
cargo tauri build --verbose --debug
```

an build app in folder: `src-tauri/target/release/bundle/`


## learn more

Tauri: https://tauri.studio/en/docs/get-started/intro

Yew: https://yew.rs/docs/getting-started/introduction

wasm-bindgen-usage: https://rustwasm.github.io/wasm-bindgen/reference/passing-rust-closures-to-js.html

trunk: https://trunkrs.dev/

## about Content Security Policy.

i use `dangerousDisableAssetCspModification` in `tauri.conf.json`. see reason below[tauri csp bug when use yew(wasm)].

- learn csp: https://content-security-policy.com/
- tauri csp bug when use yew(wasm): https://github.com/tauri-apps/tauri/issues/3583


