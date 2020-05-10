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
    static ref INDICES: Vec<u16> = vec![0, 1, 2, 0, 2, 3];
    static ref VERTICES: Vec<ColorVertex> = vec![
        ColorVertex::new(32.0,  0.0, 0.0, 255,   0,   0, 255),
        ColorVertex::new( 0.0,  0.0, 0.0,   0, 255,   0, 255),
        ColorVertex::new( 0.0, 32.0, 0.0,   0,   0, 255, 255),
        ColorVertex::new(32.0, 32.0, 0.0,   0,   0,   0, 255),
    ];
}

pub struct SpriteState {
    vbo: VBO,
}

impl State for SpriteState {
    fn new() -> SpriteState {
        SpriteState {
            vbo: VBO::new(
                BufferMode::StaticDraw,
                PrimitiveKind::Triangles,
                &VERTICES,
                Some(&INDICES),
            ),
        }
    }

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

        gl_toolkit::set_viewport(0, 0, 1, 1);
        self.vbo.render();
    }
}
