# doryen-rs

[![Build Status](https://travis-ci.org/jice-nospam/doryen-rs.svg)](https://travis-ci.org/jice-nospam/doryen-rs)

Ascii roguelike library in rust with native and wasm support.
Uses the webgl, uni-app and uni-glsl crates from the [unrust](http://github.com/unrust/unrust) game engine.

Demos :
* [Basic real-time walking @](http://roguecentral.org/~jice/doryen-rs/basic/)

![basic](http://roguecentral.org/~jice/doryen-rs/basic.png)

* [Performance test](http://roguecentral.org/~jice/doryen-rs/perf/)

![perf](http://roguecentral.org/~jice/doryen-rs/perf.png)

* [Fonts demo](http://roguecentral.org/~jice/doryen-rs/fonts/)

![fonts](http://roguecentral.org/~jice/doryen-rs/fonts.png)

# usage
* add dependency to Cargo.toml :

```toml
[dependencies]
doryen-rs { git = "https://github.com/jice-nospam/doryen-rs" }
```

Check the examples for more information.

# compilation

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
