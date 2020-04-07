use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NoMipmaps,
    InvalidTextureDimensions,
    CompileShaderStageFailed(String),
    LinkShaderProgramFailed(String),
}
