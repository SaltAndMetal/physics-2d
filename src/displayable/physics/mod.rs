use super::Displayable;
use crate::vec2::Vec2;

pub mod rect;
pub mod circle;

pub trait Physics: Displayable {
    fn position(&self) -> Vec2;
    fn velocity(&self) -> Vec2;
    fn mass(&self) -> f64;

    fn integrate(&mut self);
}

pub trait Intersect<T: Physics>: Physics {
    fn intersect(&self, other: &T) -> bool;
}
