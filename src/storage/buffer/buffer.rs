use gl::types::*;
use std::marker::PhantomData;
use super::{BufferType, BufferData, BufferAcces, ReadBufferMap, WriteBufferMap, DynamicBuffer};
use crate::get_value;

pub struct Buffer<T, Kind, Acces>
where
    T: Sized + BufferData,
    Kind: BufferType,
    Acces: BufferAcces
{
    pub(crate) id: GLuint,
    pub(crate) len: usize,
    pub(crate) data: PhantomData<T>,
    pub(crate) kind: PhantomData<Kind>,
    pub(crate) acces: PhantomData<Acces>,
}

impl<T, Kind, Acces> Buffer<T, Kind, Acces>
where
    T: Sized + BufferData,
    Kind: BufferType,
    Acces: BufferAcces
{
    pub fn new(data: &[T]) -> Buffer<T, Kind, Acces> {
        let id = get_value(0, |id| unsafe {
            gl::GenBuffers(1, id);
        });

        let vbo = Buffer {
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

    pub fn bind(&self) {
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

    pub fn read<'a>(&self) -> ReadBufferMap<'_, T, Kind, Acces>{
        let ptr = unsafe {
            gl::MapBuffer(
                Kind::value(),
                gl::READ_ONLY
            )
        } as *const T;

        let val = unsafe {
            std::slice::from_raw_parts(ptr as *const T, self.len)
        };

        ReadBufferMap {
            buff: self,
            buffer: val
        }
    }

    
}

impl<T, Kind> Buffer<T, Kind, DynamicBuffer> 
where
    T: Sized + BufferData,
    Kind: BufferType,
{
    pub fn write<'a>(&mut self) -> WriteBufferMap<'_, T, Kind, DynamicBuffer>{
        let ptr = unsafe {
            gl::MapBuffer(
                Kind::value(),
                gl::READ_WRITE
            )
        } as *const T;

        let val = unsafe {
            std::slice::from_raw_parts_mut(ptr as *mut T, self.len)
        };

        WriteBufferMap {
            buff: self,
            buffer: val
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
