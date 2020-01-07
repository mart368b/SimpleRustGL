use gl::types::*;

pub enum Primitive {
    Int,
    UInt,
    Float,
    Double,
    Nothing
}

impl Primitive {
    pub fn value(&self) -> GLenum {
        match self {
            Primitive::Int => gl::INT,
            Primitive::UInt => gl::UNSIGNED_INT,
            Primitive::Float => gl::FLOAT,
            Primitive::Double => gl::DOUBLE,
            Primitive::Nothing => 0
        }
    }

    pub fn size(&self) -> GLuint {
        let size = match self {
            Primitive::Int => std::mem::size_of::<i32>(),
            Primitive::UInt => std::mem::size_of::<u32>(),
            Primitive::Float => std::mem::size_of::<f32>(),
            Primitive::Double => std::mem::size_of::<f64>(),
            Primitive::Nothing => 1
        };
        size as GLuint
    }
}