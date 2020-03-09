use std::collections::HashSet;

use bracket::prelude::*;

use crate::console;
use crate::input::{translate_scan_code, translate_virtual_key, InputApi, Keys};

// fps
const TICKS_PER_SECOND: f64 = 60.0;
const SKIP_TICKS: f64 = 1.0 / TICKS_PER_SECOND;

// default options
pub const DEFAULT_CONSOLE_WIDTH: u32 = 80;
pub const DEFAULT_CONSOLE_HEIGHT: u32 = 50;

/// This is the complete doryen-rs API provided to you by [`App`] in [`Engine::update`] and [`Engine::render`] methods.
pub trait DoryenApi {
    /// return the root console that you can use to draw things on the screen
    fn con(&mut self) -> &mut console::Console;
    /// return the input API to check user mouse and keyboard input
    fn input(&mut self) -> &mut dyn InputApi;
    /// return the current framerate
    fn fps(&self) -> u32;
    /// return the average framerate since the start of the game
    fn average_fps(&self) -> u32;
    /// replace the current font by a new one.
    /// Put your font in the static/ directory of the project to make this work with both `cargo run` and `cargo web start`.
    /// Example
    /// ```compile_fail
    /// let mut app = App::new(AppOptions {
    ///     font_paths:vec!["terminal.png".to_owned(),"terminal2.png".to_owned()],
    ///     ..Default::default()
    /// });
    /// ...
    /// // Inside your update() function :
    /// api.set_font_index(1); // switch to terminal2.png font
    /// ```
    /// During development, this references `$PROJECT_ROOT/static/terminal.png`.
    /// Once deployed, `terminal.png` should be in the same directory as your game's executable or `index.html`.
    ///
    /// By default, doryen-rs will assume the font has a 16x16 extended ASCII layout. The character size will be calculated with :
    /// ```text
    /// char_width = font_image_width / 16
    /// char_height = font_image_height / 16
    /// ```
    /// If your font has a different layout (that's the case in the unicode example), you have to provide the character size by appending it to the font file name :
    /// ```text
    /// myfont_8x8.png
    /// ```
    ///
    /// doryen_rs support several font format. It uses the top left pixel of the font to determin the format.
    /// * If the top-left pixel alpha value is < 255, this is an RGBA font.
    /// * If the top-left pixel alpha value is 255 and its color is black, this is a greyscale font.
    /// * Else, it's an RGB font.
    ///
    /// * RGBA : transparency is stored in alpha channel. It can have semi-transparent pixels of any color. The picture below shows on the left the font image and on the right how it appears when the characters are drawn on a blue background.
    /// ![rgba](http://roguecentral.org/~jice/doryen-rs/rgba.png)
    /// * greyscale : black pixels are transparent. Grey pixels are replaced by white semi-transparent pixels. Colored pixels are opaque. The font cannot have pure grey colors.
    /// ![greyscale](http://roguecentral.org/~jice/doryen-rs/greyscale.png)
    /// * RGB : The top-left pixel's color is transparent. The font cannot have semi-transparent pixels but it can have pure grey pixels.
    /// ![rgb](http://roguecentral.org/~jice/doryen-rs/rgb.png)
    fn set_font_index(&mut self, font_index: usize);
    /// return the current screen size
    fn get_screen_size(&self) -> (u32, u32);
}

struct BracketInput {
    pub mouse_pos: (f32, f32),
    pub mouse_left: bool,
    pub close_requested: bool,
    pub text: String,
    key_press: HashSet<VirtualKeyCode>,
    key_release: HashSet<VirtualKeyCode>,
    mouse_press: HashSet<usize>,
    mouse_release: HashSet<usize>,
    pub new_size: (u32, u32),
}

impl BracketInput {
    pub fn new(con_size: (u32, u32)) -> Self {
        Self {
            mouse_pos: (0.0, 0.0),
            mouse_left: false,
            close_requested: false,
            text: String::new(),
            key_press: HashSet::new(),
            key_release: HashSet::new(),
            mouse_press: HashSet::new(),
            mouse_release: HashSet::new(),
            new_size: con_size,
        }
    }
    pub fn clear(&mut self) {
        self.text.clear();
        self.close_requested = false;
        self.key_press.clear();
        self.mouse_press.clear();
        self.key_release.clear();
        self.mouse_release.clear();
    }
    pub fn update(&mut self, char_size: (u32, u32)) {
        self.clear();
        let mut input = INPUT.lock().unwrap();
        let (mx, my) = input.mouse_pixel_pos().into();
        self.mouse_pos = (
            mx as f32 / char_size.0 as f32,
            my as f32 / char_size.1 as f32,
        );
        self.mouse_left = input.is_mouse_button_pressed(0);
        while let Some(evt) = input.pop() {
            match evt {
                BEvent::CloseRequested => self.close_requested = true,
                BEvent::Character { c } => {
                    if !c.is_control() {
                        self.text.push(c);
                    }
                }
                BEvent::KeyboardInput { key, pressed, .. } => {
                    if pressed {
                        self.key_press.insert(key);
                    } else {
                        self.key_release.insert(key);
                    }
                }
                BEvent::MouseClick { button, pressed } => {
                    if pressed {
                        self.mouse_press.insert(button);
                    } else {
                        self.mouse_release.insert(button);
                    }
                }
                BEvent::Resized { new_size } => {
                    self.new_size = (new_size.x as u32, new_size.y as u32)
                }
                _ => (),
            }
        }
    }
}

impl InputApi for BracketInput {
    fn key(&self, scan_code: &str) -> bool {
        let input = INPUT.lock().unwrap();
        input.is_scancode_pressed(translate_scan_code(scan_code))
    }
    fn keys_pressed(&self) -> Keys {
        // TODO BRACKET
        unreachable!()
    }
    fn keys_released(&self) -> Keys {
        // TODO BRACKET
        unreachable!()
    }
    fn key_pressed(&mut self, key_code: &str) -> bool {
        if let Some(key) = translate_virtual_key(key_code) {
            return self.key_press.contains(&key);
        }
        false
    }
    fn key_released(&mut self, key_code: &str) -> bool {
        if let Some(key) = translate_virtual_key(key_code) {
            return self.key_release.contains(&key);
        }
        false
    }
    fn text(&self) -> String {
        self.text.clone()
    }
    fn mouse_button(&self, num: usize) -> bool {
        let input = INPUT.lock().unwrap();
        input.is_mouse_button_pressed(num)
    }
    fn mouse_button_pressed(&mut self, num: usize) -> bool {
        self.mouse_press.contains(&num)
    }
    fn mouse_button_released(&mut self, num: usize) -> bool {
        self.mouse_release.contains(&num)
    }
    fn mouse_pos(&self) -> (f32, f32) {
        self.mouse_pos
    }
    fn close_requested(&self) -> bool {
        self.close_requested
    }
}

struct DoryenApiImpl<'a> {
    con: &'a mut console::Console,
    bracket_input: BracketInput,
    fps: u32,
    average_fps: u32,
    font_index: usize,
    con_size: (u32, u32),
}

impl<'a> DoryenApiImpl<'a> {
    pub fn new(con: &'a mut console::Console, fps: u32, char_size: (u32, u32)) -> Self {
        let con_size = (con.get_width(), con.get_height());
        let bracket_input = BracketInput::new((con_size.0 * char_size.0, con_size.1 * char_size.1));
        Self {
            con,
            bracket_input,
            fps,
            average_fps: fps,
            font_index: 0,
            con_size,
        }
    }
}

impl<'a, 'b> DoryenApi for DoryenApiImpl<'a> {
    fn con(&mut self) -> &mut console::Console {
        &mut self.con
    }
    fn input(&mut self) -> &mut dyn InputApi {
        &mut self.bracket_input
    }
    fn fps(&self) -> u32 {
        self.fps
    }
    fn average_fps(&self) -> u32 {
        self.average_fps
    }
    fn set_font_index(&mut self, font_index: usize) {
        self.font_index = font_index;
    }

    fn get_screen_size(&self) -> (u32, u32) {
        self.bracket_input.new_size
    }
}

/// What is returned by the [`Engine::update`] function
pub enum UpdateEvent {
    /// Save a screenshot. parameter = file path.
    /// The file name must have a .png extension.
    /// This is ignored on WASM platform.
    Capture(String),
    /// end the program
    Exit,
}

/// This is the trait you must implement to update and render your game.
/// See [`App::set_engine`]
pub trait Engine {
    /// Called before the first game loop for one time initialization
    fn init(&mut self, _api: &mut dyn DoryenApi) {}
    /// This is called 60 times per second and is independant of the framerate. Put your world update logic in there.
    /// You can return `Some(UpdateEvent::Exit)` to stop the program
    fn update(&mut self, _api: &mut dyn DoryenApi) -> Option<UpdateEvent> {
        None
    }
    /// This is called before drawing the console on the screen. The framerate depends on the screen frequency, the graphic cards and on whether you activated vsync or not.
    /// The framerate is not reliable so don't update time related stuff in this function.
    /// The screen will display the content of the root console provided by `api.con()`
    fn render(&mut self, api: &mut dyn DoryenApi);
    /// This is called when the size of the game window has changed.
    /// You can override this method if your game display or logic depends on the window size.
    /// You get the new window size with `api.con().get_screen_size()`. See the resize example
    fn resize(&mut self, _api: &mut dyn DoryenApi) {}
}

pub struct AppOptions {
    /// the console width in characters. Default is 80
    pub console_width: u32,
    /// the console height in characters. Default is 45
    pub console_height: u32,
    /// the game window width in pixels
    pub screen_width: u32,
    /// the game window height in pixels
    pub screen_height: u32,
    /// the title of the game window (only in native mode)
    pub window_title: String,
    /// the fonts to use. See [`DoryenApi::set_font_path`]. Default is 'terminal_8x8.png'
    pub font_paths: Vec<String>,
    /// whether framerate are limited by the screen frequency.
    /// On web platforms, this parameter is ignored and vsync is always enabled.
    /// Default is true.
    pub vsync: bool,
    /// Native only. Might not work on every platforms. Default is false.
    pub fullscreen: bool,
    /// Whether the mouse cursor should be visible in the game window. Default is true.
    pub show_cursor: bool,
    /// Whether the game window can be resized. Default is true.
    pub resizable: bool,
    /// Intercepts clicks on the window close button. Can be checked with [`InputApi::close_requested`]
    /// Default is false (clicking on the window close button exits the game)
    pub intercept_close_request: bool,
}

impl Default for AppOptions {
    fn default() -> Self {
        Self {
            console_width: DEFAULT_CONSOLE_WIDTH,
            console_height: DEFAULT_CONSOLE_HEIGHT,
            screen_width: DEFAULT_CONSOLE_WIDTH * 8,
            screen_height: DEFAULT_CONSOLE_HEIGHT * 8,
            window_title: "".to_owned(),
            font_paths: vec!["terminal_8x8.png".to_owned()],
            vsync: true,
            fullscreen: false,
            show_cursor: true,
            resizable: false,
            intercept_close_request: false,
        }
    }
}

/// This is the game application. It handles the creation of the game window, the window events including player input events and runs the main game loop.
pub struct App {
    ctx: BTerm,
    options: AppOptions,
    engine: Option<Box<dyn Engine>>,
}

impl App {
    pub fn new(options: AppOptions) -> Self {
        if options.font_paths.is_empty() {
            panic!("Error : you should provide at least one font in AppOptions::font_paths");
        }
        let mut ctx = BTermBuilder::new()
            .with_dimensions(options.console_width, options.console_height)
            .with_title(options.window_title.clone())
            .with_vsync(options.vsync);
        if options.resizable {
            ctx = ctx.with_resize_scaling(true);
        }
        if cfg!(not(target_arch = "wasm32")) {
            ctx = ctx.with_resource_path("static");
        }
        for font in options.font_paths.iter() {
            let (char_width, char_height) = font_char_size(font);
            let real_font = to_real_path(font);
            println!("loading {}", real_font);
            ctx = ctx.with_font(font, char_width, char_height);
        }
        ctx = ctx.with_simple_console(
            options.console_width,
            options.console_height,
            &options.font_paths[0],
        );
        let ctx = ctx.build().unwrap();
        INPUT.lock().unwrap().activate_event_queue();
        Self {
            ctx,
            options,
            engine: None,
        }
    }
    pub fn set_engine(&mut self, engine: Box<dyn Engine>) {
        self.engine = Some(engine);
    }

    pub fn run(mut self) {
        main_loop(
            self.ctx,
            State::new(
                self.options.console_width,
                self.options.console_height,
                self.engine.take().unwrap(),
                self.options.intercept_close_request,
                (8, 8),
                &self.options.font_paths,
            ),
        )
        .unwrap();
    }
}

struct State {
    engine: Box<dyn Engine>,
    elapsed: f32,
    con: console::Console,
    init: bool,
    cur_font_index: usize,
    char_size: (u32, u32),
    pub fonts: Vec<String>,
    intercept_close_request: bool,
}

impl State {
    fn new(
        width: u32,
        height: u32,
        engine: Box<dyn Engine>,
        intercept_close_request: bool,
        char_size: (u32, u32),
        fonts: &[String],
    ) -> Self {
        Self {
            engine,
            elapsed: 0.0,
            con: console::Console::new(width, height),
            init: false,
            cur_font_index: 0,
            fonts: fonts.iter().map(|s| s.to_owned()).collect(),
            intercept_close_request,
            char_size,
        }
    }
}

fn font_char_size(path: &str) -> (u32, u32) {
    let start = path.rfind('_').unwrap_or(0);
    let end = path.rfind('.').unwrap_or(0);
    let mut char_width = 8;
    let mut char_height = 8;
    if start > 0 && end > 0 {
        let subpath = path[start + 1..end].to_owned();
        let charsize: Vec<&str> = subpath.split('x').collect();
        char_width = charsize[0].parse::<u32>().unwrap();
        char_height = charsize[1].parse::<u32>().unwrap();
    }
    (char_width, char_height)
}

fn to_real_path(path: &str) -> String {
    if cfg!(not(target_arch = "wasm32")) && &path[0..1] != "/" && &path[1..2] != ":" {
        "static/".to_owned() + path
    } else {
        path.to_owned()
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.elapsed += ctx.frame_time_ms / 1000.0;
        let mut api = DoryenApiImpl::new(&mut self.con, ctx.fps as u32, self.char_size);
        api.font_index = self.cur_font_index;
        if !self.init {
            self.init = true;
            self.engine.init(&mut api);
        }
        if self.elapsed > SKIP_TICKS as f32 {
            api.bracket_input.update(self.char_size);
            if api.input().close_requested() && !self.intercept_close_request {
                ctx.quit();
                return;
            }
        }
        while self.elapsed > SKIP_TICKS as f32 {
            let new_api_size = (
                api.bracket_input.new_size.0 / self.char_size.0,
                api.bracket_input.new_size.1 / self.char_size.1,
            );
            if api.con_size != new_api_size {
                api.con_size = new_api_size;
                ctx.set_char_size(new_api_size.0, new_api_size.1);
                self.engine.resize(&mut api);
                api.con().resize(new_api_size.0, new_api_size.1);
            }
            if let Some(event) = self.engine.update(&mut api) {
                match event {
                    // TODO BRACKET
                    UpdateEvent::Capture(_filepath) => (),
                    UpdateEvent::Exit => ctx.quit(),
                }
            }
            api.bracket_input.clear();
            if api.font_index != self.cur_font_index {
                ctx.set_active_font(api.font_index);
                self.cur_font_index = api.font_index;
            }
            self.elapsed -= SKIP_TICKS as f32;
        }
        self.engine.render(&mut api);
        ctx.cls();
        for y in 0..self.con.get_height() {
            for x in 0..self.con.get_width() {
                let fore = self.con.unsafe_get_fore(x as i32, y as i32);
                let back = self.con.unsafe_get_back(x as i32, y as i32);
                let ascii = self.con.unsafe_get_ascii(x as i32, y as i32);
                ctx.set(
                    x as i32,
                    y as i32,
                    RGB::from_u8(fore.0, fore.1, fore.2),
                    RGB::from_u8(back.0, back.1, back.2),
                    ascii as u8,
                );
            }
        }
    }
}
