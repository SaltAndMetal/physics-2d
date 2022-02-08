use crate::vec2::Vec2;
use super::Physics;
use super::super::Displayable;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(Debug)]
pub struct Rect
{
    bottomLeft: Vec2,
    topRight: Vec2,
    rotation: f64,
}

impl Displayable for Rect
{
    fn display(&self, canvas: &mut Canvas<Window>)
    {
        let pointA = self.bottomLeft();
        let pointB = pointA + Vec2::new(self.rotation.sin()*self.topRight().x(), self.rotation.cos()*self.topRight().y());
        let secondAngle = self.rotation()+std::f64::consts::FRAC_PI_2;
        let pointC = pointB + Vec2::new(secondAngle.sin()*self.topRight.x(), secondAngle.cos()*self.topRight.y());
        let thirdAngle = self.rotation()+std::f64::consts::PI;
        let pointD = pointC + Vec2::new(thirdAngle.sin()*self.topRight.x(), thirdAngle.cos()*self.topRight.y());
        let points: Vec<Point> = [pointA, pointB, pointC, pointD, pointA].iter().map(|x| Point::from(*x)).collect();
        canvas.set_draw_color(self.color());
        canvas.draw_lines(&points[..]).unwrap();
    }
}

impl Physics for Rect {}

impl Rect
{
    pub fn new(bottomLeft: Vec2, topRight: Vec2, rotation: f64) -> Rect
    {
        Rect{bottomLeft, topRight, rotation}
    }
        
    pub fn bottomLeft(&self) -> Vec2
    {
        self.bottomLeft
    }

    pub fn topRight(&self) -> Vec2
    {
        self.topRight
    }

    pub fn rotation(&self) -> f64 
    {
        self.rotation
    }
}


