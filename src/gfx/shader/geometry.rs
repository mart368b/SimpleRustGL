use gl::types::*;
use super::{ShaderExt, Shader};
use anyhow::Result;

pub type GeometryShader = Shader<Geometry>;

pub struct Geometry {}

impl ShaderExt for Geometry {
    fn new() -> Geometry {
        Geometry{}
    }

    fn ty() -> GLenum {
        gl::GEOMETRY_SHADER
    }

    fn name() -> &'static str {
        "geometry"
    }
}

impl GeometryShader {
    pub fn from_source(source: &str) -> Result<GeometryShader> {
        let mut shader = GeometryShader::new();
        shader.set_source(source);
        shader.compile()?;
        Ok(shader)
    }
}