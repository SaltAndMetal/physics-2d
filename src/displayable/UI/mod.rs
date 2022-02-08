use super::Displayable;
use sdl2::rect::Point;

pub mod button;

pub trait UI: Displayable
{
    fn in_bounds(&self, click: Point) -> bool;

    fn click_down(&mut self);
    fn click_up(&mut self);

    fn clicked(&self) -> bool;
}
