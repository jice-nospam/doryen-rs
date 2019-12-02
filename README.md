# doryen-rs

[![Build Status](https://travis-ci.org/jice-nospam/doryen-rs.svg)](https://travis-ci.org/jice-nospam/doryen-rs)
[![Documentation](https://docs.rs/doryen-rs/badge.svg)](https://docs.rs/doryen-rs)
[![crates.io](https://meritbadge.herokuapp.com/doryen-rs)](https://crates.io/crates/doryen-rs)
[![License: MIT](https://img.shields.io/badge/license-MIT-informational.svg)](#license)

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

![unicode](http://roguecentral.org/~jice/doryen-rs/unicode3.png)

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
+ [x] keyboard input : stable
+ [x] unicode support : stable
```

# usage
* add dependency to Cargo.toml :

```toml
[dependencies]
doryen-rs="*"
```

Check the examples and [documentation](https://docs.rs/doryen-rs) for more information.

# compilation

As of February 18 2019, both native and wasm targets compile on stable channel (rust 1.32.0 / stdweb 0.4.14).

* native compilation
```
cargo run --example basic
```

* web compilation
```
rustup target install wasm32-unknown-unknown
cargo install --force cargo-web
cargo web start --example basic
```

# license

This code is released under the MIT license.

# contributions

You can contribute to this library through pull requests. If you do so, please update the CHANGELOG.md and CREDITS.md files. If you provide a new feature, consider adding an example as a tutorial/showcase.