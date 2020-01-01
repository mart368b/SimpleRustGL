use gl::types::*;
use std::marker::PhantomData;
use super::{Primitive, Vbo, VboData, VboDataType};

pub struct Vao {
    id: GLuint,
    locations: GLuint
}

impl Vao {
    pub fn new() -> Vao {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        Vao { 
            id,
            locations: 0
        }
    }

    pub fn bind(&mut self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn bind_vbo<T>(
        &mut self,
        vbo: &mut Vbo<T>
    ) -> GLuint
    where
        T: VboData + Sized
    {
        vbo.bind();
        let i0 = self.locations;
        let prototype = T::prototype();
        let prototype_len = prototype.iter().fold(0, |acc, (ty, count)| acc + ty.size() * count);
        assert_eq!(std::mem::size_of::<T>() as GLuint, prototype_len);
        self.bind_points(
            prototype
        );
        i0
    }

    pub fn bind_points(
        &mut self,
        prototype: Vec<(Primitive, GLuint)>
    ) {
        let prototype_len = prototype.iter().fold(0, |acc, (ty, count)| acc + ty.size() * count);
        self.bind();
        let mut offset: GLuint = 0;
        for (id, (ty, count)) in prototype.iter().enumerate() {
            unsafe {
                gl::EnableVertexAttribArray(id as GLuint);
                gl::VertexAttribPointer(
                    (id as GLuint) + self.locations,
                    *count as GLint,
                    ty.value(),
                    gl::FALSE,
                    prototype_len as gl::types::GLint,
                    offset as *const GLvoid,
                );
            }
            offset += ty.size() * count;
        }
        self.locations += prototype.len() as GLuint;
    }

    pub fn attr_points(
        &mut self,
        id: GLuint,
        pre_offset: GLuint,
        primitive: Primitive,
        count: GLuint,
        post_offset: GLuint
    ) {
        self.bind();
        unsafe {
            gl::EnableVertexAttribArray(id); // this is "layout (location = 0)" in vertex shader
            gl::VertexAttribPointer(
                id,         // index of the generic vertex attribute ("layout (location = 0)")
                count as GLint,         // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                (pre_offset + count * primitive.size() + post_offset) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                pre_offset as *const GLvoid,                                     // offset of the first component
            );
        }
    }

    pub fn draw_arrays(&mut self, format: GLenum, i0:GLuint, len: GLuint) {
        unsafe {
            gl::DrawArrays(
                format,
                i0 as GLint,
                len as GLint,
            );
        }
    }

    pub fn draw_elements(&mut self, format: GLenum, len: GLuint, ty: Primitive, i0: GLuint) {
        unsafe {
            gl::DrawElements(
                format,
                len as GLint,
                ty.value(),
                i0 as *const GLvoid,
            );
        }
    }
}


impl Drop for Vao {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}
