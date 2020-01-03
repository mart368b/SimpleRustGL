use gl::types::*;
use super::{BufferType, IndexPrimitive};
use crate::gfx::get_value;
use std::marker::PhantomData;
use std::borrow::Borrow;

pub struct Ebo<T, Ty> 
where
    T:IndexPrimitive,
    Ty: BufferType
{
    id: GLuint,
    len: GLuint,
    marker: PhantomData<T>,
    ty: PhantomData<Ty>
}

impl<T, Ty> Ebo<T, Ty>
where
    T:IndexPrimitive,
    Ty: BufferType
{
    pub fn new(data: &[T]) -> Ebo<T, Ty> {
        let mut id = get_value(0, |id| unsafe {
            gl::GenBuffers(1, id);
        });

        let mut ebo = Ebo {
            id,
            len: data.len() as GLuint,
            marker: PhantomData,
            ty: PhantomData
        };

        ebo.bind();

        unsafe {
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data as *const [T] as *const GLvoid,
                Ty::value(),
            );
        }

        ebo
    }

    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn bind(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }

    pub fn update(&mut self, data: &[T], offset: GLuint) {
        unsafe {
            gl::BufferSubData(
                gl::ELEMENT_ARRAY_BUFFER,
                (offset as usize * std::mem::size_of::<T>()) as isize,
                (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data as *const [T] as *const GLvoid
            );
        }
    }
}

impl<T, Ty> Drop for Ebo<T, Ty>
where
    T:IndexPrimitive,
    Ty: BufferType
{
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}