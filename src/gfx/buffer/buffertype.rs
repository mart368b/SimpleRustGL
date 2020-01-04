use gl::types::*;

// BUFFER TYPES
pub trait BufferType {
    fn value() -> GLenum;
}

pub struct ElementArrayBuffer();
pub struct ArrayBuffer();

impl BufferType for ArrayBuffer {
    fn value() -> GLenum {
        gl::ARRAY_BUFFER
    }
}

impl BufferType for ElementArrayBuffer {
    fn value() -> GLenum {
        gl::ELEMENT_ARRAY_BUFFER
    }
}

// BUFFER ACCES
pub trait BufferAcces {
    fn value() -> GLenum;
}

pub struct StaticBuffer();
pub struct DynamicBuffer();

impl BufferAcces for StaticBuffer {
    fn value() -> GLenum {
        gl::STATIC_DRAW
    }
}

impl BufferAcces for DynamicBuffer {
    fn value() -> GLenum {
        gl::DYNAMIC_DRAW
    }
}