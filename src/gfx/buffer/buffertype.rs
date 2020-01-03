use gl::types::*;

pub struct StaticBuffer();
pub struct DynamicBuffer();

pub trait BufferType {
    fn value() -> GLenum;
}

impl BufferType for StaticBuffer {
    fn value() -> GLenum {
        gl::STATIC_DRAW
    }
}

impl BufferType for DynamicBuffer {
    fn value() -> GLenum {
        gl::DYNAMIC_DRAW
    }
}