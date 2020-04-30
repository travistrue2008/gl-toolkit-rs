use crate::shader::{Shader, Stage, StageKind};

use lazy_static::lazy_static;

const SRC_BASIC_VERTEX: &str = r#"
	#version 330 core

	layout (location = 0) in vec2 a_pos;

	void main() {
		gl_Position = vec4(a_pos.x, a_pos.y, 0.0, 1.0);
	}
"#;

const SRC_BASIC_FRAGMENT: &str = r#"
  #version 330 core

  uniform vec4 u_color;

  out vec4 out_color;

  void main() {
	out_color = u_color;
  }
"#;

const SRC_COLOR_VERTEX: &str = r#"
	#version 330 core

	layout (location = 0) in vec2 a_pos;
	layout (location = 1) in vec4 a_color;

	out vec4 v_color;

	void main() {
		v_color = a_color;
		gl_Position = vec4(a_pos.x, a_pos.y, 0.0, 1.0);
	}
"#;

const SRC_COLOR_FRAGMENT: &str = r#"
  #version 330 core

  in vec4 v_color;

  out vec4 out_color;

  void main() {
	out_color = v_color;
  }
"#;

const SRC_TEXTURE_VERTEX: &str = r#"
	#version 330 core

	layout (location = 0) in vec2 a_pos;
	layout (location = 1) in vec2 a_coord;

	out vec2 v_coord;

	void main() {
		v_coord = a_coord;
		gl_Position = vec4(a_pos.x, a_pos.y, 0.0, 1.0);
	}
"#;

const SRC_TEXTURE_FRAGMENT: &str = r#"
  #version 330 core

  uniform sampler2D u_tex;

  in vec2 v_coord;

  out vec4 out_color;

  void main() {
	out_color = texture(u_tex, v_coord);
  }
"#;

lazy_static! {
    pub static ref SHADER_BASIC: Shader = Shader::make(&vec![
        Stage::make(StageKind::Vertex, SRC_BASIC_VERTEX).unwrap(),
        Stage::make(StageKind::Fragment, SRC_BASIC_FRAGMENT).unwrap(),
    ])
    .unwrap();

    pub static ref SHADER_COLOR: Shader = Shader::make(&vec![
        Stage::make(StageKind::Vertex, SRC_COLOR_VERTEX).unwrap(),
        Stage::make(StageKind::Fragment, SRC_COLOR_FRAGMENT).unwrap(),
    ])
    .unwrap();

    pub static ref SHADER_TEXTURE: Shader = Shader::make(&vec![
        Stage::make(StageKind::Vertex, SRC_TEXTURE_VERTEX).unwrap(),
        Stage::make(StageKind::Fragment, SRC_TEXTURE_FRAGMENT).unwrap(),
    ])
    .unwrap();
}
