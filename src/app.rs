use std;

use image;
use uni_app;
use webgl;

use console::Console;
use input::{DoryenInput, InputApi};
use program::{set_texture_params, Program};

// shaders
const DORYEN_VS: &'static str = include_str!("doryen_vs.glsl");
const DORYEN_FS: &'static str = include_str!("doryen_fs.glsl");

// fps
pub const MAX_FRAMESKIP: i32 = 5;
pub const TICKS_PER_SECOND: f64 = 60.0;
pub const SKIP_TICKS: f64 = 1.0 / TICKS_PER_SECOND;

struct AsyncImage(String, uni_app::fs::File);

pub trait Engine {
    fn update(&mut self, input: &mut InputApi);
    fn render(&mut self, con: &mut Console);
}

pub struct AppOptions {
    pub console_width: u32,
    pub console_height: u32,
    pub screen_width: u32,
    pub screen_height: u32,
    pub window_title: String,
    pub font_path: String,
    pub vsync: bool,
    pub fullscreen: bool,
    pub show_cursor: bool,
}

pub struct App {
    app: Option<uni_app::App>,
    gl: webgl::WebGLRenderingContext,
    async_images: Vec<Option<AsyncImage>>,
    font: webgl::WebGLTexture,
    program: Program,
    options: AppOptions,
    con: Option<Console>,
    fps: FPS,
    input: Option<DoryenInput>,
    engine: Option<Box<Engine>>,
    font_width: u32,
    font_height: u32,
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
        let input = DoryenInput::new(options.screen_width, options.screen_height);
        let font = create_texture(&gl);
        Self {
            app: Some(app),
            gl,
            async_images: Vec::new(),
            font,
            program,
            options,
            con: Some(con),
            fps: FPS::new(),
            input: Some(input),
            engine: None,
            font_width: 0,
            font_height: 0,
        }
    }
    pub fn set_engine(&mut self, engine: Box<Engine>) {
        self.engine = Some(engine);
    }
    fn load_font(&mut self) {
        match open_file(&self.options.font_path) {
            Ok(mut f) => {
                if f.is_ready() {
                    match f.read_binary() {
                        Ok(buf) => self.load_font_bytes(&buf),
                        Err(e) => {
                            panic!("Could not read file {} : {}\n", self.options.font_path, e)
                        }
                    }
                } else {
                    uni_app::App::print(format!("loading async file {}\n", self.options.font_path));
                    self.async_images
                        .push(Some(AsyncImage(self.options.font_path.to_owned(), f)));
                }
            }
            Err(e) => panic!("Could not open file {} : {}\n", self.options.font_path, e),
        }
    }

    fn load_font_async(&mut self) -> bool {
        if self.async_images.len() == 0 {
            return true;
        }
        let mut to_load = Vec::new();
        let mut idx = 0;
        for ref oasfile in self.async_images.iter() {
            if let &&Some(ref asfile) = oasfile {
                if asfile.1.is_ready() {
                    to_load.push(idx);
                }
                idx += 1;
            }
        }
        for idx in to_load.iter() {
            let mut asfile = self.async_images[*idx].take().unwrap();
            match asfile.1.read_binary() {
                Ok(buf) => {
                    self.load_font_bytes(&buf);
                    return true;
                }
                Err(e) => {
                    uni_app::App::print(format!("could not load async file {} : {}", asfile.0, e))
                }
            }
        }
        self.async_images.retain(|f| f.is_some());
        return false;
    }

    fn load_font_bytes(&mut self, image_data: &[u8]) {
        let img = &image::load_from_memory(image_data).unwrap().to_rgba();
        self.font_width = img.width() as u32;
        self.font_height = img.height() as u32;
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

    pub fn run(mut self) {
        self.load_font();
        let app = self.app.take().unwrap();
        let mut con = self.con.take().unwrap();
        let mut input = self.input.take().unwrap();
        let mut engine = self.engine.take().unwrap();
        let mut next_tick: f64 = uni_app::now();
        let mut font_loaded=false;
        app.run(move |app: &mut uni_app::App| {
            if ! font_loaded && self.load_font_async() {
                self.program.bind(&self.gl, &con, self.font_width, self.font_height);
                self.program.set_texture(&self.gl, webgl::WebGLTexture(self.font.0));
                font_loaded=true;
            } else {
                input.on_frame();
                for evt in app.events.borrow().iter() {
                    match evt {
                        &uni_app::AppEvent::Resized(size) => {
                            self.gl.viewport(0, 0, size.0, size.1);
                        }
                        _ => (),
                    }
                    input.on_event(&evt);
                }
                let mut skipped_frames: i32 = -1;
                let time = uni_app::now();
                while time > next_tick && skipped_frames < MAX_FRAMESKIP {
                    engine.update(&mut input);
                    next_tick += SKIP_TICKS;
                    skipped_frames += 1;
                }
                if skipped_frames == MAX_FRAMESKIP {
                    next_tick = time + SKIP_TICKS;
                }
                engine.render(&mut con);
                self.fps.step();
                self.program.render_primitive(&self.gl, &con);
            }
        });
    }
}

fn open_file(filename: &str) -> Result<uni_app::fs::File, std::io::Error> {
    let ffilename =
        if cfg!(not(target_arch = "wasm32")) && &filename[0..1] != "/" && &filename[1..2] != ":" {
            "static/".to_owned() + filename
        } else {
            filename.to_owned()
        };
    uni_app::fs::FileSystem::open(&ffilename)
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
    last: f64,
    pub fps: u32,
}

impl FPS {
    pub fn new() -> FPS {
        let fps = FPS {
            counter: 0,
            last: uni_app::now(),
            fps: 0,
        };

        fps
    }

    pub fn step(&mut self) {
        self.counter += 1;
        let curr = uni_app::now();
        if curr - self.last > 1.0 {
            self.last = curr;
            self.fps = self.counter;
            self.counter = 0;
            println!("{}", self.fps)
        }
    }
}
