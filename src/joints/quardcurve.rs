use crate::{Canvas, Point};

use nalgebra::geometry::Point2;

// Length of each segement
const RESOLUTION: f64 = 10f64;

pub struct QuardCurve {
    pub p0: Point2<f64>,
    pub p1: Point2<f64>,
    pub p2: Point2<f64>,
    pub cach: Vec<Point>,
    has_changed: bool
}

impl QuardCurve{
    pub fn new(p0: Point2<f64>, p1: Point2<f64>, p2: Point2<f64>) -> QuardCurve{
        QuardCurve{
            p0, p1, p2,
            cach: Vec::new(),
            has_changed: true
        }
    }

    pub fn draw(&mut self, csv: &mut Canvas) -> Result<(), String>{
        if self.has_changed{
            self.update_cach();
            self.has_changed = false;
        }

        csv.draw_lines(self)?;
        Ok(())
    }

    fn update_cach(&mut self){
        self.cach.clear();
        let length = self.len();
        let step = RESOLUTION / (length - RESOLUTION);

        self.add_to_cach(self.p(0.));

        if step > 0. {
            let mut t = (RESOLUTION / 2.) / length;
            while t < 1f64 {
                self.add_to_cach(self.p(t));
                t += step;
            }
        }

        self.add_to_cach(self.p(1f64));
    }

    fn add_to_cach(&mut self, p: Point2<f64>){
        self.cach.push(
            Point::new(p.x.floor() as i32, p.y.floor() as i32)
        );
    }

    /// https://stackoverflow.com/questions/6711707/draw-a-quadratic-b%C3%A9zier-curve-through-three-given-points
    pub fn p(&self, t: f64) -> Point2<f64>{
        assert!(t >= 0.);
        assert!(t <= 1.);
        let p0: Point2<f64> = self.p0*t*t;
        let p1: Point2<f64> = self.p1*2.*t*(1.-t);
        let p2: Point2<f64> = self.p2*((1.-t)*(1.-t));
        Point2::new(
            p0.x + p1.x + p2.x,
            p0.y + p1.y + p2.y,
        )
    }

    /// https://www.derivative-calculator.net/
    fn derived(&self, t: f64) -> f64{
        self.p0.x*2f64*t + self.p1.x*(1f64-t) - self.p1.x*t + self.p2.x*2f64*(1f64-t)
    }

    /// https://en.wikipedia.org/wiki/Arc_length
    pub fn len(&self) -> f64 {
        (1f64 + self.derived(0f64).powi(2)).sqrt()
    }
}


impl<'a> From<&'a mut QuardCurve> for &'a[Point]{
    fn from(curve: &'a mut QuardCurve) -> &'a [Point] {
        curve.cach.as_slice()
    }
}