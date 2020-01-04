use gl::types::*;
use std::marker::PhantomData;
use super::{Primitive, Format, BufferType, ReadBufferMap, WriteBufferMap, DynamicBuffer};
use crate::gfx::get_value;

use std::borrow::{BorrowMut, Borrow};

pub trait VboData {
    fn prototype() -> Vec<(Primitive, GLuint)>;
}

pub struct Vbo<T, Ty> 
where
    T: Sized + VboData,
    Ty: BufferType
{
    id: GLuint,
    len: usize,
    data: PhantomData<T>,
    ty: PhantomData<Ty>
}

impl<T, Ty> Vbo<T, Ty> 
where
    T: Sized + VboData,
    Ty: BufferType
{
    pub fn new(data: &[T]) -> Vbo<T, Ty> {
        let mut id = get_value(0, |id|unsafe {
            gl::GenBuffers(1, id);
        });

        let mut vbo = Vbo {
            id,
            len: data.len(),
            data: PhantomData,
            ty: PhantomData
        };

        vbo.bind();
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data as *const [T] as *const GLvoid,
                Ty::value(),
            );
        }

        vbo
    }

    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn bind(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    pub fn update(&mut self, data: &[T], offset: GLuint) {
        self.bind();
        self.len = data.len();
        unsafe {
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                (offset as usize * std::mem::size_of::<T>()) as isize,
                (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data as *const [T] as *const GLvoid
            );
        }
    }

    pub fn read<'a>(&self) -> ReadBufferMap<'a, T>{
        let ptr = unsafe {
            gl::MapBuffer(
                gl::ARRAY_BUFFER,
                gl::READ_ONLY
            )
        } as *const T;

        ReadBufferMap {
            id: self.id,
            buffer: unsafe {
                std::slice::from_raw_parts(ptr as *const T, self.len)
            }
        }
    }

    
}

impl<T> Vbo<T, DynamicBuffer> 
where
    T: Sized + VboData,
{
    pub fn write<'a>(&self) -> WriteBufferMap<'a, T>{
        let ptr = unsafe {
            gl::MapBuffer(
                gl::ARRAY_BUFFER,
                gl::READ_WRITE
            )
        } as *const T;

        WriteBufferMap {
            id: self.id,
            buffer: unsafe {
                std::slice::from_raw_parts_mut(ptr as *mut T, self.len)
            }
        }
    }
}

impl<T, Ty> Drop for Vbo<T, Ty> 
where
    T: Sized + VboData,
    Ty: BufferType
{
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
