use std::mem::size_of;

use webgl::{AttributeSize, BufferKind, DataType, DrawMode, Primitives, ShaderKind, WebGLBuffer,
            WebGLProgram, WebGLRenderingContext, WebGLShader, WebGLTexture, WebGLUniformLocation,
            WebGLVertexArray, IS_GL_ES};
use uni_app::App;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum PrimitiveType {
    ALL,
    TRIANGLE,
    SPRITE,
    LINE,
}

#[derive(Debug)]
pub struct PrimitiveData {
    pub count: usize,
    pub data_per_primitive: usize,
    pub color_data: Vec<f32>,
    pub pos_data: Vec<f32>,
    pub tex_data: Option<Vec<f32>>,
    pub primitive_type: PrimitiveType,
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
            primitive_type: PrimitiveType::TRIANGLE,
            draw_mode: Primitives::Triangles,
        }
    }
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
    texture: Option<WebGLTexture>,
    texture_uniform_location: Option<WebGLUniformLocation>,
    // private beforeRenderCallback: () => void;
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
        App::print(format!("compiling {} VS\n", vertex_source));
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
        App::print(format!("compiling {} FS\n", fragment_source));
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
        let mut unif = String::new();
        let vertex_pos_location = gl.get_attrib_location(&shader_program, "aVertexPosition");
        let vertex_pos_buffer = match vertex_pos_location {
            None => None,
            Some(_) => {
                unif = unif + "pos ";
                Some(gl.create_buffer())
            }
        };
        let vertex_col_location = gl.get_attrib_location(&shader_program, "aVertexColor");
        let vertex_col_buffer = match vertex_col_location {
            None => None,
            Some(_) => {
                unif = unif + "col ";
                Some(gl.create_buffer())
            }
        };
        let vertex_uv_location = gl.get_attrib_location(&shader_program, "aTextureCoord");
        let vertex_uv_buffer = match vertex_uv_location {
            None => None,
            Some(_) => {
                unif = unif + "tex ";
                Some(gl.create_buffer())
            }
        };
        let texture_uniform_location = gl.get_uniform_location(&shader_program, "uDiffuse");
        let texture = None;
        App::print(format!("{}\n", unif));
        Program {
            program: shader_program,
            vao,
            vertex_pos_location,
            vertex_col_location,
            vertex_uv_location,
            vertex_pos_buffer,
            vertex_col_buffer,
            vertex_uv_buffer,
            texture,
            texture_uniform_location,
        }
    }
    // beforeRender(callback: () => void) {
    //     this.beforeRenderCallback = callback;
    // }
    pub fn get_uniform_location(
        &self,
        gl: &WebGLRenderingContext,
        name: &str,
    ) -> Option<WebGLUniformLocation> {
        gl.get_uniform_location(&self.program, name)
    }
    pub fn set_texture(&mut self, texture: WebGLTexture) {
        self.texture = Some(texture);
    }

    pub fn bind(&self, gl: &WebGLRenderingContext) {
        gl.use_program(&self.program);
    }

    pub fn render_primitive(&mut self, gl: &WebGLRenderingContext, primitive_data: &PrimitiveData) {
        if primitive_data.count == 0 {
            return;
        }
        gl.use_program(&self.program);
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
        if let Some(ref tex) = self.texture {
            if let Some(ref sampler_location) = self.texture_uniform_location {
                gl.active_texture(0);
                gl.bind_texture(&tex);
                gl.uniform_1i(&sampler_location, 0);
            }
        }
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
