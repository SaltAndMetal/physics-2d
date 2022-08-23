use super::{Displayable, Point, Color};
use crate::vec2::Vec2;

pub mod rect;
pub mod circle;

#[derive(Debug, Clone)]
pub enum Shape
{
    Rect(rect::Rect),
    Circle(circle::Circle),
}

impl Displayable for Shape
{
    fn display(&self) -> Vec<(Point, Color)>
    {
        match self {
            Self::Rect(rect) => rect.display(),
            Self::Circle(circle) => circle.display(),
        }
    }
}
impl Physics for Shape {
    fn position(&self) -> Vec2 
    {
        match self {
            Self::Rect(rect) => rect.position(),
            Self::Circle(circle) => circle.position(),
        }
    }
    fn velocity(&self) -> Vec2 
    {
        match self {
            Self::Rect(rect) => rect.velocity(),
            Self::Circle(circle) => circle.velocity(),
        }
    }
    fn angular_velocity(&self) -> f64
    {
        match self {
            Self::Rect(rect) => rect.angular_velocity(),
            Self::Circle(circle) => circle.angular_velocity(),
        }
    }
    fn mass(&self) -> f64 
    {
        match self {
            Self::Rect(rect) => rect.mass(),
            Self::Circle(circle) => circle.mass(),
        }
    }
    fn impulse(&mut self, impulse: &Vec2)
    {
        match self {
            Self::Rect(rect) => rect.impulse(impulse),
            Self::Circle(circle) => circle.impulse(impulse),
        }
    }
    fn angular_impulse(&mut self, impulse: f64)
    {
        match self {
            Self::Rect(rect) => rect.angular_impulse(impulse),
            Self::Circle(circle) => circle.angular_impulse(impulse),
        }
    }
    fn integrate(&mut self, gravity: &Vec2)
    {
        match self {
            Self::Rect(rect) => rect.integrate(gravity),
            Self::Circle(circle) => circle.integrate(gravity),
        }
    }
    fn pointIn(&self, point: &Vec2) -> bool
    {
        match self {
            Self::Rect(rect) => rect.pointIn(point),
            Self::Circle(circle) => circle.pointIn(point),
        }
    }
    fn translateTo(&mut self, point: &Vec2)
    {
        match self {
            Self::Rect(rect) => rect.translateTo(point),
            Self::Circle(circle) => circle.translateTo(point),
        }
    }

    fn resize(&mut self, point: &Vec2, newPoint: &Vec2, archive: &Self)
    {

        match (self, archive) {
            (Self::Rect(rect), Self::Rect(rArchive)) => rect.resize(point, newPoint, rArchive),
            (Self::Circle(circle), Self::Circle(cArchive)) => circle.resize(point, newPoint, cArchive),
            _ => unreachable!(),
        }
    }

    fn rotate(&mut self, point: &Vec2, newPoint: &Vec2, archive: &Self)
    {
        match (self, archive) {
            (Self::Rect(rect), Self::Rect(rArchive)) => rect.rotate(point, newPoint, rArchive),
            (Self::Circle(circle), Self::Circle(cArchive)) => circle.rotate(point, newPoint, cArchive),
            _ => unreachable!(),
        }
    }
    fn bounce(&mut self, other: &Shape)
    {
        match self {
            Self::Rect(rect) => rect.bounce(other),
            Self::Circle(circle) => circle.bounce(other),
        }
    }

}
impl Intersect for Shape {
    fn intersect(&self, other: &Shape) -> bool 
    {
        match self {
            Self::Rect(rect) => rect.intersect(other),
            Self::Circle(circle) => circle.intersect(other),
        }
    }
}

pub trait Physics: Displayable {
    fn position(&self) -> Vec2;
    fn translateTo(&mut self, point: &Vec2);
    fn velocity(&self) -> Vec2;
    fn angular_velocity(&self) -> f64;
    fn mass(&self) -> f64;
    fn impulse(&mut self, impulse: &Vec2);
    fn angular_impulse(&mut self, impulse: f64);
    fn pointIn(&self, point: &Vec2) -> bool;
    fn integrate(&mut self, gravity: &Vec2);
    fn resize(&mut self, point: &Vec2, newPoint: &Vec2, archive: &Self);
    fn rotate(&mut self, point: &Vec2, newPoint: &Vec2, archive: &Self);
    fn bounce(&mut self, other: &Shape);
}

pub trait Intersect: Physics {
    fn intersect(&self, other: &Shape) -> bool;
}

//Unomptimised, and O(n) in number of objects
pub fn detectIntersections(objects: &Vec<Shape>) -> Vec<(usize, usize)> {
    let mut intersections = Vec::new();
    for (i, o) in objects.iter().enumerate() {
        for (i1, o1) in objects.iter().enumerate() {
            if i1 > i {
                if o.intersect(o1) {
                    intersections.push((i, i1));
                }
            }
        }
    }
    intersections
}

