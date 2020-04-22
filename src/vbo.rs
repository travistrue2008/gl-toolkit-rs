use crate::vertex::Vertex;

use gl::types::*;
use std::mem;
use std::os::raw::c_void;

pub enum AttributeKind {
    Byte,
    Short,
    Int,
    UnsignedByte,
    UnsignedShort,
    UnsignedInt,
    Half,
    Float,
    Double,
    Fixed,
}

impl AttributeKind {
    pub fn to_raw_enum(&self) -> GLenum {
        match self {
            AttributeKind::Byte => gl::BYTE,
            AttributeKind::Short => gl::SHORT,
            AttributeKind::Int => gl::INT,
            AttributeKind::UnsignedByte => gl::UNSIGNED_BYTE,
            AttributeKind::UnsignedShort => gl::UNSIGNED_SHORT,
            AttributeKind::UnsignedInt => gl::UNSIGNED_INT,
            AttributeKind::Half => gl::HALF_FLOAT,
            AttributeKind::Float => gl::FLOAT,
            AttributeKind::Double => gl::DOUBLE,
            AttributeKind::Fixed => gl::FIXED,
        }
    }

    pub fn size(&self) -> usize {
        match self {
            AttributeKind::Byte => mem::size_of::<GLchar>(),
            AttributeKind::Short => mem::size_of::<GLshort>(),
            AttributeKind::Int => mem::size_of::<GLint>(),
            AttributeKind::UnsignedByte => mem::size_of::<GLbyte>(),
            AttributeKind::UnsignedShort => mem::size_of::<GLushort>(),
            AttributeKind::UnsignedInt => mem::size_of::<GLuint>(),
            AttributeKind::Half => mem::size_of::<GLhalf>(),
            AttributeKind::Float => mem::size_of::<GLfloat>(),
            AttributeKind::Double => mem::size_of::<GLdouble>(),
            AttributeKind::Fixed => mem::size_of::<GLfixed>(),
        }
    }
}

pub enum PrimitiveKind {
    Points,
    Triangles,
    TriangleFan,
    TriangleStrip,
}

impl PrimitiveKind {
    pub fn to_raw_enum(&self) -> GLenum {
        match self {
            PrimitiveKind::Points => gl::POINTS,
            PrimitiveKind::Triangles => gl::TRIANGLES,
            PrimitiveKind::TriangleFan => gl::TRIANGLE_FAN,
            PrimitiveKind::TriangleStrip => gl::TRIANGLE_STRIP,
        }
    }
}

pub struct VBO {
    kind: PrimitiveKind,
    handle: GLuint,
    index_count: usize,
    vertex_count: usize,
}

impl VBO {
    pub fn make<T: Vertex>(kind: PrimitiveKind, vertices: &Vec<T>, indices: Option<&Vec<u16>>) -> VBO {
        let mut index_count = 0;

        let handle = unsafe {
            let mut vao = 0;

            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            vao
        };

        VBO::build_vertex_buffer(&vertices);

        if let Some(list) = indices {
            index_count = list.len();
            VBO::build_index_buffer(list);
        }

        unsafe { gl::BindVertexArray(0) };

        VBO {
            kind,
            handle,
            index_count,
            vertex_count: vertices.len(),
        }
    }

    fn build_vertex_buffer<T: Vertex>(vertices: &Vec::<T>) -> GLuint {
        let stride = mem::size_of::<T>() as GLsizei;
        let total_size = (vertices.len() * stride as usize) as GLsizeiptr;
        let root_ptr = &vertices[0] as *const T as *const c_void;

        unsafe {
            let mut vbo = 0;
            let mut offset = 0;

            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, total_size, root_ptr, gl::STATIC_DRAW);

            for (i, attr) in T::attrs().iter().enumerate() {
                let offset_ptr = offset as *const c_void;
                let normalized = match attr.0 {
                    false => gl::FALSE,
                    true => gl::TRUE,
                };

                gl::EnableVertexAttribArray(i as u32);
                gl::VertexAttribPointer(
                    i as GLuint,
                    attr.1 as GLint,
                    attr.2.to_raw_enum(),
                    normalized,
                    stride,
                    offset_ptr,
                );

                offset += attr.2.size() * attr.1;
            }

            vbo
        }
    }

    fn build_index_buffer(indices: &Vec::<u16>) -> GLuint {
        let total_size = (indices.len() * 2) as GLsizeiptr;
        let root_ptr = &indices[0] as *const u16 as *const c_void;

        unsafe {
            let mut ibo = 0;

            gl::GenBuffers(1, &mut ibo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, total_size, root_ptr, gl::STATIC_DRAW);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

            ibo as GLuint
        }
    }

    pub fn draw(&self) {
        let primitive_kind = self.kind.to_raw_enum();

        unsafe {
            gl::BindVertexArray(self.handle);

            if self.index_count > 0 {
                let root_ptr = 0 as *const u16 as *const c_void;

                gl::DrawElements(primitive_kind, self.index_count as i32, gl::UNSIGNED_SHORT, root_ptr);
            } else {
                gl::DrawArrays(primitive_kind, 0, self.vertex_count as i32);
            }

            gl::BindVertexArray(0);
        };
    }
}

impl Drop for VBO {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.handle) };
        self.handle = 0;
    }
}
