use std::{mem::size_of, slice::from_raw_parts};

use glow::{Buffer, Context, HasContext, Program, Shader};

pub(super) unsafe fn create_shader(
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

pub(super) unsafe fn create_program(
    gl: &Context,
    vertex_shader_source: &str,
    fragment_shader_source: &str,
) -> Result<Program, String> {
    let shaders = [
        create_shader(&gl, glow::VERTEX_SHADER, vertex_shader_source)?,
        create_shader(&gl, glow::FRAGMENT_SHADER, fragment_shader_source)?,
    ];
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

pub(super) unsafe fn floats_to_bytes(floats: &[f32]) -> &[u8] {
    from_raw_parts(
        floats.as_ptr() as *const u8,
        floats.len() * size_of::<f32>(),
    )
}

pub(super) unsafe fn create_buffer(gl: &Context, data: &[f32], mode: u32) -> Buffer {
    let buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(glow::ARRAY_BUFFER, Some(buffer));
    gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, floats_to_bytes(data), mode);
    buffer
}
