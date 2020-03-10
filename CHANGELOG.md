# 2.0.0 - TBD
* switched from unrust to bracket-lib
## API breaks
* fonts path are defined in `AppOptions.font_paths`. `DoryenApi.set_font_path` replaced with `DoryenApi.set_font_index`
* On native target, the window is always resizeable. If `AppOptions.resizeable` is true, the console is resized with the window, else it's scaled (default behavior). You don't have to call `Console.resize`anymore (see resize example).
* `DoryenApi.average_fps()` was removed. Use `DoryenApi.fps()` instead.
* `Engine.resize()` was removed. Console resizing is done automatically when `AppOptions.resizable` is true

# 1.2.4 - TBD
## features
* added option to return `UpdateEvent::Capture` from the `update()` function to capture in-game screenshots

# 1.2.3 - 2020 Jan 08
## fixed
* fixed #22 : mouse coordinates broken when console is resized
* fixed resizable console not taking hidpi factor into account

# 1.2.2 - 2020 Jan 03
## fixed
* fixed #21 : keyboard/mouse events lost when framerate > tickrate

# 1.2.1 - 2019 Dec 08
## fixed
* `Console.blit` not taking source console alpha value into account
* fixed #19 : rendering unicode characters >= 0x00FF
* fixed #10 : console not centered in fullscreen and wrong mouse coordinates
## features
* `AppOptions` now implements `Default`
* added `InputApi.keys_released()` and `InputApi.keys_pressed()` that return iterators on key events since last update
* added text input support through `InputApi.text()` (see text_input example)
* added alpha example showcasing framebuffer overdrawing
* provided default `update()` and `resize()` functions. Now `Engine` must only provide a render method
* new Image methods : `new_empty()`, `pixel()`, `put_pixel()`
* added a visual demo showcasing subcell resolution + dynamic lighting in a real time roguelike

# 1.2.0 - 2019 Nov 22
## fixed
* fix #13 Console.print_color with text containing ']'
## API breaks
* added `AppOptions.intercept_close_request` to intercept clicks on the window close button (native only). See 'exit' example
## features
* added `Console.text_color_len()`

# 1.1.0 - 2019 Sep 18
## fixes
* fix mouse coordinates on HiDpi screens
* web : fix #8 black borders on HiDpi screens
## API breaks
* `Image.is_loaded()` : renamed to `Image.try_load()` (see https://rust-lang.github.io/rust-clippy/master/index.html#wrong_self_convention)
* `Image.get_size()` : renamed to `Image.try_get_size()`
* `FileLoader.is_file_ready()` : renamed to `FileLoader.check_file_ready()`
* `Color` parameter in `color_blend()` and `color_dist()` are now passed by value (see https://rust-lang.github.io/rust-clippy/master/index.html#trivially_copy_pass_by_ref)

# 1.0.1 - 2019 Feb 15
## fixes
* fix HiDpi display on Mac
* fix panic on start when font is too slow to load

# 1.0.0 - 2019 Feb 1
## API breaks
* `Console.print_color()` : replace %{} color marker with #[]
* `resize()` function added to `Engine` trait to allow console resize

# 0.1.0 - 2018 Aug 3
First release
