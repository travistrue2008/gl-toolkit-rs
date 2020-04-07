use crate::{Error, Result};

use gl::types::*;
use std::ffi::CString;
use std::ptr;
use std::str;

pub enum StageKind {
    Vertex,
    Geometry,
    Fragment,
}

impl StageKind {
    fn get_native(&self) -> GLenum {
        match self {
            StageKind::Vertex => gl::VERTEX_SHADER,
            StageKind::Geometry => gl::GEOMETRY_SHADER,
            StageKind::Fragment => gl::FRAGMENT_SHADER,
        }
    }
}

pub struct Stage {
    handle: GLuint,
}

impl Stage {
    pub fn make(kind: StageKind, src: &str) -> Result<Stage> {
        unsafe {
            let src_c_str = CString::new(src.as_bytes()).unwrap();
            let handle: GLuint = gl::CreateShader(kind.get_native());

            gl::ShaderSource(handle, 1, &src_c_str.as_ptr(), ptr::null());
            gl::CompileShader(handle);

            let mut success = gl::FALSE as GLint;
            let mut log = Vec::with_capacity(512);

            log.set_len(511);
            gl::GetShaderiv(handle, gl::COMPILE_STATUS, &mut success);

            if success != gl::TRUE as GLint {
                let log_ptr = log.as_mut_ptr() as *mut GLchar;
                gl::GetShaderInfoLog(handle, 512, ptr::null_mut(), log_ptr);

                let err = str::from_utf8(&log).unwrap().into();
                Err(Error::CompileShaderStageFailed(err))
            } else {
                Ok(Stage { handle })
            }
        }
    }
}

impl Drop for Stage {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.handle) };
        self.handle = 0;
    }
}

pub struct Shader {
    handle: GLuint,
}

impl Shader {
    pub fn make(stages: &Vec<Stage>) -> Result<Shader> {
        unsafe {
            let handle = gl::CreateProgram();
            for stage in stages {
                gl::AttachShader(handle, stage.handle);
            }

            gl::LinkProgram(handle);

            let mut success = gl::FALSE as GLint;
            gl::GetProgramiv(handle, gl::LINK_STATUS, &mut success);

            if success != gl::TRUE as GLint {
                let mut log = Vec::with_capacity(512);
                log.set_len(511);

                let log_ptr = log.as_mut_ptr() as *mut GLchar;
                gl::GetShaderInfoLog(handle, 512, ptr::null_mut(), log_ptr);

                let err = str::from_utf8(&log).unwrap().into();
                Err(Error::LinkShaderProgramFailed(err))
            } else {
                Ok(Shader { handle })
            }
        }
    }

    pub fn bind(&self) {
        unsafe { gl::UseProgram(self.handle) };
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.handle) };
        self.handle = 0;
    }
}
