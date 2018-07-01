use std::cell::RefCell;
use std::rc::Rc;

use uni_app;
use webgl;

use console::Console;
use font::FontLoader;
use input::{DoryenInput, InputApi};
use program::{set_texture_params, Program};

// shaders
const DORYEN_VS: &'static str = include_str!("doryen_vs.glsl");
const DORYEN_FS: &'static str = include_str!("doryen_fs.glsl");

// fps
const MAX_FRAMESKIP: i32 = 5;
const TICKS_PER_SECOND: f64 = 60.0;
const SKIP_TICKS: f64 = 1.0 / TICKS_PER_SECOND;

/// This is the complete doryen-rs API provided to you by [`App`] in [`Engine::update`] and [`Engine::render`] methods.
pub trait DoryenApi {
    /// return the root console that you can use to draw things on the screen
    fn con(&mut self) -> &mut Console;
    /// return the input API to check user mouse and keyboard input
    fn input(&mut self) -> &mut InputApi;
    /// return the current framerate
    fn fps(&self) -> u32;
    /// return the average framerate since the start of the game
    fn average_fps(&self) -> u32;
    /// replace the current font by a new one.
    /// Put your font in the static/ directory of the project to make this work with both `cargo run` and `cargo web start`.
    /// Example
    /// ```
    /// api.set_font_path("terminal.png");
    /// ```
    /// During development, this references `$PROJECT_ROOT/static/terminal.png`.
    /// Once deployed, `terminal.png` should be in the same directory as your game's executable or `index.html`.
    ///
    /// By default, doryen-rs will assume the font has a 16x16 extended ASCII layout. The character size will be calculated with :
    /// ```
    /// char_width = font_image_width / 16
    /// char_height = font_image_height / 16
    /// ```
    /// If your font has a different layout (that's the case in the unicode example), you have to provide the character size by appending it to the font file name :
    /// ```
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
    fn set_font_path(&mut self, font_path: &str);
}

struct DoryenApiImpl {
    con: Console,
    input: DoryenInput,
    fps: u32,
    average_fps: u32,
    font_path: Option<String>,
}

impl DoryenApi for DoryenApiImpl {
    fn con(&mut self) -> &mut Console {
        &mut self.con
    }
    fn input(&mut self) -> &mut InputApi {
        &mut self.input
    }
    fn fps(&self) -> u32 {
        self.fps
    }
    fn average_fps(&self) -> u32 {
        self.average_fps
    }
    fn set_font_path(&mut self, font_path: &str) {
        self.font_path = Some(font_path.to_owned());
    }
}

impl DoryenApiImpl {
    pub fn clear_font_path(&mut self) {
        self.font_path = None;
    }
}

/// This is the trait you must implement to update and render your game.
/// See [`App::set_engine`]
pub trait Engine {
    /// Called before the first game loop for one time initialization
    fn init(&mut self, _api: &mut DoryenApi) {}
    /// This is called 60 times per second and is independant of the framerate. Put your world update logic in there.
    fn update(&mut self, api: &mut DoryenApi);
    /// This is called before drawing the console on the screen. The framerate depends on the screen frequency, the graphic cards and on whether you activated vsync or not.
    /// The framerate is not reliable so don't update time related stuff in this function.
    fn render(&mut self, api: &mut DoryenApi);
}

pub struct AppOptions {
    /// the console width in characters
    pub console_width: u32,
    /// the console height in characters
    pub console_height: u32,
    /// the game window width in pixels
    pub screen_width: u32,
    /// the game window height in pixels
    pub screen_height: u32,
    /// the title of the game window (only in native mode)
    pub window_title: String,
    /// the font to use. See [`DoryenApi::set_font_path`]
    pub font_path: String,
    /// whether framerate are limited by the screen frequency.
    /// On web platforms, this parameter is ignored and vsync is always enabled.
    pub vsync: bool,
    /// Native only. Might not work on every platforms.
    pub fullscreen: bool,
    /// Whether the mouse cursor should be visible in the game window.
    pub show_cursor: bool,
}

/// This is the game application. It handles the creation of the game window, the window events including player input events and runs the main game loop.
pub struct App {
    app: Option<uni_app::App>,
    gl: webgl::WebGLRenderingContext,
    font: webgl::WebGLTexture,
    font_loader: FontLoader,
    program: Program,
    options: AppOptions,
    fps: FPS,
    api: DoryenApiImpl,
    engine: Option<Box<Engine>>,
    font_width: u32,
    font_height: u32,
    char_width: u32,
    char_height: u32,
}

impl App {
    pub fn new(options: AppOptions) -> Self {
        let con = Console::new(options.console_width, options.console_height);
        let app = uni_app::App::new(uni_app::AppConfig {
            size: (options.screen_width, options.screen_height),
            title: options.window_title.to_owned(),
            vsync: options.vsync,
            show_cursor: options.show_cursor,
            headless: false,
            fullscreen: options.fullscreen,
        });
        let gl = webgl::WebGLRenderingContext::new(app.canvas());
        gl.viewport(0, 0, options.screen_width, options.screen_height);
        gl.enable(webgl::Flag::Blend as i32);
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear(webgl::BufferBit::Color);
        gl.blend_equation(webgl::BlendEquation::FuncAdd);
        gl.blend_func(
            webgl::BlendMode::SrcAlpha,
            webgl::BlendMode::OneMinusSrcAlpha,
        );
        let program = Program::new(&gl, DORYEN_VS, DORYEN_FS);
        let input = DoryenInput::new(
            options.screen_width,
            options.screen_height,
            options.console_width,
            options.console_height,
        );
        let font = create_texture(&gl);
        Self {
            app: Some(app),
            gl,
            font_loader: FontLoader::new(),
            font,
            program,
            options,
            api: DoryenApiImpl {
                input: input,
                con: con,
                fps: 0,
                average_fps: 0,
                font_path: None,
            },
            fps: FPS::new(),
            engine: None,
            font_width: 0,
            font_height: 0,
            char_width: 0,
            char_height: 0,
        }
    }
    pub fn set_engine(&mut self, engine: Box<Engine>) {
        self.engine = Some(engine);
    }

    fn load_font_bytes(&mut self) {
        let img = self.font_loader.img.take().unwrap();
        if self.font_loader.char_width != 0 {
            self.char_width = self.font_loader.char_width;
            self.char_height = self.font_loader.char_height;
        } else {
            self.char_width = img.width() as u32 / 16;
            self.char_height = img.height() as u32 / 16;
        }
        self.font_width = img.width() as u32;
        self.font_height = img.height() as u32;
        uni_app::App::print(format!(
            "font size: {:?} char size: {:?}\n",
            (self.font_width, self.font_height),
            (self.char_width, self.char_height)
        ));
        self.gl.active_texture(0);
        self.gl.bind_texture(&self.font);
        self.gl.tex_image2d(
            webgl::TextureBindPoint::Texture2d, // target
            0,                                  // level
            img.width() as u16,                 // width
            img.height() as u16,                // height
            webgl::PixelFormat::Rgba,           // format
            webgl::PixelType::UnsignedByte,     // type
            &*img,                              // data
        );
    }

    fn handle_input(&mut self, events: Rc<RefCell<Vec<uni_app::AppEvent>>>) {
        self.api.input.on_frame();
        for evt in events.borrow().iter() {
            match evt {
                &uni_app::AppEvent::Resized(size) => {
                    self.gl.viewport(0, 0, size.0, size.1);
                }
                _ => (),
            }
            self.api.input.on_event(&evt);
        }
    }

    pub fn run(mut self) {
        self.api.set_font_path(&self.options.font_path);
        let app = self.app.take().unwrap();
        let mut engine = self.engine.take().unwrap();
        let mut next_tick: f64 = uni_app::now();
        let mut font_loaded = false;
        engine.init(&mut self.api);
        app.run(move |app: &mut uni_app::App| {
            if self.api.font_path.is_some() {
                let font_path = self.api.font_path.clone().unwrap();
                self.api.clear_font_path();
                self.font_loader.load_font(&font_path);
                font_loaded = false;
            }
            if !font_loaded && self.font_loader.load_font_async() {
                self.load_font_bytes();
                self.program.bind(
                    &self.gl,
                    &self.api.con,
                    self.font_width,
                    self.font_height,
                    self.char_width,
                    self.char_height,
                );
                self.program
                    .set_texture(&self.gl, webgl::WebGLTexture(self.font.0));
                font_loaded = true;
            } else {
                self.handle_input(app.events.clone());
                let mut skipped_frames: i32 = -1;
                let time = uni_app::now();
                while time > next_tick && skipped_frames < MAX_FRAMESKIP {
                    engine.update(&mut self.api);
                    next_tick += SKIP_TICKS;
                    skipped_frames += 1;
                }
                if skipped_frames == MAX_FRAMESKIP {
                    next_tick = time + SKIP_TICKS;
                }
                engine.render(&mut self.api);
                self.fps.step();
                self.api.fps = self.fps.fps();
                self.api.average_fps = self.fps.average();
                self.program.render_primitive(&self.gl, &self.api.con);
            }
        });
    }
}

fn create_texture(gl: &webgl::WebGLRenderingContext) -> webgl::WebGLTexture {
    let tex = gl.create_texture();
    gl.active_texture(0);
    gl.bind_texture(&tex);
    set_texture_params(&gl, true);
    tex
}

struct FPS {
    counter: u32,
    start: f64,
    last: f64,
    total_frames: u64,
    fps: u32,
    average: u32,
}

impl FPS {
    pub fn new() -> FPS {
        let now = uni_app::now();
        let fps = FPS {
            counter: 0,
            total_frames: 0,
            start: now,
            last: now,
            fps: 0,
            average: 0,
        };

        fps
    }
    pub fn fps(&self) -> u32 {
        self.fps
    }

    pub fn step(&mut self) {
        self.counter += 1;
        self.total_frames += 1;
        let curr = uni_app::now();
        if curr - self.last > 1.0 {
            self.last = curr;
            self.fps = self.counter;
            self.counter = 0;
            self.average = (self.total_frames as f64 / (self.last - self.start)) as u32;
        }
    }
    pub fn average(&self) -> u32 {
        self.average
    }
}
