use std::cell::RefCell;
use std::collections::HashMap;
use std::mem::size_of;
use std::rc::Rc;
use std::slice;

use uni_app::App;
use webgl::{AttributeSize, BufferKind, DataType, DrawMode, PixelFormat, PixelType, Primitives,
            ShaderKind, TextureBindPoint, WebGLBuffer, WebGLProgram, WebGLRenderingContext,TextureKind,TextureParameter,TextureMinFilter,TextureWrap,TextureMagFilter,
            WebGLShader, WebGLTexture, WebGLUniformLocation, WebGLVertexArray, IS_GL_ES};

use super::{Console,Color};

#[derive(Debug)]
pub struct PrimitiveData {
    pub count: usize,
    pub data_per_primitive: usize,
    pub color_data: Vec<f32>,
    pub pos_data: Vec<f32>,
    pub tex_data: Option<Vec<f32>>,
    pub draw_mode: Primitives,
}

impl PrimitiveData {
    pub fn new() -> PrimitiveData {
        PrimitiveData {
            count: 0,
            data_per_primitive: 0,
            color_data: Vec::new(),
            pos_data: Vec::new(),
            tex_data: None,
            draw_mode: Primitives::Triangles,
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
enum DoryenUniforms {
    Font,
    Ascii,
    Foreground,
    Background,
    FontCharsPerLine,
    FontCoef,
    TermSize,
    TermCoef,
}

pub struct Program {
    program: WebGLProgram,
    vao: WebGLVertexArray,
    vertex_pos_location: Option<u32>,
    vertex_col_location: Option<u32>,
    vertex_uv_location: Option<u32>,
    vertex_pos_buffer: Option<WebGLBuffer>,
    vertex_col_buffer: Option<WebGLBuffer>,
    vertex_uv_buffer: Option<WebGLBuffer>,
    font: Option<WebGLTexture>,
    ascii: WebGLTexture,
    foreground: WebGLTexture,
    background: WebGLTexture,
    uniform_locations: HashMap<DoryenUniforms, Option<WebGLUniformLocation>>,
}

trait IntoBytes {
    fn into_bytes(self) -> Vec<u8>;
}

impl<T> IntoBytes for Vec<T> {
    fn into_bytes(self) -> Vec<u8> {
        let len = size_of::<T>() * self.len();
        unsafe {
            let slice = self.into_boxed_slice();
            Vec::<u8>::from_raw_parts(Box::into_raw(slice) as _, len, len)
        }
    }
}

fn compile_shader(
    gl: &WebGLRenderingContext,
    shader_kind: ShaderKind,
    source: &str,
) -> WebGLShader {
    let shader = gl.create_shader(shader_kind);
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);
    shader
}

impl Program {
    pub fn new(gl: &WebGLRenderingContext, vertex_source: &str, fragment_source: &str) -> Program {
        // Create a vertex shader object
        App::print(format!("compiling VS\n"));
        let vert_shader = if IS_GL_ES {
            compile_shader(
                &gl,
                ShaderKind::Vertex,
                &("#version 300 es\n".to_string() + vertex_source),
            )
        } else {
            compile_shader(
                &gl,
                ShaderKind::Vertex,
                &("#version 150\n".to_string() + vertex_source),
            )
        };
        App::print(format!("compiling FS\n"));
        let frag_shader = if IS_GL_ES {
            compile_shader(
                &gl,
                ShaderKind::Fragment,
                &("#version 300 es\nprecision highp float;\n".to_string() + fragment_source),
            )
        } else {
            compile_shader(
                &gl,
                ShaderKind::Fragment,
                &("#version 150\n".to_string() + fragment_source),
            )
        };
        App::print(format!("linking\n"));
        let shader_program = gl.create_program();
        gl.attach_shader(&shader_program, &vert_shader);
        gl.attach_shader(&shader_program, &frag_shader);
        gl.link_program(&shader_program);
        let vao = gl.create_vertex_array();
        let vertex_pos_location = gl.get_attrib_location(&shader_program, "aVertexPosition");
        let vertex_pos_buffer = match vertex_pos_location {
            None => None,
            Some(_) => {
                Some(gl.create_buffer())
            }
        };
        let vertex_col_location = gl.get_attrib_location(&shader_program, "aVertexColor");
        let vertex_col_buffer = match vertex_col_location {
            None => None,
            Some(_) => {
                Some(gl.create_buffer())
            }
        };
        let vertex_uv_location = gl.get_attrib_location(&shader_program, "aTextureCoord");
        let vertex_uv_buffer = match vertex_uv_location {
            None => None,
            Some(_) => {
                Some(gl.create_buffer())
            }
        };
        let mut uniform_locations = HashMap::new();
        uniform_locations.insert(
            DoryenUniforms::Font,
            gl.get_uniform_location(&shader_program, "uFont"),
        );
        uniform_locations.insert(
            DoryenUniforms::Ascii,
            gl.get_uniform_location(&shader_program, "uAscii"),
        );
        uniform_locations.insert(
            DoryenUniforms::Background,
            gl.get_uniform_location(&shader_program, "uBack"),
        );
        uniform_locations.insert(
            DoryenUniforms::Foreground,
            gl.get_uniform_location(&shader_program, "uFront"),
        );
        uniform_locations.insert(
            DoryenUniforms::FontCharsPerLine,
            gl.get_uniform_location(&shader_program, "uFontCharsPerLine"),
        );
        uniform_locations.insert(
            DoryenUniforms::FontCoef,
            gl.get_uniform_location(&shader_program, "uFontCoef"),
        );
        uniform_locations.insert(
            DoryenUniforms::TermCoef,
            gl.get_uniform_location(&shader_program, "uTermCoef"),
        );
        uniform_locations.insert(
            DoryenUniforms::TermSize,
            gl.get_uniform_location(&shader_program, "uTermSize"),
        );

        Program {
            program: shader_program,
            vao,
            vertex_pos_location,
            vertex_col_location,
            vertex_uv_location,
            vertex_pos_buffer,
            vertex_col_buffer,
            vertex_uv_buffer,
            font: None,
            ascii: gl.create_texture(),
            foreground: gl.create_texture(),
            background: gl.create_texture(),
            uniform_locations,
        }
    }

    pub fn set_texture(&mut self, font: WebGLTexture) {
        self.font = Some(font);
    }

    pub fn bind(&self, gl: &WebGLRenderingContext) {
        gl.use_program(&self.program);
    }

    pub fn render_primitive(&mut self, gl: &WebGLRenderingContext, primitive_data: &PrimitiveData, font_width: u32,
        font_height: u32,
        rccon: Rc<RefCell<Console>>,) {
        if primitive_data.count == 0 {
            return;
        }
        gl.bind_vertex_array(&self.vao);
        if let Some(ref buf) = self.vertex_pos_buffer {
            if let Some(ref loc) = self.vertex_pos_location {
                self.set_buffer_data(
                    gl,
                    &buf,
                    Some(primitive_data.pos_data.clone()),
                    *loc,
                    AttributeSize::Two,
                );
            }
        }
        if let Some(ref buf) = self.vertex_col_buffer {
            if let Some(ref loc) = self.vertex_col_location {
                self.set_buffer_data(
                    gl,
                    &buf,
                    Some(primitive_data.color_data.clone()),
                    *loc,
                    AttributeSize::Three,
                );
            }
        }
        if let Some(ref buf) = self.vertex_uv_buffer {
            if let Some(ref loc) = self.vertex_uv_location {
                if let Some(ref tex_data) = primitive_data.tex_data {
                    self.set_buffer_data(
                        gl,
                        &buf,
                        Some(tex_data.clone()),
                        *loc,
                        AttributeSize::Two,
                    );
                }
            }
        }
        if let Some(ref tex) = self.font {
            if let Some(&Some(ref sampler_location)) =
                self.uniform_locations.get(&DoryenUniforms::Font)
            {
                gl.active_texture(0);
                gl.bind_texture(&tex);
                gl.uniform_1i(sampler_location, 0);
            }
        }
        self.set_uniforms(gl, font_width,font_height,rccon);
        // if (this.beforeRenderCallback) {
        //     this.beforeRenderCallback();
        // }
        gl.draw_arrays(
            primitive_data.draw_mode,
            primitive_data.count * primitive_data.data_per_primitive,
        );
        gl.unbind_buffer(BufferKind::Array);
        gl.unbind_texture();
        gl.unbind_vertex_array(&self.vao);
    }

    pub fn set_uniforms(
        &mut self,
        gl: &WebGLRenderingContext,
        font_width: u32,
        font_height: u32,
        rccon: Rc<RefCell<Console>>,
    ) {
        let con = rccon.borrow();
        let con_width=con.get_width();
        let con_height=con.get_height();
        let pot_width=con.get_pot_width();
        let pot_height=con.get_pot_height();
        let pot_font_width = get_pot_value(font_width);
        let pot_font_height = get_pot_value(font_height);
        if let Some(&Some(ref location)) = self.uniform_locations.get(&DoryenUniforms::TermSize) {
            gl.uniform_2f(location, (con_width as f32, con_height as f32));
        }
        if let Some(&Some(ref location)) = self.uniform_locations.get(&DoryenUniforms::TermCoef) {
            gl.uniform_2f(location, (1.0/(pot_width as f32), 1.0/(pot_height as f32)));
        }
        if let Some(&Some(ref location)) = self.uniform_locations.get(&DoryenUniforms::FontCharsPerLine) {
            gl.uniform_1f(location, 16.0);
        }
        if let Some(&Some(ref location)) = self.uniform_locations.get(&DoryenUniforms::FontCoef) {
            gl.uniform_2f(
                location,
                (
                    (font_width as f32) / (pot_font_width as f32 * 16.0),
                    (font_height as f32) / (pot_font_height as f32 * 16.0),
                ),
            );
        }
        if let Some(&Some(ref location)) = self.uniform_locations.get(&DoryenUniforms::Ascii) {
            gl.active_texture(1);
            gl.bind_texture(&self.ascii);
            gl.tex_image2d(
                TextureBindPoint::Texture2d, // target
                0,                           // level
                pot_width as u16,        // width
                pot_height as u16,       // height
                PixelFormat::Rgba,           // format
                PixelType::UnsignedByte,     // type
                u32_to_u8(&con.borrow_ascii()[..]),          // data
            );
            set_texture_params(&gl);
            gl.uniform_1i(location, 1);
        }
        if let Some(&Some(ref location)) = self.uniform_locations.get(&DoryenUniforms::Foreground) {
            gl.active_texture(2);
            gl.bind_texture(&self.foreground);
            gl.tex_image2d(
                TextureBindPoint::Texture2d, // target
                0,                           // level
                pot_width as u16,        // width
                pot_height as u16,       // height
                PixelFormat::Rgba,           // format
                PixelType::UnsignedByte,     // type
                color_to_u8(&con.borrow_foreground()[..]),          // data
            );
            set_texture_params(&gl);
            gl.uniform_1i(location, 2);
        }
        if let Some(&Some(ref location)) = self.uniform_locations.get(&DoryenUniforms::Background) {
            gl.active_texture(3);
            gl.bind_texture(&self.background);
            gl.tex_image2d(
                TextureBindPoint::Texture2d, // target
                0,                           // level
                pot_width as u16,        // width
                pot_height as u16,       // height
                PixelFormat::Rgba,           // format
                PixelType::UnsignedByte,     // type
                color_to_u8(&con.borrow_background()[..]),          // data
            );
            set_texture_params(&gl);
            gl.uniform_1i(location, 3);
        }
    }

    fn set_buffer_data(
        &self,
        gl: &WebGLRenderingContext,
        buffer: &WebGLBuffer,
        data: Option<Vec<f32>>,
        attribute_location: u32,
        count_per_vertex: AttributeSize,
    ) {
        gl.bind_buffer(BufferKind::Array, buffer);
        gl.enable_vertex_attrib_array(attribute_location);
        if let Some(v) = data {
            gl.buffer_data(BufferKind::Array, &v.into_bytes(), DrawMode::Stream);
        }
        gl.vertex_attrib_pointer(
            attribute_location,
            count_per_vertex,
            DataType::Float,
            false,
            0,
            0,
        );
    }
}

fn u32_to_u8(v: &[u32]) -> &[u8] {
    unsafe {
        slice::from_raw_parts(
            v.as_ptr() as *const u8,
            v.len() * size_of::<u32>(),
        )
    }
}

fn color_to_u8(v: &[Color]) -> &[u8] {
    unsafe {
        slice::from_raw_parts(
            v.as_ptr() as *const u8,
            v.len() * size_of::<Color>(),
        )
    }
}

pub fn set_texture_params(gl: &WebGLRenderingContext) {
    gl.tex_parameteri(
        TextureKind::Texture2d,
        TextureParameter::TextureMagFilter,
        TextureMagFilter::Nearest as i32,
    );
    gl.tex_parameteri(
        TextureKind::Texture2d,
        TextureParameter::TextureMinFilter,
        TextureMinFilter::Nearest as i32,
    );
    let wrap = TextureWrap::ClampToEdge as i32;
    gl.tex_parameteri(
        TextureKind::Texture2d,
        TextureParameter::TextureWrapS,
        wrap,
    );
    gl.tex_parameteri(
        TextureKind::Texture2d,
        TextureParameter::TextureWrapT,
        wrap,
    );
}

fn get_pot_value(value: u32) -> u32 {
    let mut pot_value=1;
    while pot_value < value {
        pot_value *= 2;
    }
    pot_value
}