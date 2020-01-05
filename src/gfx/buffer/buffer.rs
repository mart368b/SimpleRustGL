use gl::types::*;
use std::marker::PhantomData;
use super::{BufferType, BufferData, BufferAcces, ReadBufferMap, WriteBufferMap, DynamicBuffer};
use crate::gfx::get_value;

pub struct Buffer<T, Kind, Acces>
where
    T: Sized + BufferData,
    Kind: BufferType,
    Acces: BufferAcces
{
    id: GLuint,
    len: usize,
    data: PhantomData<T>,
    kind: PhantomData<Kind>,
    acces: PhantomData<Acces>,
}

impl<T, Kind, Acces> Buffer<T, Kind, Acces>
where
    T: Sized + BufferData,
    Kind: BufferType,
    Acces: BufferAcces
{
    pub fn new(data: &[T]) -> Buffer<T, Kind, Acces> {
        let id = get_value(0, |id|unsafe {
            gl::GenBuffers(1, id);
        });

        let mut vbo = Buffer {
            id,
            len: data.len(),
            data: PhantomData,
            kind: PhantomData,
            acces: PhantomData,
        };

        vbo.bind();
        unsafe {
            gl::BufferData(
                Kind::value(),
                (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data as *const [T] as *const GLvoid,
                Acces::value(),
            );
        }

        vbo
    }

    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn bind(&mut self) {
        unsafe {
            gl::BindBuffer(Kind::value(), self.id);
        }
    }

    pub fn update(&mut self, data: &[T], offset: GLuint) {
        self.bind();
        self.len = data.len();
        unsafe {
            gl::BufferSubData(
                Kind::value(),
                (offset as usize * std::mem::size_of::<T>()) as isize,
                (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data as *const [T] as *const GLvoid
            );
        }
    }

    pub fn read<'a>(&self) -> ReadBufferMap<'a, T>{
        let ptr = unsafe {
            gl::MapBuffer(
                Kind::value(),
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

impl<T, Kind> Buffer<T, Kind, DynamicBuffer> 
where
    T: Sized + BufferData,
    Kind: BufferType,
{
    pub fn write<'a>(&self) -> WriteBufferMap<'a, T>{
        let ptr = unsafe {
            gl::MapBuffer(
                Kind::value(),
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

impl<T, Kind, Acces> Drop for Buffer<T, Kind, Acces> 
where
    T: Sized + BufferData,
    Kind: BufferType,
    Acces: BufferAcces
{
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
