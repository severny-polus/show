use std::{mem::size_of, slice::from_raw_parts};

use glow::{Buffer, Context, HasContext, Program, Shader, Texture, VertexArray};

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

pub(super) unsafe fn create_vertex_array(
    gl: &Context,
) -> Result<(VertexArray, Vec<Buffer>), String> {
    let vao = gl.create_vertex_array()?;
    gl.bind_vertex_array(Some(vao));

    let position = gl.create_buffer()?;
    gl.bind_buffer(glow::ARRAY_BUFFER, Some(position));
    let position_data = [
        -1f32, -1f32, -1f32, 1f32, 1f32, 1f32, -1f32, -1f32, 1f32, -1f32, 1f32, 1f32,
    ];
    gl.buffer_data_u8_slice(
        glow::ARRAY_BUFFER,
        floats_to_bytes(&position_data),
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
        floats_to_bytes(&tex_coord_data),
        glow::STATIC_DRAW,
    );
    gl.enable_vertex_attrib_array(1);
    gl.vertex_attrib_pointer_f32(1, 2, glow::FLOAT, false, 0, 0);

    Ok((vao, vec![position, tex_coord]))
}

pub(super) unsafe fn create_texture(gl: &Context) -> Result<Texture, String> {
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
