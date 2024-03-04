pub mod color;

use crate::math::{Bounds, Point};
pub use color::Color;
use core::ffi::c_void;
use glow::{self, Buffer, Context, HasContext, Program, Shader, Texture, VertexArray};
use image::{GenericImageView, ImageBuffer, Pixel, Rgba};
use std::{mem::size_of, slice::from_raw_parts};

const VERTEX_SHADER_SOURCE: &str = r#"
	#version 330
	layout (location = 0) in vec2 aPosition;
	layout (location = 1) in vec2 aTexCoord;
	out vec2 vTexCoord;
	void main() {
		gl_Position = vec4(aPosition, 0, 1);
		vTexCoord = aTexCoord;
	}
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
	#version 330
	uniform sampler2D tex;
	in vec2 vTexCoord;
	out vec4 FragColor;
	void main() {
		FragColor = texture(tex, vTexCoord);
	}
"#;

unsafe fn compile_shader(
    gl: &Context,
    shader_type: u32,
    shader_source: &str,
) -> Result<Shader, String> {
    let shader = gl.create_shader(shader_type)?;
    gl.shader_source(shader, shader_source);
    gl.compile_shader(shader);
    if !gl.get_shader_compile_status(shader) {
        Err(gl.get_shader_info_log(shader))
    } else {
        Ok(shader)
    }
}

unsafe fn link_program(
    gl: &Context,
    v_shader: Shader,
    f_shader: Shader,
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
    } else {
        Ok(program)
    }
}

unsafe fn create_vertex_array(gl: &Context) -> Result<(VertexArray, Vec<Buffer>), String> {
    let vao = gl.create_vertex_array()?;
    gl.bind_vertex_array(Some(vao));

    let position = gl.create_buffer()?;
    gl.bind_buffer(glow::ARRAY_BUFFER, Some(position));
    let position_data = [
        -1f32, -1f32, -1f32, 1f32, 1f32, 1f32, -1f32, -1f32, 1f32, -1f32, 1f32, 1f32,
    ];
    gl.buffer_data_u8_slice(
        glow::ARRAY_BUFFER,
        from_raw_parts(
            position_data.as_ptr() as *const u8,
            position_data.len() * size_of::<f32>(),
        ),
        glow::STATIC_DRAW,
    );
    gl.enable_vertex_attrib_array(0);
    gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 0, 0);

    let tex_coord = gl.create_buffer()?;
    gl.bind_buffer(glow::ARRAY_BUFFER, Some(tex_coord));
    let tex_coord_data = [
        0f32, 0f32, 0f32, 1f32, 1f32, 1f32, 0f32, 0f32, 1f32, 0f32, 1f32, 1f32,
    ];
    gl.buffer_data_u8_slice(
        glow::ARRAY_BUFFER,
        from_raw_parts(
            tex_coord_data.as_ptr() as *const u8,
            tex_coord_data.len() * size_of::<f32>(),
        ),
        glow::STATIC_DRAW,
    );
    gl.enable_vertex_attrib_array(1);
    gl.vertex_attrib_pointer_f32(1, 2, glow::FLOAT, false, 0, 0);

    Ok((vao, vec![position, tex_coord]))
}

unsafe fn create_texture(gl: &Context) -> Result<Texture, String> {
    let texture = gl.create_texture()?;
    gl.bind_texture(glow::TEXTURE_2D, Some(texture));
    gl.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_MIN_FILTER,
        glow::NEAREST as i32,
    );
    gl.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_MAG_FILTER,
        glow::NEAREST as i32,
    );
    gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
    gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);
    Ok(texture)
}

pub struct Canvas {
    gl: Context,
    program: Program,
    vao: VertexArray,
    vbos: Vec<Buffer>,
    texture: Texture,

    size: Point,
    dpi: f32,
    image: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl Canvas {
    pub fn new(
        size: Point,
        dpi: f32,
        loader: impl FnMut(&str) -> *const c_void,
    ) -> Result<Self, String> {
        unsafe {
            let gl = Context::from_loader_function(loader);
            let program = link_program(
                &gl,
                compile_shader(&gl, glow::VERTEX_SHADER, VERTEX_SHADER_SOURCE)?,
                compile_shader(&gl, glow::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE)?,
            )?;
            gl.use_program(Some(program));
            let (vao, vbos) = create_vertex_array(&gl)?;
            let texture = create_texture(&gl)?;
            gl.clear_color(0., 0., 0., 1.);
            let mut canvas = Self {
                gl,
                program,
                vao,
                vbos,
                texture,

                size: Point::zero(),
                dpi,
                image: ImageBuffer::new(0, 0),
            };
            canvas.set_size(size);
            Ok(canvas)
        }
    }

    pub fn set_size(&mut self, size: Point) {
        self.size = size;
        unsafe {
            self.gl.viewport(0, 0, size.x, size.y);
            // self.gl.tex_storage_2d(glow::TEXTURE_2D, 1, glow::RGBA8, size.x, size.y); // doesn't work
            self.gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                size.x,
                size.y,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                None,
            );
        };
        self.image = ImageBuffer::new(size.x as u32, size.y as u32);
    }

    pub fn clear(&self) {
        unsafe { self.gl.clear(glow::COLOR_BUFFER_BIT) };
    }

    pub fn update(&self) {
        unsafe {
            self.gl.tex_sub_image_2d(
                glow::TEXTURE_2D,
                0,
                0,
                0,
                self.image.width() as i32,
                self.image.height() as i32,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(self.image.as_raw().as_slice()),
            );
        }
    }

    pub fn flush(&self) {
        unsafe {
            self.update();
            self.gl.draw_arrays(glow::TRIANGLES, 0, 6);
        };
    }
}

impl Drop for Canvas {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_texture(self.texture);
            self.gl.delete_vertex_array(self.vao);
            for &vbo in &self.vbos {
                self.gl.delete_buffer(vbo);
            }
            self.gl.delete_program(self.program);
        }
    }
}

impl Canvas {
    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        let x = x as u32;
        let y = y as u32;
        if self.image.in_bounds(x, y) {
            let pixel = self.image.get_pixel_mut(x, y);
            pixel.blend(&Rgba(color.to_array()));
        }
    }

    pub fn fill_rectangle(&mut self, color: Color, bounds: Bounds) {
        for x in bounds.x().range() {
            for y in bounds.y().range() {
                self.set_pixel(x, y, color);
            }
        }
    }

    pub fn stroke_rectangle(&mut self, lw: u32, color: Color, b: Bounds) {
        let lw = lw as i32;
        self.fill_rectangle(
            color,
            Bounds::new(b.min.x, b.min.y, b.min.x + lw, b.max.y - lw),
        );
        self.fill_rectangle(
            color,
            Bounds::new(b.min.x, b.max.y - lw, b.max.x - lw, b.max.y),
        );
        self.fill_rectangle(
            color,
            Bounds::new(b.max.x - lw, b.min.y + lw, b.max.x, b.max.y),
        );
        self.fill_rectangle(
            color,
            Bounds::new(b.min.x + lw, b.min.y, b.max.x, b.min.y + lw),
        );
    }
}
