use gl::types::*;
use std::mem;
use std::os::raw::c_void;

#[derive(Debug, Copy, Clone)]
pub enum BufferKind {
    Vertex,
    Index,
}

impl BufferKind {
    pub fn to_raw_enum(&self) -> GLenum {
        match self {
            BufferKind::Vertex => gl::ARRAY_BUFFER,
            BufferKind::Index => gl::ELEMENT_ARRAY_BUFFER,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BufferMode {
    StaticDraw,
    StaticRead,
    StaticCopy,
    DynamicDraw,
    DynamicRead,
    DynamicCopy,
    StreamDraw,
    StreamRead,
    StreamCopy,
}

impl BufferMode {
    pub fn to_raw_enum(&self) -> GLenum {
        match self {
            BufferMode::StaticDraw => gl::STATIC_DRAW,
            BufferMode::StaticRead => gl::STATIC_READ,
            BufferMode::StaticCopy => gl::STATIC_COPY,
            BufferMode::DynamicDraw => gl::DYNAMIC_DRAW,
            BufferMode::DynamicRead => gl::DYNAMIC_READ,
            BufferMode::DynamicCopy => gl::DYNAMIC_COPY,
            BufferMode::StreamDraw => gl::STREAM_DRAW,
            BufferMode::StreamRead => gl::STREAM_READ,
            BufferMode::StreamCopy => gl::STREAM_COPY,
        }
    }
}

#[derive(Debug, Copy, Clone)]
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

pub trait Vertex: Sized {
    fn attrs() -> Vec<(bool, usize, AttributeKind)>;
    fn new() -> Self;
}

pub struct VBO {
    mode: BufferMode,
    primitive_kind: PrimitiveKind,
    handle: GLuint,
    vbo_handle: GLuint,
    ibo_handle: GLuint,
    index_count: usize,
    vertex_count: usize,
}

impl VBO {
    pub fn new<T: Vertex>(mode: BufferMode, primitive_kind: PrimitiveKind, vertices: &Vec::<T>, indices: Option<&Vec::<u16>>) -> VBO {
        let mut index_count = 0;
        let mut ibo_handle = 0;

        let handle = unsafe {
            let mut vao = 0;

            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            vao
        };

        let vbo_handle = VBO::build_vertex_buffer(mode, &vertices);

        if let Some(list) = indices {
            index_count = list.len();
            ibo_handle = VBO::build_index_buffer(list);
        }

        unsafe { gl::BindVertexArray(0) };

        VBO {
            mode,
            primitive_kind,
            handle,
            vbo_handle,
            ibo_handle,
            index_count,
            vertex_count: vertices.len(),
        }
    }

    fn build_vertex_buffer<T: Vertex>(mode: BufferMode, vertices: &Vec::<T>) -> GLuint {
        let stride = mem::size_of::<T>() as GLsizei;
        let total_size = (vertices.len() * stride as usize) as GLsizeiptr;
        let root_ptr = &vertices[0] as *const T as *const c_void;

        unsafe {
            let mut vbo = 0;
            let mut offset = 0;

            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, total_size, root_ptr, mode.to_raw_enum());

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

            ibo as GLuint
        }
    }

    fn get_buffer_handle(&self, kind: BufferKind) -> GLuint {
        match kind {
            BufferKind::Vertex => self.vbo_handle,
            BufferKind::Index => self.ibo_handle,
        }
    }

    fn write<T: Sized>(&self, kind: BufferKind, vertices: &Vec::<T>, offset: usize) {
        let size = mem::size_of::<T>() as isize;
        let offset = offset as isize * size;
        let total_size = vertices.len()  as isize * size;
        let root_ptr = &vertices[0] as *const T as *const c_void;
        let raw_kind = kind.to_raw_enum();
        let handle = self.get_buffer_handle(kind);

        unsafe {
            gl::BindVertexArray(self.handle);
            gl::BindBuffer(raw_kind, handle);
            gl::BufferSubData(raw_kind, offset, total_size, root_ptr);
        };
    }

    pub fn mode(&self) -> BufferMode {
        self.mode
    }

    pub fn write_vertices<T: Vertex>(&self, vertices: &Vec::<T>, offset: usize) {
        self.write(BufferKind::Vertex, vertices, offset);
    }

    pub fn write_indices<T: Vertex>(&self, indices: &Vec::<u16>, offset: usize) {
        self.write(BufferKind::Index, indices, offset);
    }

    pub fn render(&self) {
        let kind = self.primitive_kind.to_raw_enum();

        unsafe {
            gl::BindVertexArray(self.handle);

            if self.index_count > 0 {
                let root_ptr = 0 as *const u16 as *const c_void;

                gl::DrawElements(kind, self.index_count as i32, gl::UNSIGNED_SHORT, root_ptr);
            } else {
                gl::DrawArrays(kind, 0, self.vertex_count as i32);
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
