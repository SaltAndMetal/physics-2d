use crate::vec2::Vec2;
use super::Physics;

use sdl2::rect::Point;
use sdl2::pixels::Color;

use super::super::Displayable;

#[derive(Debug)]
pub struct Circle
{
    centre: Vec2,
    radius: f64,
}

impl Displayable for Circle
{
    fn display(&self) -> Vec<(Point, Color)>
    {
        let r2 = self.radius.powf(2_f64);
        
        //Computes how far away the centre of a pixel would be from the idealised point on the circle
        //The point given is relative to the centre of the circle
        let RE = |point: Vec2|
        {
            let d2 = point.len_squared();
            (d2-r2).abs()
        };

        let mut points = Vec::new();
        
        //Start at the top of the circle, draws one eighth
        let mut x = 0;
        let mut y = self.radius as i32;
        while x < y {
            points.push(Vec2::new(x.into(), y.into()));
            let pointA = Vec2::new((x+1).into(), y.into());
            let pointB = Vec2::new((x+1).into(), (y-1).into());
            x += 1;
            if RE(pointB) < RE(pointA) {
                y -= 1;
            }
        }
        let points: Vec<(Point, Color)> = points.iter()
            .map(|a| vec![*a, Vec2::new(-a.x(), a.y()), Vec2::new(a.x(), -a.y()), Vec2::new(-a.x(), -a.y()), Vec2::new(a.y(), a.x()), Vec2::new(-a.y(), a.x()), Vec2::new(a.y(), -a.x()), Vec2::new(-a.y(), -a.x())]).flatten()
            .map(|a| a + self.centre)
            .map(|x| (Point::from(x), self.color())).collect();
        points
    }
}
impl Physics for Circle {}

impl Circle
{
    pub fn new(centre: Vec2, radius: f64) -> Circle
    {
        Circle{centre, radius}
    }

    pub fn centre(&self) -> Vec2
    {
        self.centre
    }

    pub fn radius(&self) -> f64
    {
        self.radius
    }
}
