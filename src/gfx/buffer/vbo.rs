use gl::types::*;
use std::marker::PhantomData;
use super::{Primitive, Format};
use crate::gfx::get_value;

pub trait VboData {
    fn prototype() -> Vec<(Primitive, GLuint)>;
}

pub enum VboDataType {
    Static,
    Dynamic,
}

impl VboDataType {
    pub fn value(&self) -> GLenum {
        match self {
            VboDataType::Static => gl::STATIC_DRAW,
            VboDataType::Dynamic => gl::DYNAMIC_DRAW,
        }
    }
}

pub struct Vbo<T> 
where
    T: Sized + VboData
{
    id: GLuint,
    data: PhantomData<T>
}

impl<T> Vbo<T> 
where
    T: Sized + VboData
{
    pub fn new() -> Vbo<T> {
        let mut id = get_value(0, |id|unsafe {
            gl::GenBuffers(1, id);
        });

        Vbo {
            id,
            data: PhantomData
        }
    }

    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn bind(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    pub fn bind_data(&mut self, data: &[T], data_type: VboDataType) {
        self.bind();
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data as *const [T] as *const GLvoid,
                data_type.value(),
            );
        }
    }
}


impl<T> Drop for Vbo<T> 
where
    T: Sized + VboData
{
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
