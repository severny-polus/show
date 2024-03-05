pub mod color;
pub mod gradient;
pub mod solid;
pub mod util;

use crate::math::{Bounds, Point};
pub use color::Color;
use core::ffi::c_void;
use glow::{self, Buffer, Context, HasContext, Program, Texture, UniformLocation, VertexArray};
use image::{GenericImageView, ImageBuffer, Pixel, Rgba};
use std::mem::size_of;

const VERTEX_SHADER_SOURCE: &str = r#"
	#version 330
	layout (location = 0) in vec2 inPosition;
	layout (location = 1) in vec2 inTexCoord;
	out vec2 texCoord;
	void main() {
		gl_Position = vec4(inPosition, 0, 1);
		texCoord = inTexCoord;
	}
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
	#version 330
	uniform sampler2D tex;
	in vec2 texCoord;
	out vec4 FragColor;
	void main() {
		FragColor = texture(tex, texCoord);
	}
"#;

pub struct Canvas {
    gl: Context,

    size: Point,
    dpi: f32,
    image: ImageBuffer<Rgba<u8>, Vec<u8>>,

    solid_program: Program,
    solid_program_color: UniformLocation,

    gradient_program: Program,
}

impl Canvas {
    pub fn new(
        size: Point,
        dpi: f32,
        loader: impl FnMut(&str) -> *const c_void,
    ) -> Result<Self, String> {
        unsafe {
            let gl = Context::from_loader_function(loader);

            gl.clear_color(0., 0., 0., 1.);
            gl.enable(glow::BLEND);
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA); // for transparency
            gl.enable(glow::MULTISAMPLE); // for antialiasing

            let program = util::create_program(&gl, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE)?;
            gl.use_program(Some(program));
            let (vao, vbos) = util::create_vertex_array(&gl)?;
            let texture = util::create_texture(&gl)?;

            let solid_program = util::create_program(
                &gl,
                solid::VERTEX_SHADER_SOURCE,
                solid::FRAGMENT_SHADER_SOURCE,
            )?;
            let solid_program_color = gl.get_uniform_location(solid_program, "color").unwrap();

            let gradient_program = util::create_program(
                &gl,
                gradient::VERTEX_SHADER_SOURCE,
                gradient::FRAGMENT_SHADER_SOURCE,
            )?;

            let mut canvas = Self {
                gl,

                size: Point::zero(),
                dpi,
                image: ImageBuffer::new(0, 0),

                solid_program,
                solid_program_color,
                gradient_program,
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

    pub fn draw_image(&self) {
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
            self.gl.draw_arrays(glow::TRIANGLES, 0, 6);
        };
    }
}

fn to_screen(value: i32, max: f32) -> f32 {
    2. * value as f32 / max - 1.
}

impl Canvas {
    pub fn draw_lines(&self, points: &[Point], color: Color) {
        let w = self.size.x as f32;
        let h = self.size.y as f32;
        let floats: Vec<f32> = points
            .iter()
            .map(|p| [2. * p.x as f32 / w - 1., 2. * p.y as f32 / h - 1.])
            .flatten()
            .collect();
        unsafe {
            self.gl.use_program(Some(self.solid_program));

            let array = self.gl.create_vertex_array().unwrap();
            self.gl.bind_vertex_array(Some(array));

            let buffer = util::create_buffer(&self.gl, floats.as_slice(), glow::STREAM_DRAW);
            self.gl.vertex_attrib_pointer_f32(
                0,
                2,
                glow::FLOAT,
                false,
                2 * size_of::<f32>() as i32,
                0,
            );
            self.gl.enable_vertex_attrib_array(0);

            self.gl.uniform_4_f32(
                Some(&self.solid_program_color),
                color.r as f32 / 255.,
                color.g as f32 / 255.,
                color.b as f32 / 255.,
                color.a as f32 / 255.,
            );

            self.gl
                .draw_arrays(glow::LINE_STRIP, 0, points.len() as i32);

            self.gl.delete_buffer(buffer);
            self.gl.delete_vertex_array(array);
        }
    }

    pub fn draw_lines_gradient(&self, points: &[(Point, Color)]) {
        let w = self.size.x as f32;
        let h = self.size.y as f32;
        let floats: Vec<f32> = points
            .iter()
            .map(|(p, c)| {
                [
                    2. * p.x as f32 / w - 1.,
                    2. * p.y as f32 / h - 1.,
                    c.r as f32 / 255.,
                    c.g as f32 / 255.,
                    c.b as f32 / 255.,
                    c.a as f32 / 255.,
                ]
            })
            .flatten()
            .collect();
        unsafe {
            self.gl.use_program(Some(self.gradient_program));

            let array = self.gl.create_vertex_array().unwrap();
            self.gl.bind_vertex_array(Some(array));

            let buffer = util::create_buffer(&self.gl, floats.as_slice(), glow::STREAM_DRAW);
            self.gl.vertex_attrib_pointer_f32(
                0,
                2,
                glow::FLOAT,
                false,
                6 * size_of::<f32>() as i32,
                0,
            );
            self.gl.enable_vertex_attrib_array(0);
            self.gl.vertex_attrib_pointer_f32(
                1,
                4,
                glow::FLOAT,
                false,
                6 * size_of::<f32>() as i32,
                2 * size_of::<f32>() as i32,
            );
            self.gl.enable_vertex_attrib_array(1);

            self.gl
                .draw_arrays(glow::LINE_STRIP, 0, points.len() as i32);

            self.gl.delete_buffer(buffer);
            self.gl.delete_vertex_array(array);
        }
    }

    pub fn draw_quadrangle(&self, a: Point, b: Point, c: Point, d: Point, color: Color) {
        let w = self.size.x as f32;
        let h = self.size.y as f32;
        let floats: Vec<f32> = [a, b, d, c]
            .iter()
            .map(|p| [2. * p.x as f32 / w - 1., 2. * p.y as f32 / h - 1.])
            .flatten()
            .collect();
        unsafe {
            self.gl.use_program(Some(self.solid_program));

            let array = self.gl.create_vertex_array().unwrap();
            self.gl.bind_vertex_array(Some(array));

            let buffer = util::create_buffer(&self.gl, floats.as_slice(), glow::STREAM_DRAW);
            self.gl.vertex_attrib_pointer_f32(
                0,
                2,
                glow::FLOAT,
                false,
                2 * size_of::<f32>() as i32,
                0,
            );
            self.gl.enable_vertex_attrib_array(0);

            self.gl.uniform_4_f32(
                Some(&self.solid_program_color),
                color.r as f32 / 255.,
                color.g as f32 / 255.,
                color.b as f32 / 255.,
                color.a as f32 / 255.,
            );

            self.gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);

            self.gl.delete_buffer(buffer);
            self.gl.delete_vertex_array(array);
        }
    }

    pub fn draw_rectangle(&self, bounds: Bounds, color: Color) {
        self.draw_quadrangle(
            bounds.min,
            bounds.min_max(),
            bounds.max,
            bounds.max_min(),
            color,
        )
    }
}
