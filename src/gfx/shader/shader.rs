pub use gl::types::*;
pub use anyhow::{Result, anyhow};
pub use crate::gfx::error::get_shader_error;
use crate::gfx::get_value;
use std::ffi::CString;

pub trait ShaderExt {
    fn new() -> Self;
    fn ty() -> GLenum;
    fn name() -> &'static str;
}

pub struct Shader<T> 
    where
        T: ShaderExt
{
    id: GLuint,
    ty: T
}

impl<T> Shader<T> 
where
    T: ShaderExt
{
    pub fn new() -> Shader<T> {
        Shader {
            id: unsafe{ gl::CreateShader(T::ty()) },
            ty: T::new()
        }
    }

    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn set_source<Q>(&mut self, source: Q)
        where
            Q: AsRef<str> 
    {
        self.set_sources(vec![source]);
    }

    pub fn set_sources<Q , S> (&mut self, sources: Q)
        where 
            Q: AsRef<Vec<S>>,
            S: AsRef<str>
    {

        let sources = sources.as_ref();
        let sources_raw: Vec<CString> = sources
            .iter()
            .map(|s| {
                CString::new(s.as_ref()).unwrap()
            })
            .collect();

        let sources_ptr: Vec<*const GLchar> = sources_raw
            .iter()
            .map(|raw| raw.as_ptr() as *const GLchar)
            .collect();
        
        unsafe {
            gl::ShaderSource(
                self.id,
                sources.len() as GLint,
                sources_ptr.as_ptr(),
                std::ptr::null()
            )
        }
    }

    pub fn compile(&mut self) -> Result<()> {
        unsafe {
            gl::CompileShader(self.id);
        }

        let mut success: GLint = get_value(1, |success|unsafe {
            gl::GetShaderiv(self.id, gl::COMPILE_STATUS, success);
        });

        match success {
            1 => Ok(()),
            _ => {
                Err(anyhow!(
                    "Failed to compile {} shader: {}", 
                    T::name(), 
                    get_shader_error(self.id)
                ))
            }
        }
    }
}

impl<T> Drop for Shader<T>
where
    T: ShaderExt
{
    fn drop(&mut self) {
        unsafe {gl::DeleteShader(self.id)}
    }
}

