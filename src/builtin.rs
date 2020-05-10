use crate::color::Color;
use crate::shader::{Shader, Stage, StageKind};
use crate::vbo::{AttributeKind, Vertex};

use lazy_static::lazy_static;

use std::convert::From;
use std::vec::Vec;
use vex::{Vector2, Vector3};

const SRC_BASIC_VERTEX: &str = r#"
    #version 330 core

    layout (location = 0) in vec3 a_pos;

    void main() {
        gl_Position = vec4(a_pos.x, a_pos.y, a_pos.z, 1.0);
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

    layout (location = 0) in vec3 a_pos;
    layout (location = 1) in vec4 a_color;

    out vec4 v_color;

    void main() {
        v_color = a_color;
        gl_Position = vec4(a_pos.x, a_pos.y, a_pos.z, 1.0);
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

    layout (location = 0) in vec3 a_pos;
    layout (location = 1) in vec2 a_coord;

    out vec2 v_coord;

    void main() {
        v_coord = a_coord;
        gl_Position = vec4(a_pos.x, a_pos.y, a_pos.z, 1.0);
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
    pub static ref SHADER_BASIC: Shader = Shader::new(&vec![
        Stage::new(StageKind::Vertex, SRC_BASIC_VERTEX).unwrap(),
        Stage::new(StageKind::Fragment, SRC_BASIC_FRAGMENT).unwrap(),
    ])
    .unwrap();

    pub static ref SHADER_COLOR: Shader = Shader::new(&vec![
        Stage::new(StageKind::Vertex, SRC_COLOR_VERTEX).unwrap(),
        Stage::new(StageKind::Fragment, SRC_COLOR_FRAGMENT).unwrap(),
    ])
    .unwrap();

    pub static ref SHADER_TEXTURE: Shader = Shader::new(&vec![
        Stage::new(StageKind::Vertex, SRC_TEXTURE_VERTEX).unwrap(),
        Stage::new(StageKind::Fragment, SRC_TEXTURE_FRAGMENT).unwrap(),
    ])
    .unwrap();
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct BasicVertex {
    pub pos: Vector3,
}

impl BasicVertex {
    pub fn new(x: f32, y: f32, z: f32) -> BasicVertex {
        BasicVertex {
            pos: Vector3::make(x, y, z),
        }
    }

    pub fn from_parts(pos: Vector3) -> BasicVertex {
        BasicVertex { pos }
    }
}

impl Vertex for BasicVertex {
    fn attrs() -> Vec<(bool, usize, AttributeKind)> {
        vec![(false, 3, AttributeKind::Float)]
    }

    fn new() -> BasicVertex {
        BasicVertex {
            pos: Vector3::new(),
        }
    }
}

impl From<Vector3> for BasicVertex {
    fn from(item: Vector3) -> Self {
        BasicVertex { pos: item }
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct ColorVertex {
    pub pos: Vector3,
    pub color: Color,
}

impl ColorVertex {
    pub fn new(x: f32, y: f32, z: f32, r: u8, g: u8, b: u8, a: u8) -> ColorVertex {
        ColorVertex {
            pos: Vector3::make(x, y, z),
            color: Color::make(r, g, b, a),
        }
    }

    pub fn from_parts(pos: Vector3, color: Color) -> ColorVertex {
        ColorVertex { pos, color }
    }
}

impl Vertex for ColorVertex {
    fn attrs() -> Vec<(bool, usize, AttributeKind)> {
        vec![
            (false, 3, AttributeKind::Float),
            (true, 4, AttributeKind::UnsignedByte),
        ]
    }

    fn new() -> ColorVertex {
        ColorVertex {
            pos: Vector3::new(),
            color: Color::new(),
        }
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct TextureVertex {
    pub pos: Vector3,
    pub coord: Vector2,
}

impl TextureVertex {
    pub fn new(x: f32, y: f32, z: f32, u: f32, v: f32) -> TextureVertex {
        TextureVertex {
            pos: Vector3::make(x, y, z),
            coord: Vector2::make(u, v),
        }
    }

    pub fn from_parts(pos: Vector3, coord: Vector2) -> TextureVertex {
        TextureVertex { pos, coord }
    }
}

impl Vertex for TextureVertex {
    fn attrs() -> Vec<(bool, usize, AttributeKind)> {
        vec![
            (false, 3, AttributeKind::Float),
            (false, 2, AttributeKind::Float),
        ]
    }

    fn new() -> TextureVertex {
        TextureVertex {
            pos: Vector3::new(),
            coord: Vector2::new(),
        }
    }
}
