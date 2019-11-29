use std::collections::HashMap;
use std::iter::Filter;

use uni_app::AppEvent;

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

pub struct DoryenInput {
    kdown: HashMap<String, bool>,
    kpressed: HashMap<String, bool>,
    kreleased: HashMap<String, bool>,
    mdown: HashMap<usize, bool>,
    mpressed: HashMap<usize, bool>,
    mreleased: HashMap<usize, bool>,
    text: String,
    close_request: bool,
    mpos: (f32, f32),
    screen_size: (f32, f32),
    con_size: (f32, f32),
}

impl DoryenInput {
    pub fn new(
        screen_width: u32,
        screen_height: u32,
        con_width: u32,
        con_height: u32,
    ) -> DoryenInput {
        DoryenInput {
            kdown: HashMap::new(),
            kpressed: HashMap::new(),
            kreleased: HashMap::new(),
            mdown: HashMap::new(),
            mpressed: HashMap::new(),
            mreleased: HashMap::new(),
            mpos: (0.0, 0.0),
            text: String::new(),
            close_request: false,
            screen_size: (screen_width as f32, screen_height as f32),
            con_size: (con_width as f32, con_height as f32),
        }
    }
    fn on_key_down(&mut self, scan_code: &str) {
        if !self.key(scan_code) {
            self.kpressed.insert(scan_code.to_owned(), true);
            self.kdown.insert(scan_code.to_owned(), true);
        }
    }
    fn on_key_up(&mut self, scan_code: &str) {
        self.kpressed.insert(scan_code.to_owned(), false);
        self.kdown.insert(scan_code.to_owned(), false);
        self.kreleased.insert(scan_code.to_owned(), true);
    }
    fn on_mouse_down(&mut self, button: usize) {
        if !self.mouse_button(button) {
            self.mpressed.insert(button, true);
            self.mdown.insert(button, true);
        }
    }
    fn on_mouse_up(&mut self, button: usize) {
        self.mpressed.insert(button, false);
        self.mdown.insert(button, false);
        self.mreleased.insert(button, true);
    }
    pub fn on_frame(&mut self) {
        self.mpressed.clear();
        self.mreleased.clear();
        self.kreleased.clear();
        self.kpressed.clear();
        self.close_request = false;
        self.text.clear();
    }
    pub fn on_event(&mut self, event: &AppEvent) {
        match event {
            AppEvent::KeyDown(ref key) => {
                self.on_key_down(&key.code);
            }
            AppEvent::KeyUp(ref key) => {
                self.on_key_up(&key.code);
            }
            AppEvent::CharEvent(ch) => {
                if !ch.is_control() {
                    self.text.push(*ch);
                }
            }
            AppEvent::MousePos(ref pos) => {
                self.mpos = (
                    pos.0 as f32 / self.screen_size.0 * self.con_size.0,
                    pos.1 as f32 / self.screen_size.1 * self.con_size.1,
                );
            }
            AppEvent::MouseDown(ref mouse) => {
                self.on_mouse_down(mouse.button);
            }
            AppEvent::MouseUp(ref mouse) => {
                self.on_mouse_up(mouse.button);
            }
            AppEvent::Resized(size) => {
                self.resize(*size);
            }
            AppEvent::CloseRequested => {
                self.close_request = true;
            }
        }
    }
    fn resize(&mut self, size: (u32, u32)) {
        self.screen_size = (size.0 as f32, size.1 as f32);
    }
}

impl InputApi for DoryenInput {
    fn key(&self, scan_code: &str) -> bool {
        match self.kdown.get(scan_code) {
            Some(&true) => true,
            _ => false,
        }
    }
    fn key_pressed(&mut self, scan_code: &str) -> bool {
        match self.kpressed.get(scan_code) {
            Some(&true) => true,
            _ => false,
        }
    }
    fn keys_pressed(&self) -> Keys {
        Keys {
            inner: self.kpressed.iter().filter(|&(_, &v)| v),
        }
    }
    fn key_released(&mut self, scan_code: &str) -> bool {
        match self.kreleased.get(scan_code) {
            Some(&true) => true,
            _ => false,
        }
    }
    fn keys_released(&self) -> Keys {
        Keys {
            inner: self.kreleased.iter().filter(|&(_, &v)| v),
        }
    }
    fn text(&self) -> String {
        self.text.to_owned()
    }
    fn mouse_button(&self, num: usize) -> bool {
        match self.mdown.get(&num) {
            Some(&true) => true,
            _ => false,
        }
    }
    fn mouse_button_pressed(&mut self, num: usize) -> bool {
        match self.mpressed.get(&num) {
            Some(&true) => true,
            _ => false,
        }
    }
    fn mouse_button_released(&mut self, num: usize) -> bool {
        match self.mreleased.get(&num) {
            Some(&true) => true,
            _ => false,
        }
    }
    fn mouse_pos(&self) -> (f32, f32) {
        self.mpos
    }
    fn close_requested(&self) -> bool {
        self.close_request
    }
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
