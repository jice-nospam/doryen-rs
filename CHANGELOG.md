# 1.2.1 - TBD
## features
* `AppOptions` now implements `Default`
* added `InputApi.keys_released` and `InputApi.keys_pressed` that return iterators on key events since last update

# 1.2.0 - 2019 Nov 22
## fixed
* fix #13 Console.print_color with text containing ']'
## API breaks
* added `AppOptions.intercept_close_request` to intercept clicks on the window close button (native only). See 'exit' example
## features
* added `Console.text_color_len`

# 1.1.0 - 2019 Sep 18
## fixes
* fix mouse coordinates on HiDpi screens
* web : fix #8 black borders on HiDpi screens
## API breaks
* `Image.is_loaded` : renamed to `Image.try_load` (see https://rust-lang.github.io/rust-clippy/master/index.html#wrong_self_convention)
* `Image.get_size` : renamed to `Image.try_get_size`
* `FileLoader.is_file_ready` : renamed to `FileLoader.check_file_ready`
* `Color` parameter in `color_blend` and `color_dist` are now passed by value (see https://rust-lang.github.io/rust-clippy/master/index.html#trivially_copy_pass_by_ref)

# 1.0.1 - 2019 Feb 15
## fixes
* fix HiDpi display on Mac
* fix panic on start when font is too slow to load

# 1.0.0 - 2019 Feb 1
## API breaks
* `Console.print_color` : replace %{} color marker with #[]
* `resize` function added to `Engine` trait to allow console resize

# 0.1.0 - 2018 Aug 3
First release
