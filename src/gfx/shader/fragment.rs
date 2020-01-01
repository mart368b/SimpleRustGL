use gl::types::*;
use super::{ShaderExt, Shader};
use anyhow::Result;

pub type FragmentShader = Shader<Fragment>;

pub struct Fragment {}

impl ShaderExt for Fragment {
    fn new() -> Fragment {
        Fragment{}
    }

    fn ty() -> GLenum {
        gl::FRAGMENT_SHADER
    }

    fn name() -> &'static str {
        "fragment"
    }
}

impl FragmentShader {
    pub fn from_source(source: &str) -> Result<FragmentShader> {
        let mut shader = FragmentShader::new();
        shader.set_source(source);
        shader.compile()?;
        Ok(shader)
    }
}