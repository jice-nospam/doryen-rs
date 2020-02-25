use std::iter::Filter;

/// Provides information about user input.
/// Possible values for the `key` scancode parameter can be found in unrust/uni-app's `translate_scan_code`
/// [function](https://github.com/unrust/uni-app/blob/41246b070567e3267f128fff41ededf708149d60/src/native_keycode.rs#L160).
/// Warning, there are some slight variations from one OS to another, for example the `Command`, `F13`, `F14`, `F15` keys
/// only exist on Mac.
///
/// State functions like [`InputApi::key`], [`InputApi::mouse_button`] and [`InputApi::mouse_pos`] always work.
/// On another hand, pressed/released event functions should be called only in the update function.
///
pub trait InputApi {
    // keyboard
    /// return the current status of a key (true if pressed)
    fn key(&self, key: &str) -> bool;
    /// return true if a key was pressed since last update.
    fn key_pressed(&mut self, key: &str) -> bool;
    /// return an iterator over all the keys that were pressed since last update.
    fn keys_pressed(&self) -> Keys;
    /// return true if a key was released since last update.
    fn key_released(&mut self, key: &str) -> bool;
    /// return an iterator over all the keys that were released since last update.
    fn keys_released(&self) -> Keys;
    /// characters typed since last update
    fn text(&self) -> String;
    // mouse
    /// return the current status of a mouse button (true if pressed)
    fn mouse_button(&self, num: usize) -> bool;
    /// return true if a mouse button was pressed since last update.
    fn mouse_button_pressed(&mut self, num: usize) -> bool;
    /// return true if a mouse button was released since last update.
    fn mouse_button_released(&mut self, num: usize) -> bool;
    /// return the current mouse position in console cells coordinates (float value to have subcell precision)
    fn mouse_pos(&self) -> (f32, f32);
    /// Whether the window close button was clicked
    fn close_requested(&self) -> bool;
}

type KeyMapFilter<'a> =
    Filter<std::collections::hash_map::Iter<'a, String, bool>, fn(&(&'a String, &'a bool)) -> bool>;

/// An iterator visiting all keys in arbitrary order.
pub struct Keys<'a> {
    inner: KeyMapFilter<'a>,
}

impl<'a> Iterator for Keys<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, _)| k.as_ref())
    }
}
