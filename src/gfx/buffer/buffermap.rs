use gl::types::*;
use std::ops::{Deref, DerefMut};

pub struct ReadBufferMap<'a, T> 
where
    T: Sized
{
    pub(crate) id: GLuint,
    pub(crate) buffer: &'a [T]
}

impl<T> Deref for ReadBufferMap<'_, T>
where
    T: Sized
{
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.buffer
    }
}

impl<T> Drop for ReadBufferMap<'_, T>
where
    T: Sized
{
    fn drop(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
            gl::UnmapBuffer(gl::ARRAY_BUFFER);
        }
    }
}

pub struct WriteBufferMap <'a, T> 
where
    T: Sized
{
    pub(crate) id: GLuint,
    pub(crate) buffer: &'a mut [T]
}

impl<T> Deref for WriteBufferMap<'_, T>
where
    T: Sized
{
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.buffer
    }
}

impl<T> DerefMut for WriteBufferMap<'_, T>
where
    T: Sized
{
    fn deref_mut(&mut self) -> &mut [T] {
        self.buffer
    }
}

impl<T> Drop for WriteBufferMap<'_, T>
where
    T: Sized
{
    fn drop(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
            gl::UnmapBuffer(gl::ARRAY_BUFFER);
        }
    }
}
