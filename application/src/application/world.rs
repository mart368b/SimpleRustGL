use wrapper::buffer::*;
use crate::{Vector2, clamp};

use anyhow::Result;

pub struct World {
    pub vbo: VboBuffer<[f32; 2], DynamicBuffer>,
    pub avbo: VboBuffer<f32, DynamicBuffer>,
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
    ) -> Result<World> {
        // GENERATE VERTICIES
        let mut verticies = Vec::new();
        let mut amounts = Vec::new();
        for ix in 0..(ycount + 1) {
            let y = ix as f32 / ycount as f32;
            for iy in 0..(xcount + 1) {
                let x = iy as f32 / xcount as f32;
                let mut amount = 0f32;
                if ix == iy {
                    amount = 1.0;
                }
                verticies.push([(x*2. - 1.) * width, (1. - y*2.) * height]);
                amounts.push(amount);
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

        let mut avbo = VboBuffer::new(
            &amounts
        );

        //CREATE VAO
        let mut vao = Vao::new(Format::LinesAdj, 2);
        vao.bind_vbo(
            0,
            &mut vbo
        )?;
        vao.bind_vbo(
            1,
            &mut avbo
        )?;

        // CREATE EBO
        let ebo = EboBuffer::new(
            &index
        );

        Ok(World {
            vbo,
            avbo,
            vao,
            ebo,
            vert_count: index.len() as u32,
            xcount,
            ycount
        })
    }

    pub fn add(&mut self, pos: Vector2, amount: f32) {
        let mut avbo = self.avbo.write();
        let map = &mut *avbo;
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

        map[index0] = clamp(0., 1., map[index0] + amount * ratio0);
        map[index1] = clamp(0., 1., map[index1] + amount * ratio1);
        map[index2] = clamp(0., 1., map[index2] + amount * ratio2);
        map[index3] = clamp(0., 1., map[index3] + amount * ratio3);
    }

    pub fn draw(&mut self) {
        self.ebo.bind();
        self.vao.bind();
        self.vao.draw_elements(self.vert_count, Primitive::UInt, 0);
    }
}