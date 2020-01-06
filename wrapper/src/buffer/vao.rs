use gl::types::*;
use super::{Primitive, Buffer, BufferData, BufferAcces, BufferType, Format};
use crate::get_value;

use anyhow::{Result, bail};

use std::collections::HashMap;

type AttributePoint = (GLuint, GLint, GLenum, GLboolean, GLsizei, GLuint);

pub struct Vao {
    id: GLuint,
    format: Format,
    bindings: HashMap<GLuint, Vec<AttributePoint>>,
    location_count: GLuint
}

impl Vao {
    pub fn new(format: Format, locations: GLuint) -> Vao {
        let id = get_value(0, |id| unsafe {
            gl::GenVertexArrays(1, id);
        });
        
        let vao = Vao { 
            id,
            format: format,
            bindings: HashMap::new(),
            location_count: locations
        };

        vao.bind();
        for i in 0..locations {
            unsafe {
                gl::EnableVertexAttribArray(i as GLuint);
            }
        }

        vao
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn bind_vbo<T, Kind, Acces>(
        &mut self,
        location: GLuint,
        vbo: &mut Buffer<T, Kind, Acces>
    ) -> Result<Option<GLuint>>
    where
        T: Sized + BufferData,
        Kind: BufferType,
        Acces: BufferAcces
    {
        let mut bound = false;

        // Check if the buffer is already bound
        if let Some(bindings) = self.bindings.get(&vbo.id()) {
            bound = bindings.len() == 0 || bindings[0].0 != location;
            if bound {
                // If it is apply the binding
                self.rebind_vbo(vbo, bindings);
            }
        }
        
        // If it is not create a new binding and apply it
        if !bound {
            // Generate binding
            let prototype = T::prototype();
            let bindings = self.generate_binding(
                std::mem::size_of::<T>() as GLuint,
                location,
                prototype
            )?;
            
            // Apply binding
            self.rebind_vbo(vbo, &bindings);
            let bindings_len = bindings.len();
            self.bindings.insert(vbo.id(), bindings);

            // Return the index of the next location
            Ok(Some(location + bindings_len as GLuint))
        }else {
            Ok(None)
        }
    }

    fn rebind_vbo<T, Kind, Acces>(
        &self,
        vbo: &mut Buffer<T, Kind, Acces>,
        bindings: &Vec<AttributePoint>
    ) 
    where
        T: Sized + BufferData,
        Kind: BufferType,
        Acces: BufferAcces
    {
        // set vao and vbo as active
        self.bind();
        vbo.bind();
        for (location, size, ty, norm, stride, offset) in bindings {
            unsafe {
                gl::VertexAttribPointer(
                    *location,
                    *size,
                    *ty,
                    *norm,
                    *stride,
                    *offset as *const GLvoid,
                );
            }
        }
    }

    pub fn generate_binding(
        &mut self,
        object_size: GLuint,
        location: GLuint,
        prototype: Vec<(Primitive, GLuint)>
    ) -> Result<Vec<AttributePoint>> {
        // Check prototype size
        let prototype_len = prototype.iter().fold(0, |acc, (ty, count)| acc + ty.size() * count);        

        if object_size != prototype_len {
            bail!("Invalid prototype size of {} on object of size {}", prototype_len, object_size);
        }

        // Keep track of offset between parameters
        let mut offset: GLuint = 0;
        let mut bindings = Vec::new();
        for (id, (ty, count)) in prototype.iter().enumerate() {
            if (id as GLuint) + location > self.location_count {
                bail!("Ran out of shader variable location had {} but was making {}", self.location_count, (id as GLuint) + location);
            }

            match ty {
                Primitive::Nothing => (),
                _ => {
                    bindings.push((
                        (id as GLuint) + location,
                        *count as GLint,
                        ty.value(),
                        gl::FALSE,
                        prototype_len as gl::types::GLint,
                        offset
                    ));
                }
            };

            offset += ty.size() * count;
        }
        Ok(bindings)
    }

    pub fn draw_arrays(&mut self, i0: GLuint, len: GLuint) {
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
