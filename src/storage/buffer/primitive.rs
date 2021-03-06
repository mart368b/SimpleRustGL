use gl::types::*;

pub enum Primitive {
    Byte,
    Short,
    UShort,
    Int,
    UInt,
    Float,
    Double,
    Nothing
}

impl Primitive {
    pub fn value(&self) -> GLenum {
        match self {
            Primitive::Byte => gl::BYTE,
            Primitive::Short => gl::SHORT,
            Primitive::UShort => gl::UNSIGNED_SHORT,
            Primitive::Int => gl::INT,
            Primitive::UInt => gl::UNSIGNED_INT,
            Primitive::Float => gl::FLOAT,
            Primitive::Double => gl::DOUBLE,
            Primitive::Nothing => unimplemented!()
        }
    }

    pub fn size(&self) -> GLuint {
        let size = match self {
            Primitive::Byte => std::mem::size_of::<u8>(),
            Primitive::Short => std::mem::size_of::<i16>(),
            Primitive::UShort => std::mem::size_of::<u16>(),
            Primitive::Int => std::mem::size_of::<i32>(),
            Primitive::UInt => std::mem::size_of::<u32>(),
            Primitive::Float => std::mem::size_of::<f32>(),
            Primitive::Double => std::mem::size_of::<f64>(),
            Primitive::Nothing => 1
        };
        size as GLuint
    }
}