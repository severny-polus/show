pub const VERTEX_SHADER_SOURCE: &str = r#"
	#version 330
	layout (location = 0) in vec2 inPosition;
	layout (location = 1) in vec4 inColor;
	out vec4 color;
	void main() {
		gl_Position = vec4(inPosition, 0, 1);
		color = inColor;
	}
"#;

pub const FRAGMENT_SHADER_SOURCE: &str = r#"
	#version 330
	in vec4 color;
	out vec4 FragColor;
	void main() {
		FragColor = color;
	}
"#;
