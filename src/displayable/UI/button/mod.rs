use sdl2::pixels::Color;
use sdl2::rect::Point;
pub mod pauseButton;
pub mod moveButton;
pub mod rectButton;
pub mod circleButton;
use crate::{ManipMode, Mode};
use crate::Shape;

use crate::Vec2;
use super::Displayable;
use super::UI;

use bmp::Pixel;
use bmp::px;

pub trait Button: UI {
    fn topLeft(&self) -> Point;
    fn bottomRight(&self) -> Point;
    fn texture(&self) -> &bmp::Image;
    fn mutTexture(&mut self) -> &mut bmp::Image;
    fn clicked(&self) -> bool;
    fn clickDown(&mut self, state: &mut Mode, objects: &Vec<Shape>);
}

fn display(button: &impl Button) -> Vec<(Point, Color)> {
    let size = button.bottomRight()-button.topLeft();
    let scale = Vec2::new(size.x() as f64/button.texture().get_width() as f64, size.y() as f64/button.texture().get_height() as f64);
    
    let mut points = Vec::new();
     for (x, y) in button.texture().coordinates() {
        let px = button.texture().get_pixel(x, y);
        
        points.push((Point::from(button.topLeft())+Point::new((x as f64*scale.x()) as i32, (y as f64*scale.y() as f64) as i32), Color::RGB(px.r, px.g, px.b)));
     }
    points
}

fn in_bounds(button: &impl Button, click: Point) -> bool
{
    click.x()>button.topLeft().x() as i32 &&click.y()>button.topLeft().y() as i32 &&click.x()<button.bottomRight().x() as i32 &&click.y()<button.bottomRight().y() as i32 
}

fn invert(texture: &mut bmp::Image)
{
    for (x, y) in texture.coordinates() {
        let px = texture.get_pixel(x, y);
        texture.set_pixel(x, y, px![255-px.b, 255-px.g, 255-px.b])
    }
}
