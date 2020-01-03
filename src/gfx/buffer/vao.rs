use gl::types::*;
use std::marker::PhantomData;
use super::{Primitive, Vbo, VboData, BufferType, Format};
use crate::gfx::get_value;

pub struct Vao {
    id: GLuint,
    format: Format
}

impl Vao {
    pub fn new(format: Format) -> Vao {
        
        let mut id = get_value(0, |id| unsafe {
            gl::GenVertexArrays(1, id);
        });
        
        Vao { 
            id,
            format: format
        }
    }

    pub fn bind(&mut self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn bind_vbo<T, Ty>(
        &mut self,
        location: GLuint,
        vbo: &mut Vbo<T, Ty>
    ) -> GLuint
    where
        T: VboData + Sized,
        Ty: BufferType
    {
        vbo.bind();
        let prototype = T::prototype();
        let prototype_len = prototype.iter().fold(0, |acc, (ty, count)| acc + ty.size() * count);
        assert_eq!(std::mem::size_of::<T>() as GLuint, prototype_len);
        self.bind_prototype(
            location,
            prototype
        );
        location + prototype_len
    }

    pub fn bind_prototype(
        &mut self,
        location: GLuint,
        prototype: Vec<(Primitive, GLuint)>
    ) {
        let prototype_len = prototype.iter().fold(0, |acc, (ty, count)| acc + ty.size() * count);

        self.bind();
        let mut offset: GLuint = 0;
        for (id, (ty, count)) in prototype.iter().enumerate() {
            unsafe {
                gl::EnableVertexAttribArray(id as GLuint);
                gl::VertexAttribPointer(
                    (id as GLuint) + location,
                    *count as GLint,
                    ty.value(),
                    gl::FALSE,
                    prototype_len as gl::types::GLint,
                    offset as *const GLvoid,
                );
            }
            println!("{}, {}, {}, {}", 
                (id as GLuint) + location,
                *count as GLint,
                prototype_len as gl::types::GLint,
                offset,
            );
            offset += ty.size() * count;
        }
    }

    pub fn bind_attribute(
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

    pub fn draw_arrays(&mut self, i0:GLuint, len: GLuint) {
        unsafe {
            gl::DrawArrays(
                self.format.value(),
                i0 as GLint,
                len as GLint,
            );
        }
    }

    pub fn draw_elements(&mut self, len: GLuint, ty: Primitive, i0: GLuint) {
        unsafe {
            gl::DrawElements(
                self.format.value(),
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
