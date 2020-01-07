use gl::types::*;
use std::ops::{Deref, DerefMut};
use super::*;

pub struct WriteBufferMap <'a, T, Kind, Acces> 
where
    T: Sized + BufferData,
    Kind: BufferType,
    Acces: BufferAcces
{
    pub(crate) buff: &'a mut Buffer<T, Kind, Acces>,
    pub(crate) buffer: &'a mut [T]
}

impl<T, Kind, Acces> Deref for WriteBufferMap<'_, T, Kind, Acces> 
where
    T: Sized + BufferData,
    Kind: BufferType,
    Acces: BufferAcces
{
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.buffer
    }
}

impl<T, Kind, Acces> DerefMut for WriteBufferMap<'_, T, Kind, Acces> 
where
    T: Sized + BufferData,
    Kind: BufferType,
    Acces: BufferAcces
{
    fn deref_mut(&mut self) -> &mut [T] {
        self.buffer
    }
}

impl<T, Kind, Acces> Drop for WriteBufferMap<'_, T, Kind, Acces> 
where
    T: Sized + BufferData,
    Kind: BufferType,
    Acces: BufferAcces
{
    fn drop(&mut self) {
        unsafe {
            self.buff.bind();
            self.buff.len = self.buffer.len();
            gl::UnmapBuffer(Kind::value());
        }
    }
}