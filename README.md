# doryen-rs

[![Build Status](https://travis-ci.org/jice-nospam/doryen-rs.svg)](https://travis-ci.org/jice-nospam/doryen-rs)
[![Documentation](https://docs.rs/doryen-rs/badge.svg)](https://docs.rs/doryen-rs)
[![crates.io](https://meritbadge.herokuapp.com/doryen-rs)](https://crates.io/crates/doryen-rs)
[![License: MIT](https://img.shields.io/badge/license-MIT-informational.svg)](#license)

Ascii roguelike library in rust with native and wasm support.
Uses the uni-gl and uni-app crates from the [unrust](http://github.com/unrust/unrust) game engine.

# features
```diff
+ [x] GLSL renderer
+ [x] RGBA, RGB and greyscale fonts
+ [x] mouse input
+ [x] keyboard input
+ [x] subcell resolution
+ [x] PNG image blitting
+ [x] unicode support
```

# demos

<table>
  <tr><td>
    <a href="https://jice-nospam.github.io/doryen-rs/docs/demo/">Visual demo<br/>
        <img src="https://raw.githubusercontent.com/jice-nospam/doryen-rs/master/docs/demo_miniatures/demo.png"/>
    </a>
</td><td>
    <a href="https://jice-nospam.github.io/doryen-rs/docs/basic/">Basic real-time walking @<br/>
        <img src="https://raw.githubusercontent.com/jice-nospam/doryen-rs/master/docs/demo_miniatures/basic.png"/>
    </a>
</td></tr><tr><td>
    <a href="https://jice-nospam.github.io/doryen-rs/docs/perf/">Performance test<br/>
        <img src="https://raw.githubusercontent.com/jice-nospam/doryen-rs/master/docs/demo_miniatures/perf.png"/>
    </a>
</td><td>
    <a href="https://jice-nospam.github.io/doryen-rs/docs/fonts/">Fonts demo<br/>
        <img src="https://raw.githubusercontent.com/jice-nospam/doryen-rs/master/docs/demo_miniatures/fonts.png"/>
    </a>
</td></tr><tr><td>
    <a href="https://jice-nospam.github.io/doryen-rs/docs/unicode/">Unicode demo<br/>
        <img src="https://raw.githubusercontent.com/jice-nospam/doryen-rs/master/docs/demo_miniatures/unicode.png"/>
    </a>
</td><td>
    <a href="https://jice-nospam.github.io/doryen-rs/docs/blit/">Console blitting demo<br/>
        <img src="https://raw.githubusercontent.com/jice-nospam/doryen-rs/master/docs/demo_miniatures/blit.png"/>
    </a>
</td></tr><tr><td>
    <a href="https://jice-nospam.github.io/doryen-rs/docs/image/">Image blitting demo<br/>
        <img src="https://raw.githubusercontent.com/jice-nospam/doryen-rs/master/docs/demo_miniatures/image.png"/>
    </a>
</td><td>
    <a href="https://jice-nospam.github.io/doryen-rs/docs/subcell/">Subcell resolution demo<br/>
        <img src="https://raw.githubusercontent.com/jice-nospam/doryen-rs/master/docs/demo_miniatures/subcell.png"/>
    </a>
</td></tr><tr><td>
    <a href="https://jice-nospam.github.io/doryen-rs/docs/alpha/">Transparent console demo<br/>
        <img src="https://raw.githubusercontent.com/jice-nospam/doryen-rs/master/docs/demo_miniatures/alpha.png"/>
    </a>
</td><td>
    <a href="https://jice-nospam.github.io/doryen-rs/docs/text_input/">Text input demo<br/>
        <img src="https://raw.githubusercontent.com/jice-nospam/doryen-rs/master/docs/demo_miniatures/text_input.png"/>
    </a>
</td></tr></table>

# usage
* add dependency to Cargo.toml :

```toml
[dependencies]
doryen-rs="*"
```

Check the [examples](https://github.com/jice-nospam/doryen-rs/tree/master/examples) and [documentation](https://docs.rs/doryen-rs) for more information.

# compilation

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
