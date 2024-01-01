pub mod color;

use crate::basics::{Bounds, Point};
use color::Color;
use core::ffi::c_void;
use glow::{self, Context, HasContext, VertexArray, Shader, Program};

const VERTEX_SHADER_SOURCE: &str = r#"
	#version 330
	in vec2 in_position;
	in vec2 in_color;
	out vec2 texcoord;
	void main() {
		gl_Position = vec4(in_position, 0, 1);
		texcoord = in_texcoord;
	}
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
	#version 330
	precision mediump float;
	uniform vec4 uni_color;
	uniform sampler2D uni_texture;
	in vec2 texcoord;
	out vec4 out_color;
	void main() {
		out_color = vec4(uni_color.rgb, uni_color.a * texture(uni_texture, texcoord).r);
	}
"#;

unsafe fn compile_shader(
    gl: Context,
    shader_type: u32,
    shader_source: &str,
) -> Result<Shader, String> {
    let shader = gl.create_shader(shader_type)?;
    gl.shader_source(shader, shader_source);
    gl.compile_shader(shader);
    if !gl.get_shader_compile_status(shader) {
        Err(gl.get_shader_info_log(shader))
    }
    Ok(shader)
}

unsafe fn link_program(
    gl: G,
    v_shader: G::Shader,
    f_shader: G::Shader,
) -> Result<Program, String> {
    let shaders = [v_shader, f_shader];
    let program = gl.create_program()?;
    for shader in shaders {
        gl.attach_shader(program, shader);
    }
    gl.link_program(program);
    for shader in shaders {
        gl.detach_shader(program, shader);
        gl.delete_shader(shader);
    }
    if !gl.get_program_link_status(program) {
        Err(gl.get_program_info_log(program))
    }
}

pub struct Canvas {
    gl: Context,
    program: Program,
    size: Point,
    dpi: f32,
}

impl Canvas {
    pub fn new(
        size: Point,
        dpi: f32,
        loader_fn: impl FnMut(&str) -> *const c_void,
    ) -> Result<Self, String> {
        unsafe {
            let gl = Context::from_loader_function(loader_fn);
            let program = Self::link_program(
                gl,
                Self::compile_shader(gl, glow::VERTEX_SHADER, VERTEX_SHADER_SOURCE)?,
                Self::compile_shader(gl, glow::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE)?,
            )?;
            gl.use_program(program);
            gl.clear_color(0, 0, 0, 1);
            Ok(Self {
                gl: gl,
                program: program,
                size: size,
                dpi: dpi,
            })
        }
    }

    pub fn set_size(&mut self, size: Point) {
        self.size = size;
        unsafe { self.gl.viewport(0, 0, self.size.x, self.size.y) };
    }

    pub fn clear(&mut self) {
        unsafe { self.gl.clear(glow::COLOR_BUFFER_BIT) };
    }

    pub fn flush(&mut self) {
        unsafe { self.gl.flush() };
    }
}

#[derive(Clone)]
pub enum Shape {
    Fill { color: Color },
    Stroke { color: Color, line_width: f32 },
    Text { string: String, color: Color },
}

pub struct Rectangle {
    pub color: Color,
}

pub struct Stroke {
    pub color: Color,
    pub line_widtn: f32,
}

pub struct Object {
    pub shape: Shape,
    pub color: Color,
}

pub struct Line {
    p1: Point,
    p2: Point,
    line_widtn: f32,
}

impl Canvas {
    fn fill_rectangle(&mut self, bounds: Bounds, color: Color) -> Result<(), String> {
        unsafe {
            let vao = self.gl.create_vertex_array()?;
            self.gl.bind_vertex_array(Some(vao));
            let vbo = self.gl.create_buffer()?;
            Ok(())
        }
    }
}

impl Shape {
    pub fn draw(&self, canvas: &mut Canvas, bounds: Bounds) {
        match self {
            Shape::Fill { color } => canvas.fill_rectangle(bounds, *color),
            Shape::Stroke { color, line_width } => {
                canvas.stroke_rectangle(bounds, *color, *line_width)
            }
            Shape::Text { string, color } => canvas.fill_text(bounds, string, *color),
        }
    }
}

impl Into<VertexArray> for Shape {
    fn into(self) -> VertexArray {
        match
    }
}

pub struct ShapeObject<G: HasContext> {
    vao: G::VertexArray
}

// pub fn line(canvas: &mut Canvas, p1: Point, p2: Point, color: Color, width: u32) {
//     let width = width as f32;
//     let mut path = Path::new();
//     path.move_to(p1.x as f32, p1.y as f32);
//     path.line_to(p2.x as f32, p2.y as f32);
//     canvas.stroke_path(
//         &mut path,
//         &Paint::color(color.femtovg()).with_line_width(width),
//     )
// }
