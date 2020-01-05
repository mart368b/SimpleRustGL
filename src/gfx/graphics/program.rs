use gl::types::*;
use super::*;
use anyhow::{Result, anyhow};
use crate::gfx::error::get_program_error;
use std::rc::Rc;
use crate::gfx::get_value;
use std::collections::HashMap;
use std::borrow::Borrow;

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

    pub fn set_uniform<'a, K: 'a, Q>(&mut self, name: &str, val: K)
    where
        K: Borrow<Q>,
        Q: ?Sized,
        for<'b>
        &'b Q: Into<Uniform<'b>>
    {
        if let Some(loc) = self.get_uniform_loc(name) {
            let t = val.borrow();
            let uniform = t.into();
            unsafe {
                match uniform {
                    Uniform::Int(i) => gl::Uniform1i(loc, i),
                    Uniform::Int2(i, i2) => gl::Uniform2i(loc, i, i2),
                    Uniform::Int3(i, i2, i3) => gl::Uniform3i(loc, i, i2, i3),
                    Uniform::Int4(i, i2, i3, i4) => gl::Uniform4i(loc, i, i2, i3, i4),
                    Uniform::UInt(u) => gl::Uniform1ui(loc, u),
                    Uniform::UInt2(u, u2) => gl::Uniform2ui(loc, u, u2),
                    Uniform::UInt3(u, u2, u3) => gl::Uniform3ui(loc, u, u2, u3),
                    Uniform::UInt4(u, u2, u3, u4) => gl::Uniform4ui(loc, u, u2, u3, u4),
                    Uniform::Float(f) => gl::Uniform1f(loc, f),
                    Uniform::Float2(f, f2) => gl::Uniform2f(loc, f, f2),
                    Uniform::Float3(f, f2, f3) => gl::Uniform3f(loc, f, f2, f3),
                    Uniform::Float4(f, f2, f3, f4) => gl::Uniform4f(loc, f, f2, f3, f4),
                    
                    Uniform::IntVec(v) => gl::Uniform1iv(loc, v.len() as GLsizei, v.as_ptr() as *const GLint),
                    Uniform::IntVec2(v) => gl::Uniform2iv(loc, v.len() as GLsizei, v.as_ptr() as *const GLint),
                    Uniform::IntVec3(v) => gl::Uniform3iv(loc, v.len() as GLsizei, v.as_ptr() as *const GLint),
                    Uniform::IntVec4(v) => gl::Uniform4iv(loc, v.len() as GLsizei, v.as_ptr() as *const GLint),
                    Uniform::UIntVec(v) => gl::Uniform1uiv(loc, v.len() as GLsizei, v.as_ptr() as *const GLuint),
                    Uniform::UIntVec2(v) => gl::Uniform2uiv(loc, v.len() as GLsizei, v.as_ptr() as *const GLuint),
                    Uniform::UIntVec3(v) => gl::Uniform3uiv(loc, v.len() as GLsizei, v.as_ptr() as *const GLuint),
                    Uniform::UIntVec4(v) => gl::Uniform4uiv(loc, v.len() as GLsizei, v.as_ptr() as *const GLuint),
                    Uniform::FloatVec(v) => gl::Uniform1fv(loc, v.len() as GLsizei, v.as_ptr() as *const GLfloat),
                    Uniform::FloatVec2(v) => gl::Uniform2fv(loc, v.len() as GLsizei, v.as_ptr() as *const GLfloat),
                    Uniform::FloatVec3(v) => gl::Uniform3fv(loc, v.len() as GLsizei, v.as_ptr() as *const GLfloat),
                    Uniform::FloatVec4(v) => gl::Uniform4fv(loc, v.len() as GLsizei, v.as_ptr() as *const GLfloat),

                    Uniform::Matrix2(m) => gl::UniformMatrix2fv(loc, m.len() as GLsizei, false as GLboolean, m.as_ptr() as *const GLfloat),
                    Uniform::Matrix3(m) => gl::UniformMatrix3fv(loc, m.len() as GLsizei, false as GLboolean, m.as_ptr() as *const GLfloat),
                    Uniform::Matrix4(m) => gl::UniformMatrix4fv(loc, m.len() as GLsizei, false as GLboolean, m.as_ptr() as *const GLfloat),
                    Uniform::Matrix2x3(m) => gl::UniformMatrix2x3fv(loc, m.len() as GLsizei, true as GLboolean, m.as_ptr() as *const GLfloat),
                    Uniform::Matrix3x2(m) => gl::UniformMatrix2x3fv(loc, m.len() as GLsizei, false as GLboolean, m.as_ptr() as *const GLfloat),
                    Uniform::Matrix2x4(m) => gl::UniformMatrix2x4fv(loc, m.len() as GLsizei, true as GLboolean, m.as_ptr() as *const GLfloat),
                    Uniform::Matrix4x2(m) => gl::UniformMatrix2x4fv(loc, m.len() as GLsizei, false as GLboolean, m.as_ptr() as *const GLfloat),
                    Uniform::Matrix3x4(m) => gl::UniformMatrix3x4fv(loc, m.len() as GLsizei, true as GLboolean, m.as_ptr() as *const GLfloat),
                    Uniform::Matrix4x3(m) => gl::UniformMatrix3x4fv(loc, m.len() as GLsizei, false as GLboolean, m.as_ptr() as *const GLfloat),
                }
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