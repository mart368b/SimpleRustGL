mod vbo;
mod vao;
mod ebo;

pub use ebo::Ebo;
pub use vao::Vao;
pub use vbo::{Vbo, VboDataType, VboData};

use gl::types::*;

pub enum Primitive {
    Byte,
    UByte,
    Short,
    UShort,
    Int,
    UInt,
    Float,
    Double
}

impl Primitive {
    pub fn value(&self) -> GLenum {
        match self {
            Primitive::Byte => gl::BYTE,
            Primitive::UByte => gl::UNSIGNED_BYTE,
            Primitive::Short => gl::SHORT,
            Primitive::UShort => gl::UNSIGNED_SHORT,
            Primitive::Int => gl::INT,
            Primitive::UInt => gl::UNSIGNED_INT,
            Primitive::Float => gl::FLOAT,
            Primitive::Double => gl::DOUBLE
        }
    }

    pub fn size(&self) -> GLuint {
        let size = match self {
            Primitive::Byte => std::mem::size_of::<i8>(),
            Primitive::UByte => std::mem::size_of::<u8>(),
            Primitive::Short => std::mem::size_of::<i16>(),
            Primitive::UShort => std::mem::size_of::<u16>(),
            Primitive::Int => std::mem::size_of::<i32>(),
            Primitive::UInt => std::mem::size_of::<u32>(),
            Primitive::Float => std::mem::size_of::<f32>(),
            Primitive::Double => std::mem::size_of::<f64>(),
        };
        size as GLuint
    }
}

pub enum Format {
    Points,
    LineStrip,
    LineLoop,
    Lines,
    LineStripAdj,
    LinesAdj,
    TriangleStrip,
    TriangleFan,
    Triangles,
    TriangleStripAdj,
    TriangleAdj,
    Patches,
    Quard
}

impl Format {
    pub fn value(&self) -> GLenum {
        match self {
            Format::Points => gl::POINTS,
            Format::LineStrip => gl::LINE_STRIP,
            Format::LineLoop => gl::LINE_LOOP,
            Format::Lines => gl::LINES,
            Format::LineStripAdj => gl::LINE_STRIP_ADJACENCY,
            Format::LinesAdj => gl::LINES_ADJACENCY,
            Format::TriangleStrip => gl::TRIANGLE_STRIP,
            Format::TriangleFan => gl::TRIANGLE_FAN,
            Format::Triangles => gl::TRIANGLES,
            Format::TriangleStripAdj => gl::TRIANGLE_STRIP_ADJACENCY,
            Format::TriangleAdj => gl::TRIANGLES_ADJACENCY,
            Format::Patches => gl::PATCHES,
            Format::Quard => gl::QUADS,
        }
    }
}