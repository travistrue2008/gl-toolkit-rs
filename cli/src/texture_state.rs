use crate::state::State;

use lazy_static::lazy_static;

use gl_toolkit::{
    SHADER_TEXTURE,
    WrapCoord,
    ClampMode,
    BufferMode,
    PrimitiveKind,
    TextureVertex,
    Texture,
    VBO,
};

lazy_static! {
    static ref TEXTURE_DATA: Vec<u8> = vec![
        255, 255, 255, 255,
          0,   0,   0, 255,
          0,   0,   0, 255,
        255, 255, 255, 255,
    ];

    static ref VERTICES: Vec<TextureVertex> = vec![
        TextureVertex::new( 1.0,  1.0, 0.0, 8.0, 0.0),
        TextureVertex::new(-1.0,  1.0, 0.0, 0.0, 0.0),
        TextureVertex::new(-1.0, -1.0, 0.0, 0.0, 8.0),
        TextureVertex::new( 1.0, -1.0, 0.0, 8.0, 8.0),
    ];
}

pub struct TextureState {
    vbo: VBO,
    texture: Texture,
}

impl TextureState {
    pub fn new() -> TextureState {
        let mut result = TextureState {
            texture: Texture::make(&TEXTURE_DATA, 2, 2, false).unwrap(),
            vbo: VBO::new(
                BufferMode::StaticDraw,
                PrimitiveKind::TriangleFan,
                &VERTICES,
                None,
            ),
        };

        result.texture.set_clamp(WrapCoord::S, ClampMode::Repeat);
        result.texture.set_clamp(WrapCoord::T, ClampMode::Repeat);
        result
    }
}

impl State for TextureState {
    fn key_up(&self) {
    }

    fn key_down(&self) {
    }

    fn resize(&self, width: u32, height: u32) {
    }

    fn update(&self, elapsed_time: f32) {
    }

    fn render(&self) {
        SHADER_TEXTURE.bind();
        SHADER_TEXTURE.upload_texture("u_tex", &self.texture, 0);

        self.texture.bind(0);
        self.vbo.render();
    }
}
