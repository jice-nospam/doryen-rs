# doryen-rs

[![Build Status](https://travis-ci.org/jice-nospam/doryen-rs.svg?branch=master)](https://travis-ci.org/jice-nospam/doryen-rs)

Ascii roguelike library in rust with native and wasm support.
Uses the webgl, uni-app and uni-glsl crates from the [unrust](http://github.com/unrust/unrust) game engine.

# usage
* add dependency to Cargo.toml :

```toml
[dependencies]
doryen-rs { git = "https://github.com/jice-nospam/doryen-rs" }
```

Check the examples for more information.

You should create a `doryen-rs::App` and provide a trait object implementing `doryen-rs::Engine` :

```rust
fn main() {
    let mut app = App::new(CONSOLE_WIDTH, CONSOLE_HEIGHT,
        "my roguelike", "terminal8x8_aa_ro.png", 128, 128);
    app.set_engine(Box::new(MyRoguelike::new()));
    app.run();
}
```

MyRoguelike can draw on the console and get user input through the `Engine` interface :
```rust
pub trait Engine {
    fn update(&mut self, input: &mut InputApi);
    fn render(&self, con: &mut Console);
}
```

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
