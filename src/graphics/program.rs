use gl::types::*;
use super::*;
use anyhow::{Result, anyhow};
use crate::error::get_program_error;
use std::rc::Rc;
use crate::get_value;
use std::collections::HashMap;


pub trait ProgramAttachment {
    fn id(&self) -> GLuint;
}

impl<T> ProgramAttachment for Shader<T> 
    where
        T: ShaderExt
{
    fn id(&self) -> GLuint {
        self.id()
    }
}

pub struct Program {
    id: GLuint,
    shaders: Vec<Rc<dyn ProgramAttachment>>,
    uniform_locations: HashMap<String, GLint>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            id: unsafe{gl::CreateProgram()},
            shaders: Vec::new(),
            uniform_locations: HashMap::new(),
        }
    }

    pub fn from_shaders(shaders: Vec<Rc<dyn ProgramAttachment>>) -> Result<Program> 
    {
        let mut program = Program::new();
        for shader in shaders {
            program.attach(shader);
        }
        program.link()?;
        Ok(program)
    }

    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn attach(&mut self, shader: Rc<dyn ProgramAttachment>) {
        unsafe {
            gl::AttachShader(self.id, shader.id())
        }
        self.shaders.push(shader);
    }

    pub fn link(&mut self) -> Result<()> {
        unsafe { gl::LinkProgram(self.id); }

        let success = get_value(1, |success|unsafe {
            gl::GetProgramiv(self.id, gl::LINK_STATUS, success);
        });

        match success {
            1 => Ok(()),
            _ => {
                Err(anyhow!(
                    "Failed to link program: {}", 
                    get_program_error(self.id)
                ))
            }
        }
    }

    pub fn set_uniform<K>(&mut self, name: &str, val: K)
    where
        K: Uniform<K>
    {
        if let Some(loc) = self.get_uniform_loc(name) {
            unsafe {
                val.set(loc);
            }
        }
    }

    /// Returns the uniform location for the name or none, if it fails.
    fn get_uniform_loc(&mut self, name: &str) -> Option<GLint> {
        if let Some(loc) = self.uniform_locations.get(name) {
            return Some(*loc);
        }

        use std::ffi::CString;
        let c_name = CString::new(name).unwrap();
        let loc = unsafe { gl::GetUniformLocation(self.id, c_name.as_ptr()) };
        if loc != -1 {
            self.uniform_locations.insert(name.to_owned(), loc);
            Some(loc)
        } else {
            None
        }
    }

    pub fn set_used(&mut self) {
        unsafe {gl::UseProgram(self.id)}
    }

}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            for shader in &self.shaders {
                gl::DetachShader(self.id, shader.id())
            }
            gl::DeleteProgram(self.id)
        }
    }
}