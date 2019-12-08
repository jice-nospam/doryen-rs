# doryen-rs

[![Build Status](https://travis-ci.org/jice-nospam/doryen-rs.svg)](https://travis-ci.org/jice-nospam/doryen-rs)
[![Documentation](https://docs.rs/doryen-rs/badge.svg)](https://docs.rs/doryen-rs)
[![crates.io](https://meritbadge.herokuapp.com/doryen-rs)](https://crates.io/crates/doryen-rs)
[![License: MIT](https://img.shields.io/badge/license-MIT-informational.svg)](#license)

Ascii roguelike library in rust with native and wasm support.
Uses the uni-gl and uni-app crates from the [unrust](http://github.com/unrust/unrust) game engine.

Demos :
* [Visual demo](https://jice-nospam.github.io/doryen-rs/demo/)

![demo](https://raw.githubusercontent.com/jice-nospam/doryen-rs/master/docs/demo_miniatures/demo.png)

* [Basic real-time walking @](https://jice-nospam.github.io/doryen-rs/basic/)

![basic](https://raw.githubusercontent.com/jice-nospam/doryen-rs/master/docs/demo_miniatures/basic.png)

* [Performance test](https://jice-nospam.github.io/doryen-rs/perf/)

![perf](https://raw.githubusercontent.com/jice-nospam/doryen-rs/master/docs/demo_miniatures/perf.png)

* [Fonts demo](https://jice-nospam.github.io/doryen-rs/fonts/)

![fonts](https://raw.githubusercontent.com/jice-nospam/doryen-rs/master/docs/demo_miniatures/fonts.png)

* [Unicode demo](https://jice-nospam.github.io/doryen-rs/unicode/)

![unicode](https://raw.githubusercontent.com/jice-nospam/doryen-rs/master/docs/demo_miniatures/unicode.png)

* [Console blitting demo](https://jice-nospam.github.io/doryen-rs/blit/)

![blit](https://raw.githubusercontent.com/jice-nospam/doryen-rs/master/docs/demo_miniatures/blit.png)

* [Image blitting demo](https://jice-nospam.github.io/doryen-rs/image/)

![image](https://raw.githubusercontent.com/jice-nospam/doryen-rs/master/docs/demo_miniatures/image.png)

* [Subcell resolution demo](https://jice-nospam.github.io/doryen-rs/subcell/)

![subcell](https://raw.githubusercontent.com/jice-nospam/doryen-rs/master/docs/demo_miniatures/subcell.png)

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
cargo install --force cargo-web
cargo web start --example basic
```

# license

This code is released under the MIT license.

# contributions

You can contribute to this library through pull requests. If you do so, please update the CHANGELOG.md and CREDITS.md files. If you provide a new feature, consider adding an example as a tutorial/showcase.