use std::mem::size_of;

use glow::{Buffer, HasContext, VertexArray};

use crate::{
    graphics::{util::f32s_to_u8s, DrawMode, Object},
    Context, Point,
};

pub struct Polyline {
    vertex_array: VertexArray,
    buffer: Buffer,
    vertices: usize,
}

impl Object for Polyline {
    type Vertex = Point<f32>;

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
                2 * size_of::<f32>() as i32,
                0,
            );
            context.gl.enable_vertex_attrib_array(0);

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
            .map(|p| {
                [
                    2. * p.x / context.size.x - 1.,
                    2. * p.y / context.size.y - 1.,
                ]
            })
            .flatten()
            .collect();
        self.vertices = data.len() / 2;

        unsafe {
            context
                .gl
                .bind_buffer(glow::ARRAY_BUFFER, Some(self.buffer));
            context
                .gl
                .buffer_data_u8_slice(glow::ARRAY_BUFFER, f32s_to_u8s(&data), mode as u32);
        }
    }

    fn draw(&self, context: &mut Context) {
        unsafe {
            context.gl.use_program(Some(context.solid_program));
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
