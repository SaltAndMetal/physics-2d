use super::{Displayable, Point, Color};
use crate::vec2::Vec2;

pub mod rect;
pub mod circle;

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
    fn position(&self) -> Vec2 {
        match self {
            Self::Rect(rect) => rect.position(),
            Self::Circle(circle) => circle.position(),
        }
    }
    fn velocity(&self) -> Vec2 {
        match self {
            Self::Rect(rect) => rect.velocity(),
            Self::Circle(circle) => circle.velocity(),
        }
    }
    fn mass(&self) -> f64 {
        match self {
            Self::Rect(rect) => rect.mass(),
            Self::Circle(circle) => circle.mass(),
        }
    }
    fn integrate(&mut self){
        match self {
            Self::Rect(rect) => rect.integrate(),
            Self::Circle(circle) => circle.integrate(),
        }
    }
}
impl Intersect for Shape {
    fn intersect(&self, other: &Shape) -> bool {
        match self {
            Self::Rect(rect) => rect.intersect(other),
            Self::Circle(circle) => circle.intersect(other),
        }
    }
}

pub trait Physics: Displayable {
    fn position(&self) -> Vec2;
    fn velocity(&self) -> Vec2;
    fn mass(&self) -> f64;

    fn integrate(&mut self);
}

pub trait Intersect: Physics {
    fn intersect(&self, other: &Shape) -> bool;
}

