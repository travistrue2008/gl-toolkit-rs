use crate::color::Color;
use crate::vbo::AttributeKind;

use std::convert::From;
use std::vec::Vec;
use vex::Vector2;

pub trait Vertex {
    fn attrs() -> Vec<(bool, usize, AttributeKind)>;
    fn new() -> Self;
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct BasicVertex {
    pub pos: Vector2,
}

impl BasicVertex {
    pub fn make(x: f32, y: f32) -> BasicVertex {
        BasicVertex {
            pos: Vector2::make(x, y),
        }
    }

    pub fn make_from_parts(pos: Vector2) -> BasicVertex {
        BasicVertex { pos }
    }
}

impl Vertex for BasicVertex {
    fn attrs() -> Vec<(bool, usize, AttributeKind)> {
        vec![(false, 2, AttributeKind::Float)]
    }

    fn new() -> BasicVertex {
        BasicVertex {
            pos: Vector2::new(),
        }
    }
}

impl From<Vector2> for BasicVertex {
    fn from(item: Vector2) -> Self {
        BasicVertex { pos: item }
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct ColorVertex {
    pub pos: Vector2,
    pub color: Color,
}

impl ColorVertex {
    pub fn make(x: f32, y: f32, r: u8, g: u8, b: u8, a: u8) -> ColorVertex {
        ColorVertex {
            pos: Vector2::make(x, y),
            color: Color::make(r, g, b, a),
        }
    }

    pub fn make_from_parts(pos: Vector2, color: Color) -> ColorVertex {
        ColorVertex { pos, color }
    }
}

impl Vertex for ColorVertex {
    fn attrs() -> Vec<(bool, usize, AttributeKind)> {
        vec![
            (false, 2, AttributeKind::Float),
            (true, 4, AttributeKind::UnsignedByte),
        ]
    }

    fn new() -> ColorVertex {
        ColorVertex {
            pos: Vector2::new(),
            color: Color::new(),
        }
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct TextureVertex {
    pub pos: Vector2,
    pub coord: Vector2,
}

impl TextureVertex {
    pub fn make(x: f32, y: f32, u: f32, v: f32) -> TextureVertex {
        TextureVertex {
            pos: Vector2::make(x, y),
            coord: Vector2::make(u, v),
        }
    }

    pub fn make_from_parts(pos: Vector2, coord: Vector2) -> TextureVertex {
        TextureVertex { pos, coord }
    }
}

impl Vertex for TextureVertex {
    fn attrs() -> Vec<(bool, usize, AttributeKind)> {
        vec![
            (false, 2, AttributeKind::Float),
            (false, 2, AttributeKind::Float),
        ]
    }

    fn new() -> TextureVertex {
        TextureVertex {
            pos: Vector2::new(),
            coord: Vector2::new(),
        }
    }
}
