use gl::types::*;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NoMipmaps,
    AlreadyInitialized,
    InvalidTextureDimensions,
    CompileShaderStageFailed(String),
    LinkShaderProgramFailed(String),
}

#[derive(Debug)]
pub enum GlError {
    None,
    InvalidEnum,
    InvalidValue,
    InvalidOperation,
    InvalidFramebufferOperation,
    OutOfMemory,
    StackUnderflow,
    StackOverflow,
    Unknown(GLenum),
}

impl GlError {
    pub fn new(raw: GLenum) -> GlError {
        match raw {
            gl::NO_ERROR => GlError::None,
            gl::INVALID_ENUM => GlError::InvalidEnum,
            gl::INVALID_VALUE => GlError::InvalidValue,
            gl::INVALID_OPERATION => GlError::InvalidOperation,
            gl::INVALID_FRAMEBUFFER_OPERATION => GlError::InvalidFramebufferOperation,
            gl::OUT_OF_MEMORY => GlError::OutOfMemory,
            gl::STACK_UNDERFLOW => GlError::StackUnderflow,
            gl::STACK_OVERFLOW => GlError::StackOverflow,
            n => GlError::Unknown(n),
        }
    }

    pub fn get_native(&self) -> GLenum {
        match self {
            GlError::None => gl::NO_ERROR,
            GlError::InvalidEnum => gl::INVALID_ENUM,
            GlError::InvalidValue => gl::INVALID_VALUE,
            GlError::InvalidOperation => gl::INVALID_OPERATION,
            GlError::InvalidFramebufferOperation => gl::INVALID_FRAMEBUFFER_OPERATION,
            GlError::OutOfMemory => gl::OUT_OF_MEMORY,
            GlError::StackUnderflow => gl::STACK_UNDERFLOW,
            GlError::StackOverflow => gl::STACK_OVERFLOW,
            GlError::Unknown(n) => *n as GLenum,
        }
    }
}

pub fn get_error() -> GlError {
    let raw = unsafe { gl::GetError() };

    GlError::new(raw)
}
