use crate::error::{Result, Error};

use gl::types::*;
use std::os::raw::c_void;
use std::vec::Vec;

#[derive(Copy, Clone)]
pub enum Coord {
    S,
    T,
}

impl Coord {
    pub fn get_native(&self) -> GLenum {
        match self {
            Coord::S => gl::TEXTURE_WRAP_S,
            Coord::T => gl::TEXTURE_WRAP_T,
        }
    }
}

#[derive(Copy, Clone)]
pub enum ClampMode {
    Edge,
    Repeat,
    RepeatMirrored,
}

impl ClampMode {
    pub fn get_native(&self) -> GLenum {
        match self {
            ClampMode::Edge => gl::CLAMP_TO_EDGE,
            ClampMode::Repeat => gl::REPEAT,
            ClampMode::RepeatMirrored => gl::MIRRORED_REPEAT,
        }
    }
}

#[derive(Copy, Clone)]
pub enum MinFilter {
    Nearest,
    Linear,
    NearestMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapNearest,
    LinearMipmapLinear,
}

impl MinFilter {
    pub fn get_native(&self) -> GLenum {
        match self {
            MinFilter::Nearest => gl::NEAREST,
            MinFilter::Linear => gl::LINEAR,
            MinFilter::NearestMipmapNearest => gl::NEAREST_MIPMAP_NEAREST,
            MinFilter::NearestMipmapLinear => gl::NEAREST_MIPMAP_LINEAR,
            MinFilter::LinearMipmapNearest => gl::LINEAR_MIPMAP_LINEAR,
            MinFilter::LinearMipmapLinear => gl::LINEAR_MIPMAP_LINEAR,
        }
    }
}

#[derive(Copy, Clone)]
pub enum MagFilter {
    Nearest,
    Linear,
}

impl MagFilter {
    pub fn get_native(&self) -> GLenum {
        match self {
            MagFilter::Nearest => gl::NEAREST,
            MagFilter::Linear => gl::LINEAR,
        }
    }
}

pub struct Texture {
    mipmaps: bool,
    handle: GLuint,
    s_clamp: ClampMode,
    t_clamp: ClampMode,
    min_filter: MinFilter,
    mag_filter: MagFilter,
    width: usize,
    height: usize,
}

impl Texture {
    pub fn new(width: usize, height: usize) -> Texture {
        let total_size = width * height * 4;
        let buf = vec![0u8; total_size];

        Texture::build_texture(&buf, width, height, false).unwrap()
    }

    pub fn make(buf: &Vec::<u8>, width: usize, height: usize, mipmaps: bool) -> Result<Texture> {
        Texture::build_texture(buf, width, height, mipmaps)
    }

    fn build_texture(buf: &[u8], width: usize, height: usize, mipmaps: bool) -> Result<Texture> {
        let mut handle = 0 as GLuint;
        let total_size = width * height * 4;

        if buf.len() != total_size {
            return Err(Error::InvalidTextureDimensions);
        }

        unsafe {
            gl::GenTextures(1, &mut handle);
            gl::BindTexture(gl::TEXTURE_2D, handle);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                &buf[0] as *const u8 as *const c_void,
            );

            if mipmaps {
                gl::GenerateMipmap(gl::TEXTURE_2D);
            }
        }

        Ok(Texture {
            mipmaps,
            handle,
            s_clamp: ClampMode::Edge,
            t_clamp: ClampMode::Edge,
            min_filter: MinFilter::Nearest,
            mag_filter: MagFilter::Nearest,
            width,
            height,
        })
    }

    pub fn bind(&self, unit: GLenum) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(gl::TEXTURE_2D, self.handle);
        }
    }

    pub fn write(&self, buf: &[u8], x: usize, y: usize, width: usize, height: usize) {
        unsafe {
            gl::TextureSubImage2D(
                self.handle,
                0,
                x as i32,
                y as i32,
                width as GLsizei,
                height as GLsizei,
                gl::RGBA as GLenum,
                gl::UNSIGNED_BYTE as GLenum,
                &buf[0] as *const u8 as *const c_void,
            );
        }
    }

    pub fn set_clamp(&mut self, coord: Coord, mode: ClampMode) {
        self.bind(0);

        unsafe {
            let raw_coord = coord as u32;

            gl::TexParameteri(gl::TEXTURE_2D, raw_coord, mode as i32);
        }

        match coord {
            Coord::S => self.s_clamp = mode,
            Coord::T => self.t_clamp = mode,
        }
    }

    pub fn set_min_filter(&mut self, filter: MinFilter) -> Result<()> {
        self.bind(0);

        match filter {
            MinFilter::Nearest | MinFilter::Linear => (),
            MinFilter::NearestMipmapNearest
            | MinFilter::NearestMipmapLinear
            | MinFilter::LinearMipmapNearest
            | MinFilter::LinearMipmapLinear => {
                if !self.mipmaps {
                    return Err(Error::NoMipmaps);
                }
            }
        };

        unsafe {
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                filter.get_native() as i32,
            );
        }

        self.min_filter = filter;
        Ok(())
    }

    pub fn set_mag_filter(&mut self, filter: MagFilter) {
        self.bind(0);

        unsafe {
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                filter.get_native() as i32,
            );
        }

        self.mag_filter = filter;
    }

    pub fn handle(&self) -> GLuint {
        self.handle
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.handle) };
        self.handle = 0;
    }
}
