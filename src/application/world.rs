use crate::gfx::buffer::*;
use crate::Vector2;

#[derive(Debug)]
#[repr(C)]
pub struct WorldVertex {
    pub x: f32,
    pub y: f32,
    pub amount: f32
}

impl WorldVertex {
    pub fn as_vector(&self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }
}

impl BufferData for WorldVertex {
    fn prototype() -> Vec<(Primitive, u32)> {
        vec![
            (Primitive::Float, 2),
            (Primitive::Float, 1)
        ]
    }
}

pub struct World {
    pub vbo: VboBuffer<WorldVertex, DynamicBuffer>,
    pub vao: Vao,
    pub ebo: EboBuffer<u32, StaticBuffer>,
    vert_count: u32,
}

impl World {
    pub fn new(
        width: f32,
        height: f32,
        xcount: usize,
        ycount: usize
    ) -> World {
        // GENERATE VERTICIES
        let mut verticies = Vec::new();
        for ix in 0..(ycount + 1) {
            let y = ix as f32 / ycount as f32;
            for iy in 0..(xcount + 1) {
                let x = iy as f32 / xcount as f32;
                let mut amount = 0.;
                if ix == iy {
                    amount = 1.0;
                }
                verticies.push(WorldVertex{x: (x*2. - 1.) * width, y: (1. - y*2.) * height, amount});
            }
        }

        // GENERATE INDEX INTO VERTICIES
        let xwidth: usize = xcount + 1;
        let mut index: Vec<u32> = Vec::new();
        for iy in 0..ycount {
            for ix in 0..xcount {
                index.push((ix + iy * xwidth) as u32);
                index.push(((ix + 1) + iy * xwidth) as u32);
                index.push(((ix + 1) + (iy + 1) * xwidth) as u32);
                index.push((ix + (iy + 1) * xwidth) as u32);
            }
        }

        // CREATE VBO
        let mut vbo = VboBuffer::new(
            &[
                WorldVertex{x: 0.0, y: 0.0, amount: 0.0},
                WorldVertex{x: 1.0, y: 0.0, amount: 1.0},
                WorldVertex{x: 1.0, y: 1.0, amount: 0.0},
                WorldVertex{x: 0.0, y: 1.0, amount: 0.0},
            ]
        );

        //CREATE VAO
        let mut vao = Vao::new(Format::LinesAdj);
        vao.bind_vbo(
            0,
            &mut vbo
        );

        // CREATE EBO
        let mut ebo = EboBuffer::new(
            &[0, 1, 2, 3]
        );

        World {
            vbo,
            vao,
            ebo,
            vert_count: index.len() as u32
        }
    }

    pub fn draw(&mut self) {
        self.vao.bind();
        self.ebo.bind();
        self.vao.draw_elements(self.vert_count, Primitive::UInt, 0);
    }
}