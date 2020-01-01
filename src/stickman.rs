use crate::{Canvas, Point};

pub struct StickMan{
    pub points: [Point; 10]
}

impl StickMan{
    pub fn new() -> StickMan{
        let left_hand = Point::new(-10, 0);
        let right_hand = Point::new(40, 0);
        let neck = Point::new(10, 0);
        let hip = Point::new(10, 10);
        let left_foot = Point::new(0, 50);
        let right_foot = Point::new(20, 50);

        StickMan{
            points: [
                left_hand, neck,
                right_hand, neck,
                neck, hip,
                left_foot, hip,
                right_foot, hip
            ]
        }
    }

    pub fn set_left_hand(&mut self, p: Point){
        self.points[0] = p;
    }

    pub fn set_right_hand(&mut self, p: Point){
        self.points[2] = p;
    }

    pub fn set_neck(&mut self, p: Point){
        self.points[1] = p;
        self.points[3] = p;
    }

    pub fn set_hip(&mut self, p: Point){
        self.points[5] = p;
        self.points[7] = p;
        self.points[9] = p;
    }

    pub fn set_left_foot(&mut self, p: Point){
        self.points[6] = p;
    }

    pub fn set_right_foot(&mut self, p: Point){
        self.points[8] = p;
    }

    pub fn draw(&self, cvs:&mut Canvas) -> Result<(), String>{
        cvs.draw_lines(self)?;
        Ok(())
    }
}

impl<'a> From<&'a StickMan> for &'a[Point]{
    fn from(man: &'a StickMan) -> &'a [Point] {
        &man.points
    }
} 