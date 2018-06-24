use std;
use std::rc::Rc;
use std::cell::RefCell;

use webgl;
use uni_app;
use image;

use program::{PrimitiveData, Program, set_texture_params};
use console::Console;

const DORYEN_VS: &'static str = include_str!("doryen_vs.glsl");
const DORYEN_FS: &'static str = include_str!("doryen_fs.glsl");

struct AsyncImage(String, uni_app::fs::File);

struct FrameBufferContext {
    texture: webgl::WebGLTexture,
    data: PrimitiveData,
}

pub struct App {
    app: Option<uni_app::App>,
    gl: webgl::WebGLRenderingContext,
    async_images: Vec<Option<AsyncImage>>,
    font: Rc<RefCell<FrameBufferContext>>,
    program: Program,
    font_width: u32,
    font_height: u32,
    con:Rc<RefCell<Console>>,
}

impl App {
    pub fn new(con_width: u32, con_height: u32, title: &str, font_width: u32, font_height: u32) -> Self {
        let char_width = font_width / 16;
        let char_height = font_height / 16;
        let screen_width=con_width * char_width;
        let screen_height=con_height*char_height;
        let app = uni_app::App::new(uni_app::AppConfig {
            size: (screen_width, screen_height),
            title: title.to_owned(),
            vsync: true,
            show_cursor: false,
            headless: false,
            fullscreen: false,
        });
        let gl = webgl::WebGLRenderingContext::new(app.canvas());
        gl.viewport(0, 0, screen_width, screen_height);
        gl.enable(webgl::Flag::Blend as i32);
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear(webgl::BufferBit::Color);
        gl.blend_equation(webgl::BlendEquation::FuncAdd);
        gl.blend_func(
            webgl::BlendMode::SrcAlpha,
            webgl::BlendMode::OneMinusSrcAlpha,
        );
        let font = Rc::new(RefCell::new(create_framebuffer(
            &gl,
            (font_width, font_height),
        )));
        let program = Program::new(&gl, DORYEN_VS, DORYEN_FS);
        Self {
            app: Some(app),
            gl,
            async_images: Vec::new(),
            font,
            program,
            font_width,
            font_height,
            con:Rc::new(RefCell::new(Console::new(con_width,con_height))),
        }
    }
    pub fn console(&mut self) -> Rc<RefCell<Console>> {
        self.con.clone()
    }
    fn load_font(&mut self, font: &str) {
        match open_file(&font) {
            Ok(mut f) => {
                if f.is_ready() {
                    match f.read_binary() {
                        Ok(buf) => self.load_font_bytes(&buf),
                        Err(e) => panic!("Could not read file {} : {}\n", font, e),
                    }
                } else {
                    uni_app::App::print(format!("loading async file {}\n", font));
                    self.async_images.push(Some(AsyncImage(font.to_owned(), f)));
                }
            }
            Err(e) => panic!("Could not open file {} : {}\n", font, e),
        }
    }

    fn load_font_bytes(&mut self, image_data: &[u8]) {
        let img = &image::load_from_memory(image_data).unwrap().to_rgba();
        self.gl.active_texture(0);
        self.gl.bind_texture(&self.font.borrow().texture);
        self.gl.tex_image2d(
            webgl::TextureBindPoint::Texture2d, // target
            0,                                  // level
            img.width() as u16,                 // width
            img.height() as u16,                // height
            webgl::PixelFormat::Rgba,           // format
            webgl::PixelType::UnsignedByte,     // type
            &*img,                              // data
        );
        self.gl.unbind_texture();
    }

    pub fn run(&mut self, font: &str) {
        self.load_font(font);
        let app = self.app.take().unwrap();
        app.run(move |app: &mut uni_app::App| {
            for _evt in app.events.borrow().iter() {
                // TODO
            }
            self.program
                .set_texture(webgl::WebGLTexture(self.font.borrow().texture.0));
            self.program.bind(&self.gl);
            //self.program.set_uniforms(&self.gl, self.font_width, self.font_height, self.con.clone());
            self.program
                .render_primitive(&self.gl, &self.font.borrow().data, self.font_width, self.font_height, self.con.clone());
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

fn create_framebuffer(gl: &webgl::WebGLRenderingContext, size: (u32, u32)) -> FrameBufferContext {
    let tex = gl.create_texture();

    gl.active_texture(0);
    gl.bind_texture(&tex);
    gl.tex_image2d(
        webgl::TextureBindPoint::Texture2d, // target
        0,                                  // level
        size.0 as u16,                      // width
        size.1 as u16,                      // height
        webgl::PixelFormat::Rgba,           // format
        webgl::PixelType::UnsignedByte,     // type
        &[],                                // data
    );
    set_texture_params(&gl);

    let mut data = PrimitiveData::new();
    data.pos_data.push(-1.0);
    data.pos_data.push(-1.0);
    data.pos_data.push(-1.0);
    data.pos_data.push(1.0);
    data.pos_data.push(1.0);
    data.pos_data.push(1.0);
    data.pos_data.push(1.0);
    data.pos_data.push(-1.0);

    let mut tex_data = Vec::new();
    tex_data.push(0.0);
    tex_data.push(1.0);
    tex_data.push(0.0);
    tex_data.push(0.0);
    tex_data.push(1.0);
    tex_data.push(0.0);
    tex_data.push(1.0);
    tex_data.push(1.0);
    data.tex_data = Some(tex_data);

    data.count = 4;
    data.data_per_primitive = 1;
    data.draw_mode = webgl::Primitives::TriangleFan;

    FrameBufferContext {
        texture: tex,
        data,
    }
}
