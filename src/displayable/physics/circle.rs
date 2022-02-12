use crate::vec2::Vec2;
use super::{Physics, Intersect, rect::Rect, Shape};

use sdl2::rect::Point;
use sdl2::pixels::Color;

use super::super::Displayable;

#[derive(Debug, Clone, Copy)]
pub struct Circle
{
    centre: Vec2,
    radius: f64,
    velocity: Vec2,
    mass: f64,
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
impl Physics for Circle
{
    #[inline]
    fn position(&self) -> Vec2
    {
        self.centre
    }
    #[inline]
    fn velocity(&self) -> Vec2
    {
        self.velocity
    }
    #[inline]
    fn mass(&self) -> f64
    {
        self.mass
    }

    #[inline]
    fn integrate(&mut self)
    {
        self.centre += self.velocity
    }
}

impl Intersect for Circle
{
    fn intersect(&self, other: &Shape) -> bool
    {
        match other {
            Shape::Circle(circle) => {
                (self.position()-circle.position()).len_squared() < (self.radius+circle.radius).powf(2.0_f64)
            }
            Shape::Rect(rect) => {
                
                let rotCirc = Circle::new(self.centre().rotate(&Vec2::new(0.0, 0.0), -rect.rotation()), self.radius());
                
                let newBotLeft = rect.bottomLeft().rotate(&Vec2::new(0.0, 0.0), -rect.rotation());
                let newTopRight = rect.topRight().rotate(&Vec2::new(0.0, 0.0), -rect.rotation());
                let rotRect = Rect::new(newBotLeft, newTopRight, 0.0);
                let points = rotRect.points();

                #[derive(Debug)]
                enum XDirection {
                    Left,
                    Middle,
                    Right,
                }
                
                #[derive(Debug)]
                enum YDirection {
                    Above,
                    Middle,
                    Below,
                }

                let xDirection = match rotCirc.position().x() {
                    x if x < points[0].x() => XDirection::Left,
                    x if x > points[3].x() => XDirection::Right,
                    _ => XDirection::Middle,
                };

                let yDirection = match rotCirc.position().y() {
                    y if y > points[1].y() => YDirection::Above,
                    y if y < points[0].y() => YDirection::Below,
                    _ => YDirection::Middle,
                };

                let closestPoint = match (xDirection, yDirection) {
                    (XDirection::Left, YDirection::Above) => points[1],
                    (XDirection::Left, YDirection::Middle) => Vec2::new(points[0].x(), rotCirc.position().y()),
                    (XDirection::Left, YDirection::Below) => points[0],
                    (XDirection::Middle, YDirection::Above) =>  Vec2::new(rotCirc.position().x(), points[1].y()),
                    (XDirection::Middle, YDirection::Middle) => return true,
                    (XDirection::Middle, YDirection::Below) =>  Vec2::new(rotCirc.position().x(), points[0].y()),
                    (XDirection::Right, YDirection::Above) => points[2],
                    (XDirection::Right, YDirection::Middle) => Vec2::new(points[3].x(), rotCirc.position().y()),
                    (XDirection::Right, YDirection::Below) => points[3],
                };
                if (closestPoint-rotCirc.position()).len_squared() < rotCirc.radius().powf(2.0_f64) {
                    true
                }
                else {
                    false
                }
            },
        }
    }
}

impl Circle
{
    #[inline]
    pub fn new(centre: Vec2, radius: f64) -> Circle
    {
        Circle{centre, radius, velocity: Vec2::new(2.0, -2.0), mass: 1.0}
    }

    #[inline]
    pub fn centre(&self) -> Vec2
    {
        self.centre
    }

    #[inline]
    pub fn radius(&self) -> f64
    {
        self.radius
    }
}
