
use std::ops::Deref;
use super::*;

pub struct ReadBufferMap<'a, T, Kind, Acces> 
where
    T: Sized + BufferData,
    Kind: BufferType,
    Acces: BufferAcces
{
    pub(crate) buff: &'a Buffer<T, Kind, Acces>,
    pub(crate) buffer: &'a [T]
}

impl<T, Kind, Acces> Deref for ReadBufferMap<'_, T, Kind, Acces> 
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

impl<T, Kind, Acces> Drop for ReadBufferMap<'_, T, Kind, Acces> 
where
    T: Sized + BufferData,
    Kind: BufferType,
    Acces: BufferAcces
{
    fn drop(&mut self) {
        unsafe {
            gl::BindBuffer(Kind::value(), self.buff.id());
            gl::UnmapBuffer(Kind::value());
        }
    }
}
