mod builtin;
mod color;
mod error;
mod shader;
mod context;
mod texture;
mod vbo;

pub use builtin::*;
pub use context::*;
pub use color::*;
pub use error::*;
pub use shader::*;
pub use texture::*;
pub use vbo::*;

pub fn init() -> Result<()> {
    context::init()?;
    texture::init();
    shader::init();

    Ok(())
}
