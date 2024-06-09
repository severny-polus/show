pub const VERTEX_SHADER_SOURCE: &str = r#"
	#version 330
	layout (location = 0) in vec2 inPosition;
	void main() {
		gl_Position = vec4(inPosition, 0, 1);
	}
"#;

pub const FRAGMENT_SHADER_SOURCE: &str = r#"
	#version 330
	uniform vec4 color;
	out vec4 FragColor;
	void main() {
		FragColor = color;
	}
"#;
