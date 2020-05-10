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

use std::os::raw::c_void;

pub fn init<F: FnMut(&'static str) -> *const c_void>(loader: F) -> Result<()> {
    context::init(loader)?;
    texture::init();
    shader::init();

    Ok(())
}
