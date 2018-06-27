use std::collections::HashMap;
use std::mem::size_of;
use std::slice;

use uni_app::App;
use webgl::{
    AttributeSize, BufferKind, DataType, DrawMode, PixelFormat, PixelType, Primitives, ShaderKind,
    TextureBindPoint, TextureKind, TextureMagFilter, TextureMinFilter, TextureParameter,
    TextureWrap, WebGLBuffer, WebGLProgram, WebGLRenderingContext, WebGLShader, WebGLTexture,
    WebGLUniformLocation, WebGLVertexArray, IS_GL_ES,
};

use super::{Color, Console};

#[derive(Debug)]
pub struct PrimitiveData {
    pub count: usize,
    pub data_per_primitive: usize,
    pub pos_data: Vec<f32>,
    pub tex_data: Vec<f32>,
    pub draw_mode: Primitives,
}

impl PrimitiveData {
    pub fn new() -> PrimitiveData {
        PrimitiveData {
            count: 0,
            data_per_primitive: 0,
            pos_data: Vec::new(),
            tex_data: Vec::new(),
            draw_mode: Primitives::Triangles,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
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
    vertex_uv_location: Option<u32>,
    vertex_pos_buffer: Option<WebGLBuffer>,
    vertex_uv_buffer: Option<WebGLBuffer>,
    font: Option<WebGLTexture>,
    ascii: WebGLTexture,
    foreground: WebGLTexture,
    background: WebGLTexture,
    uniform_locations: HashMap<DoryenUniforms, Option<WebGLUniformLocation>>,
    data: PrimitiveData,
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

fn compile_shader_wasm_native(
    gl: &WebGLRenderingContext,
    shader_kind: ShaderKind,
    source: &str,
) -> WebGLShader {
    if IS_GL_ES {
        compile_shader(
            &gl,
            shader_kind,
            &("#version 300 es\n".to_string() + source),
        )
    } else {
        compile_shader(&gl, shader_kind, &("#version 150\n".to_string() + source))
    }
}

fn create_program(
    gl: &WebGLRenderingContext,
    vertex_source: &str,
    fragment_source: &str,
) -> WebGLProgram {
    App::print(format!("compiling VS\n"));
    let vert_shader = compile_shader_wasm_native(&gl, ShaderKind::Vertex, vertex_source);
    App::print(format!("compiling FS\n"));
    let frag_shader = compile_shader_wasm_native(&gl, ShaderKind::Fragment, fragment_source);
    App::print(format!("linking\n"));
    let shader_program = gl.create_program();
    gl.attach_shader(&shader_program, &vert_shader);
    gl.attach_shader(&shader_program, &frag_shader);
    gl.link_program(&shader_program);
    shader_program
}

impl Program {
    pub fn new(gl: &WebGLRenderingContext, vertex_source: &str, fragment_source: &str) -> Program {
        let data = create_primitive();
        let shader_program = create_program(gl, vertex_source, fragment_source);
        let vao = gl.create_vertex_array();
        let vertex_pos_location = gl.get_attrib_location(&shader_program, "aVertexPosition");
        let vertex_pos_buffer = vertex_pos_location.and(Some(gl.create_buffer()));
        let vertex_uv_location = gl.get_attrib_location(&shader_program, "aTextureCoord");
        let vertex_uv_buffer = vertex_uv_location.and(Some(gl.create_buffer()));
        let mut uniform_locations = HashMap::new();
        for uniform in [
            (DoryenUniforms::Font, "uFont"),
            (DoryenUniforms::Ascii, "uAscii"),
            (DoryenUniforms::Background, "uBack"),
            (DoryenUniforms::Foreground, "uFront"),
            (DoryenUniforms::FontCharsPerLine, "uFontCharsPerLine"),
            (DoryenUniforms::FontCoef, "uFontCoef"),
            (DoryenUniforms::TermCoef, "uTermCoef"),
            (DoryenUniforms::TermSize, "uTermSize"),
        ].iter()
        {
            uniform_locations.insert(
                uniform.0,
                gl.get_uniform_location(&shader_program, uniform.1),
            );
        }

        Program {
            program: shader_program,
            vao,
            vertex_pos_location,
            vertex_uv_location,
            vertex_pos_buffer,
            vertex_uv_buffer,
            font: None,
            ascii: gl.create_texture(),
            foreground: gl.create_texture(),
            background: gl.create_texture(),
            uniform_locations,
            data,
        }
    }

    pub fn set_texture(&mut self, gl: &WebGLRenderingContext, font: WebGLTexture) {
        if let Some(&Some(ref sampler_location)) = self.uniform_locations.get(&DoryenUniforms::Font)
        {
            gl.active_texture(0);
            gl.bind_texture(&font);
            gl.uniform_1i(sampler_location, 0);
        }
        self.font = Some(font);
    }

    pub fn bind(
        &self,
        gl: &WebGLRenderingContext,
        con: &Console,
        font_width: u32,
        font_height: u32,
    ) {
        gl.use_program(&self.program);
        gl.bind_vertex_array(&self.vao);
        if let Some(ref buf) = self.vertex_pos_buffer {
            if let Some(ref loc) = self.vertex_pos_location {
                set_buffer_data(
                    gl,
                    &buf,
                    Some(self.data.pos_data.clone()),
                    *loc,
                    AttributeSize::Two,
                );
            }
        }
        if let Some(ref buf) = self.vertex_uv_buffer {
            if let Some(ref loc) = self.vertex_uv_location {
                set_buffer_data(
                    gl,
                    &buf,
                    Some(self.data.tex_data.clone()),
                    *loc,
                    AttributeSize::Two,
                );
            }
        }
        let pot_width = con.get_pot_width();
        let pot_height = con.get_pot_height();
        let con_width = con.get_width();
        let con_height = con.get_height();
        let pot_font_width = get_pot_value(font_width);
        let pot_font_height = get_pot_value(font_height);
        if let Some(&Some(ref location)) = self.uniform_locations.get(&DoryenUniforms::TermSize) {
            gl.uniform_2f(location, (con_width as f32, con_height as f32));
        }
        if let Some(&Some(ref location)) = self.uniform_locations.get(&DoryenUniforms::TermCoef) {
            gl.uniform_2f(
                location,
                (1.0 / (pot_width as f32), 1.0 / (pot_height as f32)),
            );
        }
        if let Some(&Some(ref location)) = self
            .uniform_locations
            .get(&DoryenUniforms::FontCharsPerLine)
        {
            gl.uniform_1f(location, 16.0);
        }
        if let Some(&Some(ref location)) = self.uniform_locations.get(&DoryenUniforms::FontCoef) {
            gl.uniform_2f(
                location,
                (
                    1.0 / 16.0,
                    1.0 / 16.0,
                ),
            );
        }
    }

    pub fn render_primitive(&mut self, gl: &WebGLRenderingContext, con: &Console) {
        self.set_uniforms(gl, con);

        gl.draw_arrays(
            self.data.draw_mode,
            self.data.count * self.data.data_per_primitive,
        );
    }

    fn update_uniform_texture(
        &mut self,
        gl: &WebGLRenderingContext,
        uniform: DoryenUniforms,
        tex_num: u32,
        tex: &WebGLTexture,
        data: &[u8],
        pot_width: u32,
        pot_height: u32,
    ) {
        if let Some(&Some(ref location)) = self.uniform_locations.get(&uniform) {
            gl.active_texture(tex_num);
            gl.bind_texture(tex);
            gl.tex_image2d(
                TextureBindPoint::Texture2d, // target
                0,                           // level
                pot_width as u16,            // width
                pot_height as u16,           // height
                PixelFormat::Rgba,           // format
                PixelType::UnsignedByte,     // type
                data,                        // data
            );
            set_texture_params(&gl, true);
            gl.uniform_1i(location, tex_num as i32);
        }
    }

    pub fn set_uniforms(&mut self, gl: &WebGLRenderingContext, con: &Console) {
        let pot_width = con.get_pot_width();
        let pot_height = con.get_pot_height();
        let ascii_tex = WebGLTexture(self.ascii.0);
        self.update_uniform_texture(
            gl,
            DoryenUniforms::Ascii,
            1,
            &ascii_tex,
            u32_to_u8(&con.borrow_ascii()[..]),
            pot_width,
            pot_height,
        );
        let fore_tex = WebGLTexture(self.foreground.0);
        self.update_uniform_texture(
            gl,
            DoryenUniforms::Foreground,
            2,
            &fore_tex,
            color_to_u8(&con.borrow_foreground()[..]),
            pot_width,
            pot_height,
        );
        let back_tex = WebGLTexture(self.background.0);
        self.update_uniform_texture(
            gl,
            DoryenUniforms::Background,
            3,
            &back_tex,
            color_to_u8(&con.borrow_background()[..]),
            pot_width,
            pot_height,
        );
    }
}

fn set_buffer_data(
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

fn u32_to_u8(v: &[u32]) -> &[u8] {
    unsafe { slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * size_of::<u32>()) }
}

fn color_to_u8(v: &[Color]) -> &[u8] {
    unsafe { slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * size_of::<Color>()) }
}

pub fn set_texture_params(gl: &WebGLRenderingContext, nearest: bool) {
    gl.tex_parameteri(
        TextureKind::Texture2d,
        TextureParameter::TextureMagFilter,
        if nearest {
            TextureMagFilter::Nearest
        } else {
            TextureMagFilter::Linear
        } as i32,
    );
    gl.tex_parameteri(
        TextureKind::Texture2d,
        TextureParameter::TextureMinFilter,
        if nearest {
            TextureMinFilter::Nearest
        } else {
            TextureMinFilter::Linear
        } as i32,
    );
    let wrap = TextureWrap::ClampToEdge as i32;
    gl.tex_parameteri(TextureKind::Texture2d, TextureParameter::TextureWrapS, wrap);
    gl.tex_parameteri(TextureKind::Texture2d, TextureParameter::TextureWrapT, wrap);
}

fn get_pot_value(value: u32) -> u32 {
    let mut pot_value = 1;
    while pot_value < value {
        pot_value *= 2;
    }
    pot_value
}

fn create_primitive() -> PrimitiveData {
    let mut data = PrimitiveData::new();
    data.pos_data.push(-1.0);
    data.pos_data.push(-1.0);
    data.pos_data.push(-1.0);
    data.pos_data.push(1.0);
    data.pos_data.push(1.0);
    data.pos_data.push(1.0);
    data.pos_data.push(1.0);
    data.pos_data.push(-1.0);

    data.tex_data.push(0.0);
    data.tex_data.push(1.0);
    data.tex_data.push(0.0);
    data.tex_data.push(0.0);
    data.tex_data.push(1.0);
    data.tex_data.push(0.0);
    data.tex_data.push(1.0);
    data.tex_data.push(1.0);

    data.count = 4;
    data.data_per_primitive = 1;
    data.draw_mode = Primitives::TriangleFan;

    data
}
