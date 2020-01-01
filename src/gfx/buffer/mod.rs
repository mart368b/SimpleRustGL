mod vbo;
mod vao;
mod index_vbo;

pub use index_vbo::IndexVbo;
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