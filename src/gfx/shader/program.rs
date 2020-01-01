use gl::types::*;
use std::any::Any;
use super::*;
use anyhow::{Result, anyhow};
use crate::gfx::error::get_program_error;
use std::rc::Rc;

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
    shaders: Vec<Rc<dyn ProgramAttachment>>
}

impl Program {
    pub fn new() -> Program {
        Program {
            id: unsafe{gl::CreateProgram()},
            shaders: Vec::new()
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

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut success);
        }

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