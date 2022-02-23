use super::Displayable;
use sdl2::rect::Point;
use crate::State;
pub mod button;

pub trait UI: Displayable
{
    fn in_bounds(&self, click: Point) -> bool;

    fn click_down(&mut self, state: &mut State);
    fn click_up(&mut self);
}
