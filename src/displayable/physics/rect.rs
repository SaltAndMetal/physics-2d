use crate::vec2::Vec2;
use super::Physics;
use super::super::Displayable;

use sdl2::pixels::Color;
use sdl2::rect::Point;

extern crate bresenham;
use bresenham::Bresenham;

#[derive(Debug)]
pub struct Rect
{
    points: [Vec2; 4],
    rotation: f64,
    velocity: Vec2,
    mass: f64,
}

impl Displayable for Rect
{
    fn display(&self) -> Vec<(Point, Color)>
    {
        let pointA = self.bottomLeft();

        let pointB = pointA + Vec2::new(self.rotation.sin()*self.topRight().x(), self.rotation.cos()*self.topRight().y());

        let secondAngle = self.rotation()+std::f64::consts::FRAC_PI_2;
        let pointC = pointB + Vec2::new(secondAngle.sin()*self.topRight().x(), secondAngle.cos()*self.topRight().y());

        let thirdAngle = self.rotation()+std::f64::consts::PI;
        let pointD = pointC + Vec2::new(thirdAngle.sin()*self.topRight().x(), thirdAngle.cos()*self.topRight().y());

        let extr = |a: Vec2| {let a = Point::from(a); (a.x() as isize, a.y() as isize)};
        let revExtr = |a: (isize, isize)| {(Point::new(a.0 as i32, a.1 as i32), self.color())};

        let points: Vec<(Point, Color)> = Bresenham::new(extr(pointA), extr(pointB))
            .chain(Bresenham::new(extr(pointB), extr(pointC)))
            .chain(Bresenham::new(extr(pointC), extr(pointD)))
            .chain(Bresenham::new(extr(pointD), extr(pointA)))
            .map(|a| revExtr(a))
            .collect();
            
        points
    }
}

impl Physics for Rect
{
    fn position(&self) -> Vec2
    {
        (self.points[0]+self.points[1])/2.0
    }
    fn velocity(&self) -> Vec2
    {
        self.velocity
    }
    fn mass(&self) -> f64
    {
        self.mass
    }

    fn integrate(&mut self)
    {
        self.points.map(|x| x + self.velocity);
    }
}

impl Rect
{
    pub fn new(bottomLeft: Vec2, topRight: Vec2, rotation: f64) -> Rect
    {
        let pointA = bottomLeft;
        let pointB = bottomLeft + Vec2::new(rotation.sin()*topRight.x(), rotation.cos()*topRight.y());
        let pointC = topRight;
        let angle = rotation + std::f64::consts::PI;
        let pointD = pointC + Vec2::new(angle.sin()*topRight.x(), angle.cos()*topRight.y());

        let points = [pointA, pointB, pointC, pointD];
        Rect{points, rotation, velocity: Vec2::new(0.0, 0.0), mass: 1.0}
    }
        
    pub fn bottomLeft(&self) -> Vec2
    {
        self.points[0]
    }

    pub fn topRight(&self) -> Vec2
    {
        self.points[2]
    }

    pub fn rotation(&self) -> f64 
    {
        self.rotation
    }
}


