#![feature(i128_type)]

extern crate image;
extern crate uni_app;
extern crate webgl;

mod program;

use std::rc::Rc;
use std::cell::RefCell;

use program::{PrimitiveData, Program};

const DORYEN_VS: &'static str = include_str!("doryen_vs.glsl");
const DORYEN_FS: &'static str = include_str!("doryen_fs.glsl");

pub struct Console {
    width: u32,
    height: u32,
    ascii: Vec<u16>,
    back: image::RgbaImage,
}

impl Console {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            ascii: Vec::new(),
            back: image::RgbaImage::new(width, height),
        }
    }
    fn offset(&self, x: i32, y: i32) -> usize {
        x as usize + y as usize * self.width as usize
    }
    fn check_coords(&self, x: i32, y: i32) -> bool {
        (x as u32) < self.width && (y as u32) < self.height
    }
    pub fn ascii(&mut self, x: i32, y: i32, ascii: u16) {
        if self.check_coords(x, y) {
            let off = self.offset(x, y);
            self.ascii[off] = ascii;
        }
    }
}

struct AsyncImage(String, uni_app::fs::File);

struct FrameBufferContext {
    size: (u32, u32),
    texture: webgl::WebGLTexture,
    framebuffer: webgl::WebGLFrameBuffer,
    data: PrimitiveData,
}

pub struct App {
    app: Option<uni_app::App>,
    gl: webgl::WebGLRenderingContext,
    async_images: Vec<Option<AsyncImage>>,
    font: Rc<RefCell<FrameBufferContext>>,
    program: Program,
}

impl App {
    pub fn new(width: u32, height: u32, title: &str, font_width: u32, font_height: u32) -> Self {
        let app = uni_app::App::new(uni_app::AppConfig {
            size: (width, height),
            title: title.to_owned(),
            vsync: true,
            show_cursor: false,
            headless: false,
            fullscreen: false,
        });
        let gl = webgl::WebGLRenderingContext::new(app.canvas());
        gl.viewport(0, 0, width, height);
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
        }
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
            self.program
                .set_texture(webgl::WebGLTexture(self.font.borrow().texture.0));
            self.program.bind(&self.gl);
            self.program
                .render_primitive(&self.gl, &self.font.borrow().data);
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
    gl.tex_parameteri(
        webgl::TextureKind::Texture2d,
        webgl::TextureParameter::TextureMagFilter,
        webgl::TextureMagFilter::Linear as i32,
    );
    gl.tex_parameteri(
        webgl::TextureKind::Texture2d,
        webgl::TextureParameter::TextureMinFilter,
        webgl::TextureMinFilter::Linear as i32,
    );
    let wrap = webgl::TextureWrap::ClampToEdge as i32;
    gl.tex_parameteri(
        webgl::TextureKind::Texture2d,
        webgl::TextureParameter::TextureWrapS,
        wrap,
    );
    gl.tex_parameteri(
        webgl::TextureKind::Texture2d,
        webgl::TextureParameter::TextureWrapT,
        wrap,
    );

    let fb = gl.create_framebuffer();
    gl.bind_framebuffer(webgl::Buffers::Framebuffer, &fb);

    gl.framebuffer_texture2d(
        webgl::Buffers::Framebuffer,
        webgl::Buffers::ColorAttachment0,
        webgl::TextureBindPoint::Texture2d,
        &tex,
        0,
    );

    gl.unbind_framebuffer(webgl::Buffers::Framebuffer);
    gl.unbind_texture();

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
        size,
        texture: tex,
        framebuffer: fb,
        data,
    }
}
