use crate::gfx::buffer::*;
use crate::{Vector2, clamp};

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
    xcount: usize,
    ycount: usize
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
            &verticies
        );

        //CREATE VAO
        let mut vao = Vao::new(Format::LinesAdj);
        vao.bind_vbo(
            0,
            &mut vbo
        );

        // CREATE EBO
        let ebo = EboBuffer::new(
            &index
        );

        World {
            vbo,
            vao,
            ebo,
            vert_count: index.len() as u32,
            xcount,
            ycount
        }
    }

    pub fn add(&mut self, pos: Vector2, amount: f32) {
        let mut vbo = self.vbo.write();
        let map = &mut *vbo;
        let pos0 = Vector2::new(pos.index(0).floor(), pos.index(1).floor());
        let pos1 = pos0 + Vector2::new(1., 0.);
        let pos2 = pos0 + Vector2::new(0., 1.);
        let pos3 = pos0 + Vector2::new(1., 1.);
        
        let max_dist: f32 = 45f32.sin() * 2.;

        let ratio0 = 1. - (pos.metric_distance(&pos0) / max_dist);
        let ratio1 = 1. - (pos.metric_distance(&pos1) / max_dist);
        let ratio2 = 1. - (pos.metric_distance(&pos2) / max_dist);
        let ratio3 = 1. - (pos.metric_distance(&pos3) / max_dist);

        let index0 = (pos0.index(0) + pos0.index(1) * (self.xcount + 1) as f32) as usize;
        let index1 = index0 + 1;
        let index2 = index0 + 1 + self.xcount;
        let index3 = index0 + 2 + self.xcount;

        map[index0].amount = clamp(0., 1., map[index0].amount + amount * ratio0);
        map[index1].amount = clamp(0., 1., map[index1].amount + amount * ratio1);
        map[index2].amount = clamp(0., 1., map[index2].amount + amount * ratio2);
        map[index3].amount = clamp(0., 1., map[index3].amount + amount * ratio3);
    }

    pub fn draw(&mut self) {
        self.vao.bind();
        self.ebo.bind();
        self.vao.draw_elements(self.vert_count, Primitive::UInt, 0);
    }
}