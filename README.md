# Tauri + Yew Demo

This is a small demo to accompany the Tauri + Yew tutorial

https://dev.to/stevepryde/create-a-desktop-app-in-rust-using-tauri-and-yew-2bhe

## Installation

```shell
rustup target add wasm32-unknown-unknown
cargo install trunk
cargo install wasm-bindgen-cli
cargo install tauri-cli --version ^1.0.0-beta
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



can js hold some persistent callback from yew?
i tried this:
```rust
let bbb = ctx.link().callback(|e|  Msg::JSEvt(e));
let yewcb = Closure::new(move |evt:JsValue| {
    bbb.emit(evt);
}).into_js_value();
binding::jsSetYewCb(yewcb);
```
but the compiler err at `Closure::new`, see: 
```
main.rs(117, 17): `self` is a reference that is only valid in the associated function body
main.rs(117, 17): let's call the lifetime of this reference `'1`
```
means i cannot hold the self