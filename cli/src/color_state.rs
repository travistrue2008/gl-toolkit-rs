use crate::state::State;

use lazy_static::lazy_static;

use gl_toolkit::{
    SHADER_COLOR,
    BufferMode,
    PrimitiveKind,
    ColorVertex,
    VBO,
};

lazy_static! {
    static ref VERTICES: Vec<ColorVertex> = vec![
        ColorVertex::new( 1.0,  1.0, 0.0, 255,   0,   0, 255),
        ColorVertex::new(-1.0,  1.0, 0.0,   0, 255,   0, 255),
        ColorVertex::new(-1.0, -1.0, 0.0,   0,   0, 255, 255),
        ColorVertex::new( 1.0, -1.0, 0.0,   0,   0,   0, 255),
    ];
}

pub struct ColorState {
    vbo: VBO,
}

impl ColorState {
    pub fn new() -> ColorState {
        ColorState {
            vbo: VBO::new(
                BufferMode::StaticDraw,
                PrimitiveKind::TriangleFan,
                &VERTICES,
                None,
            ),
        }
    }
}

impl State for ColorState {
    fn key_up(&self) {
    }

    fn key_down(&self) {
    }

    fn resize(&self, width: u32, height: u32) {
    }

    fn update(&self, elapsed_time: f32) {
    }

    fn render(&self) {
        SHADER_COLOR.bind();

        self.vbo.render();
    }
}
