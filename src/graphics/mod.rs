pub mod color;
pub mod object;
pub mod vertices;

pub use color::Color;
pub use vertices::*;

mod gradient;
mod solid;
mod util;

use crate::math::{Bounds, Point};
use core::ffi::c_void;
use glow::{self, HasContext, Program, UniformLocation};
use std::{iter::zip, mem::size_of};

pub struct Context {
    gl: glow::Context,

    size: Point<f32>,
    dpi: f32,

    solid_program: Program,
    solid_program_color: UniformLocation,

    gradient_program: Program,
}

impl Context {
    pub fn new(
        size: Point,
        dpi: f32,
        loader: impl FnMut(&str) -> *const c_void,
    ) -> Result<Self, String> {
        unsafe {
            let gl = glow::Context::from_loader_function(loader);

            gl.clear_color(0., 0., 0., 1.);
            gl.enable(glow::BLEND);
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA); // for transparency
            gl.enable(glow::MULTISAMPLE); // for antialiasing
            gl.enable(glow::LINE_SMOOTH);

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

            let mut context = Self {
                gl,

                size: Point::new(0., 0.),
                dpi,

                solid_program,
                solid_program_color,
                gradient_program,
            };
            context.set_size(size);

            Ok(context)
        }
    }

    pub fn set_size(&mut self, size: Point) {
        self.size = size.to_f32();
        unsafe { self.gl.viewport(0, 0, size.x, size.y) }
    }

    pub fn clear(&self) {
        unsafe { self.gl.clear(glow::COLOR_BUFFER_BIT) }
    }

    pub fn set_line_width(&self, line_width: f32) {
        unsafe { self.gl.line_width(line_width) }
    }

    pub fn set_color(&self, color: Color) {
        unsafe {
            self.gl.uniform_4_f32(
                Some(&self.solid_program_color),
                color.r,
                color.g,
                color.b,
                color.a,
            )
        }
    }
}

impl Context {
    pub fn draw_points(&self, points: &[Point<f32>], colors: &[Color]) {
        let floats: Vec<f32> = zip(points, colors)
            .map(|(p, c)| {
                [
                    2. * p.x / self.size.x - 1.,
                    2. * p.y / self.size.y - 1.,
                    c.r,
                    c.g,
                    c.b,
                    c.a,
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

            self.gl.draw_arrays(glow::POINTS, 0, points.len() as i32);

            self.gl.delete_buffer(buffer);
            self.gl.delete_vertex_array(array);
        }
    }

    pub fn draw_lines(&self, points: &[Point<f32>], color: Color) {
        let floats: Vec<f32> = points
            .iter()
            .map(|p| [2. * p.x / self.size.x - 1., 2. * p.y / self.size.y - 1.])
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
                color.r,
                color.g,
                color.b,
                color.a,
            );

            self.gl
                .draw_arrays(glow::LINE_STRIP, 0, points.len() as i32);

            self.gl.delete_buffer(buffer);
            self.gl.delete_vertex_array(array);
        }
    }

    pub fn draw_quadrangle(
        &self,
        a: Point<f32>,
        b: Point<f32>,
        c: Point<f32>,
        d: Point<f32>,
        color: Color,
    ) {
        let floats: Vec<f32> = [a, b, d, c]
            .iter()
            .map(|p| [2. * p.x / self.size.x - 1., 2. * p.y / self.size.y - 1.])
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
                color.r,
                color.g,
                color.b,
                color.a,
            );

            self.gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);

            self.gl.delete_buffer(buffer);
            self.gl.delete_vertex_array(array);
        }
    }

    pub fn draw_rectangle(&self, bounds: Bounds, color: Color) {
        self.draw_quadrangle(
            bounds.min.to_f32(),
            bounds.min_max().to_f32(),
            bounds.max.to_f32(),
            bounds.max_min().to_f32(),
            color,
        )
    }
}
