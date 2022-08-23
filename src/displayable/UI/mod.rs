use super::Displayable;
use sdl2::rect::Point;
use crate::Mode;
use crate::Shape;
pub mod button;

pub trait UI: Displayable
{
    fn in_bounds(&self, click: Point) -> bool;

    fn click_down(&mut self, state: &mut Mode, objects: &Vec<Shape>);
    fn click_up(&mut self, objects: &Vec<Shape>);
}
