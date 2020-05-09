use crate::color::Color;
use crate::vbo::AttributeKind;

use std::convert::From;
use std::vec::Vec;
use vex::{Vector2, Vector3};

pub trait Vertex: Sized {
    fn attrs() -> Vec<(bool, usize, AttributeKind)>;
    fn new() -> Self;
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct BasicVertex {
    pub pos: Vector2,
}

impl BasicVertex {
    pub fn new(x: f32, y: f32) -> BasicVertex {
        BasicVertex {
            pos: Vector2::make(x, y),
        }
    }

    pub fn from_parts(pos: Vector2) -> BasicVertex {
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
            (false, 2, AttributeKind::Float),
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
