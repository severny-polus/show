pub mod color;
pub mod gradient;
pub mod util;

use crate::math::{Bounds, Point};
pub use color::Color;
use core::ffi::c_void;
use glow::{self, Buffer, Context, HasContext, Program, Texture, VertexArray};
use image::{GenericImageView, ImageBuffer, Pixel, Rgba};

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
    program: Program,
    vao: VertexArray,
    vbos: Vec<Buffer>,
    texture: Texture,

    size: Point,
    dpi: f32,
    image: ImageBuffer<Rgba<u8>, Vec<u8>>,

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

            let gradient_program = util::create_program(
                &gl,
                gradient::VERTEX_SHADER_SOURCE,
                gradient::FRAGMENT_SHADER_SOURCE,
            )
            .unwrap();

            let mut canvas = Self {
                gl,
                program,
                vao,
                vbos,
                texture,

                size: Point::zero(),
                dpi,
                image: ImageBuffer::new(0, 0),

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

impl Canvas {
    pub fn draw_lines_gradient(&self, points: &[Point], colors: &[Color]) {
        let w = self.size.x as f32;
        let h = self.size.y as f32;
        let points: Vec<f32> = points
            .iter()
            .map(|p| [2. * p.x as f32 / w - 1., 2. * p.y as f32 / h - 1.])
            .flatten()
            .collect();
        let colors: Vec<f32> = colors.iter().map(|c| c.to_vec4()).flatten().collect();
        unsafe {
            self.gl.use_program(Some(self.gradient_program));

            let array = self.gl.create_vertex_array().unwrap();
            self.gl.bind_vertex_array(Some(array));

            let position = util::create_buffer(&self.gl, points.as_slice(), glow::STREAM_DRAW);
            self.gl
                .vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 0, 0);
            self.gl.enable_vertex_attrib_array(0);

            let color = util::create_buffer(&self.gl, colors.as_slice(), glow::STREAM_DRAW);
            self.gl
                .vertex_attrib_pointer_f32(1, 4, glow::FLOAT, false, 0, 0);
            self.gl.enable_vertex_attrib_array(1);

            self.gl
                .draw_arrays(glow::LINE_STRIP, 0, points.len() as i32 / 2);

            self.gl.delete_buffer(position);
            self.gl.delete_buffer(color);
            self.gl.delete_vertex_array(array);
        }
    }
}
