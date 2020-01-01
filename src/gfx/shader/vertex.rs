use gl::types::*;
use anyhow::Result;
use super::{ShaderExt, Shader};

pub type VertexShader = Shader<Vertex>;

pub struct Vertex {}

impl ShaderExt for Vertex {
    fn new() -> Vertex {
        Vertex{}
    }

    fn ty() -> GLenum {
        gl::VERTEX_SHADER
    }

    fn name() -> &'static str {
        "vertex"
    }
}

impl VertexShader {
    pub fn from_source(source: &str) -> Result<VertexShader> {
        let mut shader = VertexShader::new();
        shader.set_source(source);
        shader.compile()?;
        Ok(shader)
    }
}