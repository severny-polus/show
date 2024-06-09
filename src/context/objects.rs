use std::mem::size_of;

use glow::{Buffer, HasContext, VertexArray};

use crate::{Color, Context, Point};

use super::util;

#[repr(u32)]
pub enum DrawMode {
    Static = glow::STATIC_DRAW,
    Dynamic = glow::DYNAMIC_DRAW,
    Stream = glow::STREAM_DRAW,
}

pub trait Object {
    type Vertex;

    fn new(context: &mut Context) -> Self;

    fn store(
        &mut self,
        context: &mut Context,
        data: impl Iterator<Item = Self::Vertex>,
        mode: DrawMode,
    );

    fn draw(&self, context: &mut Context);

    fn delete(&self, context: &mut Context);

    fn draw_stream(&mut self, context: &mut Context, data: impl Iterator<Item = Self::Vertex>) {
        self.store(context, data, DrawMode::Stream);
        self.draw(context);
    }

    fn stream(context: &mut Context, data: impl Iterator<Item = Self::Vertex>)
    where
        Self: Sized,
    {
        let mut object = Self::new(context);
        object.draw_stream(context, data);
        object.delete(context);
    }
}

pub struct PolylineGradient {
    vertex_array: VertexArray,
    buffer: Buffer,
    vertices: usize,
}

impl Object for PolylineGradient {
    type Vertex = (Point<f32>, Color);

    fn new(context: &mut Context) -> Self {
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
                vertices: 0,
            }
        }
    }

    fn store(
        &mut self,
        context: &mut Context,
        data: impl Iterator<Item = Self::Vertex>,
        mode: DrawMode,
    ) {
        let data: Vec<f32> = data
            .map(|(p, c)| {
                [
                    2. * p.x / context.size.x - 1.,
                    2. * p.y / context.size.y - 1.,
                    c.r,
                    c.g,
                    c.b,
                    c.a,
                ]
            })
            .flatten()
            .collect();
        self.vertices = data.len() / 6;

        unsafe {
            context
                .gl
                .bind_buffer(glow::ARRAY_BUFFER, Some(self.buffer));
            context.gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                util::floats_to_bytes(&data),
                mode as u32,
            );
        }
    }

    fn draw(&self, context: &mut Context) {
        unsafe {
            context.gl.use_program(Some(context.gradient_program));
            context.gl.bind_vertex_array(Some(self.vertex_array));
            context
                .gl
                .draw_arrays(glow::LINE_STRIP, 0, self.vertices as i32);
        }
    }

    fn delete(&self, context: &mut Context) {
        unsafe {
            context.gl.delete_buffer(self.buffer);
            context.gl.delete_vertex_array(self.vertex_array);
        }
    }
}
