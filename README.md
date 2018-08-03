# doryen-rs

[![Build Status](https://travis-ci.org/jice-nospam/doryen-rs.svg)](https://travis-ci.org/jice-nospam/doryen-rs)
[![Documentation](https://docs.rs/doryen-rs/badge.svg)](https://docs.rs/doryen-rs)
[![crates.io](https://meritbadge.herokuapp.com/doryen-rs)](https://crates.io/crates/doryen-rs)

Ascii roguelike library in rust with native and wasm support.
Uses the uni-gl and uni-app crates from the [unrust](http://github.com/unrust/unrust) game engine.

Demos :
* [Basic real-time walking @](http://roguecentral.org/~jice/doryen-rs/basic/)

![basic](http://roguecentral.org/~jice/doryen-rs/basic.png)

* [Performance test](http://roguecentral.org/~jice/doryen-rs/perf/)

![perf](http://roguecentral.org/~jice/doryen-rs/perf.png)

* [Fonts demo](http://roguecentral.org/~jice/doryen-rs/fonts/)

![fonts](http://roguecentral.org/~jice/doryen-rs/colored.png)

* [Unicode demo](http://roguecentral.org/~jice/doryen-rs/unicode/)

![unicode](http://roguecentral.org/~jice/doryen-rs/unicode2.png)

* [Console blitting demo](http://roguecentral.org/~jice/doryen-rs/blit/)

![blit](http://roguecentral.org/~jice/doryen-rs/blit.png)

* [Image blitting demo](http://roguecentral.org/~jice/doryen-rs/image/)

![image](http://roguecentral.org/~jice/doryen-rs/image.png)

* [Subcell resolution demo](http://roguecentral.org/~jice/doryen-rs/subcell/)

![subcell](http://roguecentral.org/~jice/doryen-rs/subcell.png)

# status
```diff
+ [x] GLSL renderer: stable
+ [x] RGBA, RGB and greyscale fonts : stable
+ [x] mouse input : stable
+ [x] subcell resolution : stable
+ [x] PNG image blitting : stable
- [x] unicode support : beta
- [x] keyboard input : alpha
```

# usage
* add dependency to Cargo.toml :

```toml
[dependencies]
doryen-rs="*"
```

Check the examples and [documentation](https://docs.rs/doryen-rs) for more information.

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
