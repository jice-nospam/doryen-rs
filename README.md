# doryen-rs
Ascii roguelike library in rust with native and wasm support.
Uses the webgl, uni-app and uni-glsl crates from the [unrust](http://github.com/unrust/unrust) game engine.

# usage
* add dependency to Cargo.toml :

```
[dependencies]
doryen-rs {git="https://github.com/jice-nospam/doryen-rs"}
```

* native compilation
```
rustup default nightly
cargo run --example basic
```

* web compilation
```
rustup default nightly
rustup target install wasm32-unknown-unknown
cargo install cargo-web
cargo web start --example basic
```
