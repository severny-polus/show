use std::mem::size_of;

use glow::{Buffer, HasContext};

use crate::Point;

use super::{util::f32s_to_u8s, Color, Context};

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum DrawMode {
    Static = glow::STATIC_DRAW, // The vertex data will be uploaded once and drawn many times (e.g. the world).
    Dynamic = glow::DYNAMIC_DRAW, // The vertex data will be created once, changed from time to time, but drawn many times more than that.
    Stream = glow::STREAM_DRAW,   // The vertex data will be uploaded once and drawn once.
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum Shape {
    Points = glow::POINTS,
    Lines = glow::LINES,
    LineLoop = glow::LINE_LOOP,
    LineStrip = glow::LINE_STRIP,
    Triangles = glow::TRIANGLES,
    TriangleStrip = glow::TRIANGLE_STRIP,
    TriangleFan = glow::TRIANGLE_FAN,
    Quads = glow::QUADS,
}

pub trait VertexArray {
    type Vertex;

    fn new(context: &Context) -> Self;

    fn delete(&self, context: &Context);

    fn buffer_data(
        &mut self,
        context: &Context,
        data: impl Iterator<Item = Self::Vertex>,
        draw_mode: DrawMode,
    );

    fn draw(&self, context: &Context, shape: Shape);

    fn draw_stream(
        &mut self,
        context: &Context,
        data: impl Iterator<Item = Self::Vertex>,
        shape: Shape,
    ) {
        self.buffer_data(context, data, DrawMode::Stream);
        self.draw(context, shape);
    }

    fn draw_once(context: &Context, data: impl Iterator<Item = Self::Vertex>, shape: Shape)
    where
        Self: Sized,
    {
        let mut object = Self::new(context);
        object.draw_stream(context, data, shape);
        object.delete(context);
    }
}

fn norm(p: Point<f32>, max: Point<f32>) -> Point<f32> {
    Point::new(2. * (p.x + 0.5) / max.x - 1., 2. * (p.y + 0.5) / max.y - 1.)
}

pub struct PointArray {
    vertex_array: glow::VertexArray,
    buffer: Buffer,
    count: usize,
}

impl VertexArray for PointArray {
    type Vertex = Point<f32>;

    fn new(context: &Context) -> Self {
        unsafe {
            context.gl.use_program(Some(context.solid_program));

            let vertex_array = context.gl.create_vertex_array().unwrap();
            context.gl.bind_vertex_array(Some(vertex_array));

            let buffer = context.gl.create_buffer().unwrap();
            context.gl.bind_buffer(glow::ARRAY_BUFFER, Some(buffer));
            context.gl.vertex_attrib_pointer_f32(
                0,
                2,
                glow::FLOAT,
                false,
                2 * size_of::<f32>() as i32,
                0,
            );
            context.gl.enable_vertex_attrib_array(0);

            Self {
                vertex_array,
                buffer,
                count: 0,
            }
        }
    }

    fn buffer_data(
        &mut self,
        context: &Context,
        data: impl Iterator<Item = Self::Vertex>,
        draw_mode: DrawMode,
    ) {
        let data: Vec<f32> = data
            .map(|p| {
                let p = norm(p, context.size);
                [p.x, p.y]
            })
            .flatten()
            .collect();
        self.count = data.len() / 2;
        unsafe {
            context
                .gl
                .bind_buffer(glow::ARRAY_BUFFER, Some(self.buffer));
            context.gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                f32s_to_u8s(&data),
                draw_mode as u32,
            );
        }
    }

    fn draw(&self, context: &Context, shape: Shape) {
        unsafe {
            context.gl.use_program(Some(context.solid_program));
            context.gl.bind_vertex_array(Some(self.vertex_array));
            context.gl.draw_arrays(shape as u32, 0, self.count as i32);
        }
    }

    fn delete(&self, context: &Context) {
        unsafe {
            context.gl.delete_buffer(self.buffer);
            context.gl.delete_vertex_array(self.vertex_array);
        }
    }
}

pub struct PointColorArray {
    vertex_array: glow::VertexArray,
    buffer: Buffer,
    count: usize,
}

impl VertexArray for PointColorArray {
    type Vertex = (Point<f32>, Color);

    fn new(context: &Context) -> Self {
        unsafe {
            context.gl.use_program(Some(context.gradient_program));

            let vertex_array = context.gl.create_vertex_array().unwrap();
            context.gl.bind_vertex_array(Some(vertex_array));

            let buffer = context.gl.create_buffer().unwrap();
            context.gl.bind_buffer(glow::ARRAY_BUFFER, Some(buffer));
            context.gl.vertex_attrib_pointer_f32(
                0,
                2,
                glow::FLOAT,
                false,
                6 * size_of::<f32>() as i32,
                0,
            );
            context.gl.enable_vertex_attrib_array(0);
            context.gl.vertex_attrib_pointer_f32(
                1,
                4,
                glow::FLOAT,
                false,
                6 * size_of::<f32>() as i32,
                2 * size_of::<f32>() as i32,
            );
            context.gl.enable_vertex_attrib_array(1);

            Self {
                vertex_array,
                buffer,
                count: 0,
            }
        }
    }

    fn buffer_data(
        &mut self,
        context: &Context,
        data: impl Iterator<Item = Self::Vertex>,
        draw_mode: DrawMode,
    ) {
        let data: Vec<f32> = data
            .map(|(p, c)| {
                let p = norm(p, context.size);
                [p.x, p.y, c.r, c.g, c.b, c.a]
            })
            .flatten()
            .collect();
        self.count = data.len() / 6;
        unsafe {
            context
                .gl
                .bind_buffer(glow::ARRAY_BUFFER, Some(self.buffer));
            context.gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                f32s_to_u8s(&data),
                draw_mode as u32,
            );
        }
    }

    fn draw(&self, context: &Context, shape: Shape) {
        unsafe {
            context.gl.use_program(Some(context.gradient_program));
            context.gl.bind_vertex_array(Some(self.vertex_array));
            context.gl.draw_arrays(shape as u32, 0, self.count as i32);
        }
    }

    fn delete(&self, context: &Context) {
        unsafe {
            context.gl.delete_buffer(self.buffer);
            context.gl.delete_vertex_array(self.vertex_array);
        }
    }
}
