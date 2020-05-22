use anyhow::Result;

use std::rc::Rc;
use std::include_str;

pub use simple_gl::graphics::*;

pub struct Graphics {
    pub program: Program
}

impl Graphics {
    pub fn new() -> Result<Graphics> {

        let vert_shader = VertexShader::from_source(
            include_str!("../../resources/shaders/cube/cube.vert")
        )?;
        let geom_shader = GeometryShader::from_source(
            include_str!("../../resources/shaders/cube/cube.geom")
        )?;
        let frag_shader = FragmentShader::from_source(
            include_str!("../../resources/shaders/cube/cube.frag")
        )?;

        let mut program = Program::from_shaders(vec![
            Rc::new(vert_shader),
            Rc::new(geom_shader),
            Rc::new(frag_shader),
        ])?;
        program.set_used();

        program.set_uniform("margin", 0.5f32);
        
        Ok(Graphics {
            program
        })
    }
}