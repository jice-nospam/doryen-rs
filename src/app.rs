use std::cell::RefCell;
use std::rc::Rc;

use image;
use uni_app;
use webgl;

use console::{Console};
use input::{DoryenInput, InputApi};
use program::{set_texture_params, Program};
use font::FontLoader;

// shaders
const DORYEN_VS: &'static str = include_str!("doryen_vs.glsl");
const DORYEN_FS: &'static str = include_str!("doryen_fs.glsl");

// fps
pub const MAX_FRAMESKIP: i32 = 5;
pub const TICKS_PER_SECOND: f64 = 60.0;
pub const SKIP_TICKS: f64 = 1.0 / TICKS_PER_SECOND;


pub trait DoryenApi {
    fn con(&mut self) -> &mut Console;
    fn input(&mut self) -> &mut InputApi;
    fn fps(&self) -> u32;
    fn average_fps(&self) -> u32;
    fn set_font_path(&mut self, font_path: &str);
}

pub struct DoryenApiImpl {
    con: Console,
    input: DoryenInput,
    fps: u32,
    average_fps: u32,
    font_path:Option<String>,
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
        self.font_path=Some(font_path.to_owned());
    }
}

impl DoryenApiImpl {
    pub fn clear_font_path(&mut self) {
        self.font_path=None;
    }
}

pub trait Engine {
    fn update(&mut self, api: &mut DoryenApi);
    fn render(&mut self, api: &mut DoryenApi);
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
    font: webgl::WebGLTexture,
    font_loader: FontLoader,
    program: Program,
    options: AppOptions,
    fps: FPS,
    api: DoryenApiImpl,
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
            font_loader: FontLoader::new(),
            font,
            program,
            options,
            api: DoryenApiImpl {
                input: input,
                con: con,
                fps: 0,
                average_fps: 0,
                font_path:None,
            },
            fps: FPS::new(),
            engine: None,
            font_width: 0,
            font_height: 0,
        }
    }
    pub fn set_engine(&mut self, engine: Box<Engine>) {
        self.engine = Some(engine);
    }

    fn load_font_bytes(&mut self) {
        let file_data = &self.font_loader.image_data.take().unwrap()[..];
        let img = &mut image::load_from_memory(file_data).unwrap().to_rgba();
        self.process_image(img);
        self.font_width = img.width() as u32;
        self.font_height = img.height() as u32;
        uni_app::App::print(format!("font size: {:?}\n",(self.font_width,self.font_height)));
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

    fn process_image(&mut self, img: &mut image::RgbaImage) {
        let pixel=img.get_pixel(0,0).data;
        let alpha = pixel[3];
        if alpha == 255 {
            let transparent_color=(pixel[0],pixel[1],pixel[2]);
            uni_app::App::print(format!("transparent color: {:?}\n",transparent_color));
            let (width,height)=img.dimensions();
            for y in 0..height {
                for x in 0..width {
                    let p = img.get_pixel_mut(x,y);
                    let pixel = p.data;
                    if (pixel[0],pixel[1],pixel[2]) == transparent_color {
                        p.data[3] = 0;
                    }
                }
            }
        }
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
        let font_path = &self.options.font_path.clone();
        self.font_loader.load_font(font_path);
        let app = self.app.take().unwrap();
        let mut engine = self.engine.take().unwrap();
        let mut next_tick: f64 = uni_app::now();
        let mut font_loaded = false;
        app.run(move |app: &mut uni_app::App| {
            if self.api.font_path.is_some() {
                let font_path = self.api.font_path.clone().unwrap();
                self.api.clear_font_path();
                self.font_loader.load_font(&font_path);
                font_loaded = false;
            }
            if !font_loaded && self.font_loader.load_font_async() {
                self.load_font_bytes();
                self.program
                    .bind(&self.gl, &self.api.con, self.font_width, self.font_height);
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
