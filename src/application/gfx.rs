use anyhow::Result;

use std::rc::Rc;

use super::shaders::*;
pub use crate::gfx::graphics::*;

pub struct Graphics {
    pub program: Program
}

impl Graphics {
    pub fn new() -> Result<Graphics> {
        let vert_shader = create_cube_vertex_shader()?;
        let geom_shader = create_cube_geometry_shader()?;
        let frag_shader = create_cube_frag_shader()?;

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