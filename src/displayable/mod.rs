pub mod physics;
pub mod UI;

use sdl2::render::Canvas;
use sdl2::video::Window;

use sdl2::pixels::Color;
use sdl2::rect::Point;

pub trait Displayable 
{
    fn display(&self) -> Vec<(Point, Color)>;
    fn color(&self) -> Color
    {
        Color::RGB(255, 255, 255)
    }
}
